use std::io::{self, Read};
use std::fmt::{self, Debug};
use podio::ReadPodExt;
use varint::{self, ReadVarintExt};
use flate2::read::DeflateDecoder;

/// The maximum size a metadata chunk is allowed to have.
/// This limit exists to avoid DoS caused by allocating too much memory.
pub const REASONABLE_METADATA_LENGTH: u64 = 5 * 1024 * 1024; // 5 MB

pub struct Metadata {
    /// name of the chunk (every chunk is assumed to be unique, 4 ascii letters plus terminating 0)
    pub format: Format,
    pub data: Vec<u8>,
}

impl Metadata {
    pub fn all_from_reader<R: Read>(r: &mut R) -> Result<Vec<Metadata>, Error> {
        // TODO: Maybe create iterator for this?
        let mut result = Vec::new();
        loop {
            let metadata = Self::from_reader(r)?;
            if let Some(metadata) = metadata {
                result.push(metadata);
            } else {
                return Ok(result);
            }
        }
    }

    pub fn from_reader<R: Read>(r: &mut R) -> Result<Option<Metadata>, Error> {
        let mut name = [0; 4];

        // Check the first byte
        name[0] = r.read_u8()?;
        if name[0] == 0 || name[0] > 127 {
            return Ok(None);
        }
        if name[0] < 32 {
            return Err(Error::FutureFormat);
        }

        // Check the remaining bytes
        r.read(&mut name[1..])?;

        let format = Format::from_bytes(name)?;

        let length = r.read_varint().map_err(Error::InvalidLength)?;
        if length > REASONABLE_METADATA_LENGTH {
            return Err(Error::UnreasonableLength);
        }

        // Decompress metadata using deflate
        let mut data = Vec::new();
        let mut deflate = DeflateDecoder::new(r.take(length));
        deflate.read_to_end(&mut data)?;

        Ok(Some(Metadata {
            format: format,
            data: data,
        }))
    }
}

impl Debug for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.format)
    }
}

#[derive(Debug,Copy,Clone)]
pub enum Format {
    Icc,
    Exif,
    Xmp,
}

impl Format {
    fn from_bytes(name: [u8; 4]) -> Result<Self, Error> {
        match &name {
            b"iCCP" => Ok(Format::Icc),
            b"eXif" => Ok(Format::Exif),
            b"eXmp" => Ok(Format::Xmp),
            _ =>
                if name[0] > b'Z' {
                    Err(Error::UnknownChunk(name))
                } else {
                    Err(Error::UnknownCriticalChunk(name))
                }
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        UnknownChunk(name: [u8; 4]) {
            description("Unknown chunk")
            display("Unknown metadata chunk `{:?}`", name)
        }
        UnknownCriticalChunk(name: [u8; 4]) {
            description("Unknown critical chunk")
            display("Unknown critical metadata chunk `{:?}`", name)
        }
        InvalidLength(err: varint::Error) {
            description("Invalid metadata length")
            cause(err)
        }
        UnreasonableLength {
            description("Metadata too big (>5MB)")
        }
        FutureFormat {
            description("Not a FLIF16 image, but a more recent FLIF file")
        }
        Io(err: io::Error) {
            from()
        }
    }
}
