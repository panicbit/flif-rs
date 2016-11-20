use std::io::{self, Read};
use podio::ReadPodExt;

pub fn read<R: Read>(r: &mut R) -> Result<u64, Error> {
    let mut result: u64 = 0;
    for _bytes_read in 0..10 {
        let byte = r.read_u8()? as u64;
        let last = byte < 0x80;
        let number = byte & 0x7F;

        result = result.checked_add(number).ok_or(Error::WouldOverflow)?;

        if last {
            return Ok(result)
        }

        result = result.checked_shl(7).ok_or(Error::WouldOverflow)?;
    }
    Err(Error::InvalidNumber)
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidNumber {}
        WouldOverflow {}
        Io(err: io::Error) { from() }
    }
}

#[cfg(test)]
mod test {
    use super::read;
    #[test]
    fn varint() {
        assert_eq!(read(&mut &[0x82, 0x2b][..]).unwrap(), 299);
        assert_eq!(read(&mut &[0x86, 0x1f][..]).unwrap(), 799);
        assert_eq!(read(&mut &[0x84, 0x57][..]).unwrap(), 599);
    }
}
