use std::io::{self, Read};
use {image, Image, Options, Movement, Encoding, MetadataOptions};
use podio::ReadPodExt;
use varint::{self, ReadVarintExt};

#[derive(Debug)]
struct FlifInfo {
    width: u32,
    height: u32,
    channels: u8,
    bit_depth: u8,
    num_images: u8,
}

struct DecodeResult {
    images: Vec<u8>,
    partial_images: Vec<u8>,
    metadata_options: image::MetadataOptions,
}

pub fn decode<R: Read>(r: &mut R,
                       callback: (),
                       first_callback_quality: i32,
                       mut options: Options)
                       -> Result<MetadataOptions, Error> {
    let scale = options.scale;
    let rw = options.resize_width;
    let rh = options.resize_height;

    let fit = options.fit;
    let mut just_identify = false;
    let mut just_metadata = false;

    match scale {
        -1 => just_identify = true,
        -2 => just_metadata = true,
        1 | 2 | 4 | 8 | 16 | 32 | 64 | 128 => (),
        _ => return Err(Error::InvalidScaleDownFactor(scale)),
    }

    // Read the magic
    let mut buf: [u8; 4] = [0; 4];
    r.read_exact(&mut buf)?;

    // Try to read optional AR archive
    if &buf[..4] == b"!<ar" {
        r.read_exact(&mut buf)?;
        if &buf[..4] != b"ch>\n" {
            return Err(Error::InvalidMagic);
        }
        panic!("AR FLIFF unimplemented");
    }

    if &buf != b"FLIF" {
        return Err(Error::InvalidMagic);
    }

    let format_and_colorspace = r.read_u8()?;
    let (movement, encoding) = match format_and_colorspace >> 4 {
        0x3 => (Movement::Static, Encoding::NonInterlaced),
        0x4 => (Movement::Static, Encoding::Interlaced),
        0x5 => (Movement::Animated, Encoding::NonInterlaced),
        0x6 => (Movement::Animated, Encoding::Interlaced),
        _ => return Err(Error::InvalidFormat),
    };
    options.method = Some(encoding);

    if encoding == Encoding::NonInterlaced {
        // TODO: validate options
    }

    let num_planes = format_and_colorspace & 0x0F;
    if ![1, 3, 4].contains(&num_planes) {
        return Err(Error::UnsupportedColorChannel);
    }

    let color_depth_ident = r.read_u8()?;
    if ![b'0', b'1', b'2'].contains(&color_depth_ident) {
        return Err(Error::UnsupportedColorDepth);
    }

    let width = r.read_varint()? + 1;
    let height = r.read_varint()? + 1;

    let num_frames = if let Movement::Animated = movement {
        r.read_varint()? + 2
    } else {
        1
    };

    println!("{:?} ({} frame(s))", movement, num_frames);
    println!("{:?}", encoding);
    println!("{:?}x{:?}", width, height);

    unimplemented!()
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
        InvalidFormat {
            description("Invalid (or unknown) FLIF format byte")
        }
        UnsupportedColorChannel {
            description("Unsupported color channels")
        }
        UnsupportedColorDepth {
            description("Unsupported color depth")
        }
        Io(err: io::Error) {
            from()
        }
        Varint(err: varint::Error) {
            from()
        }
    }
}
