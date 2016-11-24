use std::io::{self, Read};
use super::{RacInput,RacConfig};

struct UniformSymbolDecoder<C: RacConfig, R: Read> {
    rac: RacInput<C, R>
}

impl<C: RacConfig, R: Read> UniformSymbolDecoder<C, R> {
    fn new(rac: RacInput<C, R>) -> Self {
        UniformSymbolDecoder {
            rac: rac,
        }
    }

    fn read_int(&mut self, min: isize, mut max: isize) -> Result<isize, Error> {
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

    fn read_int_bits(&mut self, bits: isize) -> Result<isize, Error> {
        self.read_int(0, (1<<bits)-1)
    }
}

#[derive(Debug,Copy,Clone)]
enum SymbolChanceBitType {
    BitZero,
    BitSign,
    BitExp,
    BitMant,
}

const EXP_CHANCES: [u16; 17] = [1000, 1200, 1500, 1750, 2000, 2300, 2800, 2400, 2300,
                                2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048];

const MANT_CHANCES: [u16; 18] = [1900, 1850, 1800, 1750, 1650, 1600, 1600, 2048, 2048,
                                 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048];

const ZERO_CHANCE: u16 = 1000;
const SIGN_CHANCE: u16 = 2048;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Rac(err: super::Error) {
            from()
        }
    }
}
