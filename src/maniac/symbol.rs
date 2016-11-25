use std::io::Read;
use super::rac;

pub struct UniformSymbolDecoder<C: rac::Config, R: Read> {
    rac: rac::Input<C, R>
}

impl<C: rac::Config, R: Read> UniformSymbolDecoder<C, R> {
    pub fn new(rac: rac::Input<C, R>) -> Self {
        UniformSymbolDecoder {
            rac: rac,
        }
    }

    pub fn read_int(&mut self, min: isize, mut max: isize) -> Result<isize, Error> {
        assert!(max >= min);
        if min != 0 {
            max -= min;
        }
        if max == 0 {
            return Ok(min);
        }

        let med = max / 2;
        let bit = self.rac.read_bit()?;

        // TODO: Remove recursion
        if bit {
            self.read_int(min+med+1, min+max)
        }
        else {
            self.read_int(min, min+med)
        }
    }

    pub fn read_int_bits(&mut self, bits: isize) -> Result<isize, Error> {
        self.read_int(0, (1<<bits)-1)
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Rac(err: rac::Error) {
            from()
        }
    }
}
