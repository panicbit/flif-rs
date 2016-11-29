use std::io::{self, Read};
use podio::ReadPodExt;
use varint::{self, ReadVarintExt};
use format::{Format, Encoding};
use metadata::{self, Metadata};
use maniac::{rac, symbol};

pub fn decode<R: Read>(r: &mut R) -> Result<Info, Error> {
    // Read the magic
    let mut buf: [u8; 4] = [0; 4];
    r.read_exact(&mut buf)?;

    // Try to read optional AR archive
    if &buf == b"!<ar" {
        r.read_exact(&mut buf)?;
        if &buf != b"ch>\n" {
            return Err(Error::InvalidMagic);
        }
        return Err(Error::ArchivedFlifNotSupported)
    }

    if &buf != b"FLIF" {
        return Err(Error::InvalidMagic);
    }

    let format = Format::from_reader(r)?;

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

    let metadata = Metadata::all_from_reader(r)?;

    let rac = rac::Input24::new(r)?;
    let mut decoder = symbol::UniformSymbolDecoder::new(rac);

    let mut highest_bpp = 0;
    for _ in 0..format.num_planes {
        let bpp = match bpp_ident {
            b'1' => 8,
            b'2' => 16,
            b'0' => decoder.read_int(1, 16)? as u8,
            _ => unreachable!(),
        };
        if bpp > highest_bpp {
            highest_bpp = bpp;
        }
    };

    let alpha_zero = if format.num_planes > 3 {
        decoder.read_int(0, 1)? != 0
    } else {
        false
    };

    let n_loops = if format.is_animated {
        Some(decoder.read_int(0, 100)? as u8)
    } else {
        None
    };

    Ok(Info {
        width: width,
        height: height,
        highest_bpp: highest_bpp,
        n_frames: n_frames,
        encoding: format.encoding,
        alpha_zero: alpha_zero,
        metadata: metadata,
        n_channels: format.num_planes,
        n_loops: n_loops,
    })
}

fn decode_image<R: Read>(r: &mut R, info: Info, options: DecoderOptions) -> Result<(), Error> {
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
        if scale > 1 {
            return Err(Error::ResizeParameterConflict);
        }

        while (resize_w > 0 && (((width - 1) / scale) + 1) > resize_w) ||
              (resize_h > 0 && (((height - 1) / scale) + 1) > resize_h) {
            scale *= 2;
        }
    }

    if scale != 1 && info.encoding == Encoding::NonInterlaced {
        return Err(Error::ScaleNonInterlaced);
    }

    let scale_shift = (scale as f64).log2() as u8;

    if scale_shift > 0 {
       // Log
    }

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
        ArchivedFlifNotSupported {
            description("FLIF files in AR archives are not supported")
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
}

impl Default for DecoderOptions {
    fn default() -> Self {
        DecoderOptions {
            scale_down: ScaleDownFactor::By1,
            resize_dimensions: None,
            fit: false,
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
