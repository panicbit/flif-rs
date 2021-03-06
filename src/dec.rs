use std::io::{self, Read};
use podio::ReadPodExt;
use varint::{self, ReadVarintExt};
use format::{Format, Encoding};
use metadata::{self, Metadata};
use maniac::{rac, symbol, UniformSymbolDecoder};
use image::Image;

pub fn decode<R: Read>(mut r: R) -> Result<ImageDecoderBuilder<R>, Error> {
    // Read the magic
    let mut buf: [u8; 4] = [0; 4];
    r.read_exact(&mut buf)?;

    if &buf != b"FLIF" {
        return Err(Error::InvalidMagic);
    }

    let format = Format::from_reader(&mut r)?;

    let bpp_ident = r.read_u8()?;
    if ![b'0', b'1', b'2'].contains(&bpp_ident) {
        return Err(Error::UnsupportedColorDepth);
    }

    let width = r.read_varint()? + 1;
    let height = r.read_varint()? + 1;

    let n_frames = if format.is_animated {
        r.read_varint()? + 2
    } else {
        1
    };

    let metadata = Metadata::all_from_reader(&mut r)?;

    let rac = rac::Input24::new(r)?;
    let mut meta_decoder = symbol::UniformSymbolDecoder::new(rac);

    let mut highest_bpp = 0;
    for _ in 0..format.num_planes {
        let bpp = match bpp_ident {
            b'1' => 8,
            b'2' => 16,
            b'0' => meta_decoder.read_int(1, 16)? as u8,
            _ => unreachable!(),
        };
        if bpp > highest_bpp {
            highest_bpp = bpp;
        }
    };

    let alpha_zero = if format.num_planes > 3 {
        meta_decoder.read_bool()?
    } else {
        false
    };

    let n_loops = if format.is_animated {
        Some(meta_decoder.read_int(0, 100)? as u8)
    } else {
        None
    };

    Ok(ImageDecoderBuilder {
        meta_decoder: meta_decoder,
        info: Info {
            width: width,
            height: height,
            highest_bpp: highest_bpp,
            n_frames: n_frames,
            encoding: format.encoding,
            alpha_zero: alpha_zero,
            metadata: metadata,
            n_channels: format.num_planes,
            n_loops: n_loops,
        }
    })
}

pub struct ImageDecoderBuilder<R> {
    meta_decoder: UniformSymbolDecoder<rac::Config24, R>,
    info: Info,
}

impl<R> ImageDecoderBuilder<R> {
    pub fn info(&self) -> &Info {
        &self.info
    }
}

pub fn decode_image<R: Read>(builder: ImageDecoderBuilder<R>, options: DecoderOptions) -> Result<(), Error> {
    let info = builder.info;
    let mut meta_decoder = builder.meta_decoder;
    let width = info.width;
    let height = info.height;
    let resize_dimensions = options.resize_dimensions;
    let (mut resize_w, mut resize_h) = resize_dimensions.unwrap_or((width, height));

    let (mut target_w, mut target_h) = (resize_w, resize_h);
    if options.fit {
        if resize_w == 0 || resize_h == 0 {
            return Err(Error::InvalidResizeDimensions)
        }

        // Use larger decode dimensions to ensure good chroma
        resize_w = resize_w * 2 - 1;
        resize_h = resize_h * 2 - 1;

        // Don't upscale
        if target_w > width {
            target_w = width;
        }
        if target_h > height {
            target_h = height;
        }
    }

    // Find a fitting downscale factor if resize dimensions are set
    let mut scale: u64 = options.scale_down.into();
    if resize_dimensions.is_some() {
        debug!("Figuring out scale factor for resize dimensions");

        if scale > 1 {
            return Err(Error::ResizeParameterConflict);
        }

        while (resize_w > 0 && (((width - 1) / scale) + 1) > resize_w) ||
              (resize_h > 0 && (((height - 1) / scale) + 1) > resize_h) {
            scale *= 2;
            trace!("Increasing scale to {}", scale);
        }
    }

    debug!("scale = {}", scale);

    if scale != 1 && info.encoding == Encoding::NonInterlaced {
        return Err(Error::ScaleNonInterlaced);
    }

    let scale_shift = (scale as f64).log2() as u8;
    debug!("scale_shift = {}", scale_shift);

    if scale_shift > 0 {
        let new_width = ((width-1)/scale)+1;
        let new_height = ((height-1)/scale)+1;
        debug!("Decoding downscaled image at scale 1:{} ({}x{} -> {}x{})", scale, width, height, new_width, new_height);
    }

    // Estimate buffer size
    let n_channels = info.n_channels as u64;
    let bytes_per_pixel_per_channel = if info.highest_bpp <= 8 { 1 } else { 2 };
    let additional_bytes = if n_channels > 1 { 2 } else { 0 };
    let bytes_per_pixel: u64 = bytes_per_pixel_per_channel * (n_channels + additional_bytes);
    let n_frames = info.n_frames;
    let estimated_buffer_size: u64 = (((width-1)/scale)+1) * (((height-1)/scale)+1) * n_frames * n_channels * bytes_per_pixel;
    debug!("estimated_buffer_size = {}", estimated_buffer_size);

    if estimated_buffer_size > options.max_image_buffer_size {
        return Err(Error::BufferSizeExceedsLimit);
    }

    if n_frames > options.max_frames {
        return Err(Error::FrameLimitExceeded);
    }

    let mut images = Vec::new();
    for frame_i in 0..n_frames {
        let delay = if info.n_frames > 1 {
            trace!("Decoding delay for frame {}", frame_i);
            Some(meta_decoder.read_int(0, 60_000)? as u16)
        } else {
            None
        };
        debug!("delay of frame {}: {:?}", frame_i, delay);

        let image = Image::new(width, height, delay);
        images.push(image);
    }

    let mut cutoff: u8 = 2;
    let mut alpha = u32::max_value() / 19;

    if meta_decoder.read_bool()? {
        cutoff = meta_decoder.read_int(1, 128)? as u8;
        alpha = u32::max_value() / meta_decoder.read_int(2, 128)? as u32;
        if meta_decoder.read_bool()? {
            return Err(Error::Unimplemented("non-default bitchance"));
        }
    }

    debug!("cutoff = {}", cutoff);
    debug!("alpha = {}", alpha);

    unimplemented!()
}

