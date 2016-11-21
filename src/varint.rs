use std::io::{self, Read};
use podio::ReadPodExt;

/// Extend `Read` trait with Varint-specific methods
pub trait ReadVarintExt {
    /// Read a variable length integer that fints into an u64
    fn read_varint(&mut self) -> Result<u64, Error>;
}

impl<R: Read> ReadVarintExt for R {
    fn read_varint(&mut self) -> Result<u64, Error> {
        let mut result: u64 = 0;

        for _bytes_read in 0..10 {
            let byte = self.read_u8()? as u64;
            let last = byte < 0x80;

            if last {
                result = result.checked_add(byte).ok_or(Error::WouldOverflow)?;
                return Ok(result)
            }

            let number = byte & 0x7F;

            result = result.checked_add(number).ok_or(Error::WouldOverflow)?;
            result = result.checked_mul(1 << 7).ok_or(Error::WouldOverflow)?;
        }

        Err(Error::InvalidNumber)
    }
}

quick_error! {
    /// Varint read errors
    #[derive(Debug)]
    pub enum Error {
        InvalidNumber {
            description("Invalid number")
        }
        WouldOverflow {
            description("Variable int would overflow u64")
        }
        Io(err: io::Error) {
            from()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! read_varint {
        ($($x:expr),+) => (
            (&mut &[$($x),+][..]).read_varint()
        )
    }

    macro_rules! assert_err {
        ($err:pat, $x:expr) => {
            if let Err($err) = $x {
                // all is well
            } else {
                panic!("expected {:?}, got {:?}", stringify!($err), $x);
            }
        }
    }

    #[test]
    fn easy() {
        assert_eq!(read_varint!(0x82, 0x2b).unwrap(), 299);
        assert_eq!(read_varint!(0x86, 0x1f).unwrap(), 799);
        assert_eq!(read_varint!(0x84, 0x57).unwrap(), 599);
    }

    #[test]
    fn edge() {
        assert_eq!(
            read_varint!(0x81, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F).unwrap(),
            ::std::u64::MAX);
    }

    #[test]
    fn invalid_number() {
        assert_err!(Error::WouldOverflow,
            read_varint!(0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F));
        assert_err!(Error::WouldOverflow,
            read_varint!(0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF));

        assert_err!(Error::InvalidNumber,
            read_varint!(0x80, 0x80, 0x80, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF));
        assert_err!(Error::InvalidNumber,
            read_varint!(0x80, 0x80, 0x80, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF));
    }
}
