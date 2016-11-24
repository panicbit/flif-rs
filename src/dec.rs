use std::io::{self, Read};
use {image, Image, Options, Movement};
use podio::ReadPodExt;
use varint::{self, ReadVarintExt};
use format::{Format, Encoding};
use metadata::{self, Metadata};

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
    metadata_type: metadata::Format,
}

pub fn decode<R: Read>(r: &mut R,
                       callback: (),
                       first_callback_quality: i32,
                       mut options: Options)
                       -> Result<metadata::Format, Error> {
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
    if &buf == b"!<ar" {
        r.read_exact(&mut buf)?;
        if &buf != b"ch>\n" {
            return Err(Error::InvalidMagic);
        }
        panic!("AR FLIFF unimplemented");
    }

    if &buf != b"FLIF" {
        return Err(Error::InvalidMagic);
    }

    let format = Format::from_reader(r)?;
    options.method = Some(format.encoding);

    let color_depth_ident = r.read_u8()?;
    if ![b'0', b'1', b'2'].contains(&color_depth_ident) {
        return Err(Error::UnsupportedColorDepth);
    }

    let width = r.read_varint()? + 1;
    let height = r.read_varint()? + 1;

    let num_frames = if format.is_animated {
        r.read_varint()? + 2
    } else {
        1
    };

    let metadata = Metadata::all_from_reader(r)?;

    println!("Animated: {} ({} frame(s))", format.is_animated, num_frames);
    println!("{:?}", format.encoding);
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
        UnsupportedColorDepth {
            description("Unsupported color depth")
        }
        Format(err: ::format::Error) {
            from()
        }
        Metadata(err: metadata::Error) {
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