#[derive(Debug)]
pub struct Info {
    width: u64,
    height: u64,
    highest_bpp: u8,
    n_frames: u64,
    encoding: Encoding,
    alpha_zero: bool,
    metadata: Vec<Metadata>,
    n_channels: u8,
    n_loops: Option<u8>,
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidScaleDownFactor(scale: i32) {
            description("Invalid scale down factor")
            display("Invalid scale down factor `{}`", scale)
        }
        InvalidMagic {
            description("Invalid file header (probably not a FLIF file)")
        }
        UnsupportedColorDepth {
            description("Unsupported color depth")
        }
        InvalidResizeDimensions {
            description("Invalid resize dimensions")
        }
        ResizeParameterConflict {
            description("Resize dimensions and resize factor are mutually exclusive")
        }
        ScaleNonInterlaced {
            description("Cannot decode non-interlaced FLIF file at lower scale")
        }
        BufferSizeExceedsLimit {
            description("The required buffer size exceeds the limit")
        }
        FrameLimitExceeded {
            description("Maximum number of frames exceeded")
        }
        Unimplemented(err: &'static str) {
            from()
        }
        Format(err: ::format::Error) {
            from()
        }
        Metadata(err: metadata::Error) {
            from()
        }
        Rac(err: rac::Error) {
            from()
        }
        Symbol(err: symbol::Error) {
            from()
        }
        Io(err: io::Error) {
            from()
        }
        Varint(err: varint::Error) {
            from()
        }
    }
}

#[derive(Debug,Copy,Clone)]
pub struct DecoderOptions {
    pub scale_down: ScaleDownFactor,
    pub resize_dimensions: Option<(u64, u64)>,
    pub fit: bool,
    /// Maximum image buffer size to attempt to decode.
    /// Default: 5GB
    /// This is one frame of 1000 megapixels 8-bit RGB (it's 5 bytes per pixel
    /// because YCoCg uses 2 bytes each for Co/Cg)
    /// (or 1000 frames of 1 megapixel)
    pub max_image_buffer_size: u64,
    /// Maximum number of frames to decode.
    /// Default: 50_000
    pub max_frames: u64,
}

impl Default for DecoderOptions {
    fn default() -> Self {
        DecoderOptions {
            scale_down: ScaleDownFactor::By1,
            resize_dimensions: None,
            fit: false,
            max_image_buffer_size: 5 * 1024 * 1024 * 1024,
            max_frames: 50_000,
        }
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum ScaleDownFactor {
    By1,
    By2,
    By4,
    By8,
    By16,
    By32,
    By64,
    By128,
}

impl From<ScaleDownFactor> for u64 {
    fn from(scale: ScaleDownFactor) -> Self {
        match scale {
            ScaleDownFactor::By1 => 1,
            ScaleDownFactor::By2 => 2,
            ScaleDownFactor::By4 => 4,
            ScaleDownFactor::By8 => 8,
            ScaleDownFactor::By16 => 16,
            ScaleDownFactor::By32 => 32,
            ScaleDownFactor::By64 => 64,
            ScaleDownFactor::By128 => 128,
        }
    }
}
