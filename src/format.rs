use std::io::{self, Read};
use podio::ReadPodExt;

#[derive(Debug)]
pub struct Format {
    pub is_animated: bool,
    pub encoding: Encoding,
    pub num_planes: u8,
}

impl Format {
    pub fn from_reader<R: Read>(r: &mut R) -> Result<Self, Error> {
        let format_byte = r.read_u8()?;
        Self::from_u8(format_byte)
    }

    pub fn from_u8(format: u8) -> Result<Self, Error> {
        let (is_animated, encoding) = match format >> 4 {
            0x3 => (false, Encoding::NonInterlaced),
            0x4 => (false, Encoding::Interlaced),
            0x5 => (true, Encoding::NonInterlaced),
            0x6 => (true, Encoding::Interlaced),
            _ => return Err(Error::InvalidFormat),
        };

        if encoding == Encoding::NonInterlaced {
            // TODO: validate options
        }

        let num_planes = format & 0x0F;
        if ![1, 3, 4].contains(&num_planes) {
            return Err(Error::UnsupportedColorChannel);
        }

        Ok(Format {
            is_animated: is_animated,
            encoding: encoding,
            num_planes: num_planes,
        })
    }
}


#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum Encoding {
    Interlaced,
    NonInterlaced,
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidFormat {
            description("Invalid (or unknown) FLIF format byte")
        }
        UnsupportedColorChannel {
            description("Unsupported color channels")
        }
        Io(err: io::Error) {
            from()
        }
    }
}
