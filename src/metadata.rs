use std::io::{self, Read};
use podio::ReadPodExt;
use varint::{self, ReadVarintExt};
use flate2::read::DeflateDecoder;

/// The maximum size a metadata chunk is allowed to have.
/// This limit exists to avoid DoS caused by allocating too much memory.
pub const REASONABLE_METADATA_LENGTH: u64 = 5 * 1024 * 1024; // 5 MB

#[derive(Debug)]
pub struct Metadata {
    /// name of the chunk (every chunk is assumed to be unique, 4 ascii letters plus terminating 0)
    pub name: [u8; 4],
    pub contents: Vec<u8>,
}

impl Metadata {
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

        if ![b"iCCP", b"eXif", b"eXmp"].contains(&&name) {
            if name[0] > b'Z' {
                return Err(Error::UnknownChunk(name));
            } else {
                return Err(Error::UnknownCriticalChunk(name));
            }
        }

        let length = r.read_varint().map_err(Error::InvalidLength)?;
        if length > REASONABLE_METADATA_LENGTH {
            return Err(Error::UnreasonableLength);
        }

        // Decompress metadata using deflate
        let mut contents = Vec::new();
        let mut deflate = DeflateDecoder::new(r.take(length));
        deflate.read_to_end(&mut contents)?;

        Ok(Some(Metadata {
            name: name,
            contents: contents,
        }))
    }
}

#[derive(Debug,Copy,Clone)]
pub enum Type {
    Icc,
    Exif,
    Xmp,
}

impl Type {
    fn from_bytes(name: [u8; 4]) -> Result<Self, Error> {
        match &name {
            b"iCCP" => Ok(Type::Icc),
            b"eXif" => Ok(Type::Exif),
            b"eXmp" => Ok(Type::Xmp),
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
