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

    Ok(Info {
        width: width,
        height: height,
        highest_bpp: highest_bpp,
        n_frames: n_frames,
        encoding: format.encoding,
        alpha_zero: alpha_zero,
        metadata: metadata,
    })
}

fn decode_image<R: Read>(r: &mut R, info: Info, options: DecoderOptions) -> Result<(), Error> {

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
}

#[derive(Debug,Copy,Clone)]
pub enum ScaleDownFactor {
    By1,
    By2,
    By4,
    By8,
    By16,
    By32,
    By64,
    By128
}

impl Default for DecoderOptions {
    fn default() -> Self {
        DecoderOptions {
            scale_down: ScaleDownFactor::By1,
        }
    }
}
