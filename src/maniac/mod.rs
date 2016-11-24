use std::mem::size_of;
use std::marker::PhantomData;
use std::io::{self, Read};
use std::ops::{ShlAssign, ShrAssign, Shr, BitOrAssign, Sub, SubAssign};
use podio::ReadPodExt;

mod symbol;

pub trait RacConfig {
    type Data: From<u8> + PartialOrd + ShlAssign<u32> + ShrAssign<u32> + Shr<u32,Output=Self::Data> + BitOrAssign<Self::Data> + Copy + Sub<Self::Data,Output=Self::Data> + SubAssign<Self::Data>;
    fn max_range_bits() -> Self::Data;
    fn min_range_bits() -> Self::Data;
    fn min_range() -> Self::Data;
    fn base_range() -> Self::Data;
    fn chance_12bit_chance(b12: isize, range: Self::Data) -> Self::Data;
}

#[derive(Debug)]
pub struct RacConfig24;

impl RacConfig for RacConfig24 {
    type Data = u32;
    fn max_range_bits() -> Self::Data {
        24
    }
    fn min_range_bits() -> Self::Data {
        16
    }
    fn min_range() -> Self::Data {
        1 << Self::min_range_bits()
    }
    fn base_range() -> Self::Data {
        1 << Self::max_range_bits()
    }
    fn chance_12bit_chance(b12: isize, range: Self::Data) -> Self::Data {
        assert!(b12 > 0);
        assert!((b12 >> 12) == 0);
        let b12 = b12 as Self::Data;

        if size_of::<Self::Data>() > 4 {
            (range * b12 + 0x800) >> 12
        }
        else {
            ((((range & 0xFFF) * b12 + 0x800) >> 12) + ((range >> 12) * b12))
        }
    }
}

#[derive(Debug)]
pub struct RacInput<C: RacConfig, R: Read> {
    _config: PhantomData<C>,
    r: R,
    range: C::Data,
    low: C::Data,

}

impl<C: RacConfig, R: Read> RacInput<C, R> {
    pub fn new(r: R) -> Result<Self, Error> {
        let mut this = RacInput {
            _config: PhantomData,
            r: r,
            range: C::base_range(),
            low: 0.into(),
        };

        let mut range = C::base_range();
        while range > 1.into() {
            this.low <<= 8;
            this.low |= this.read_catch_eof()?;
            range >>= 8;
        }

        Ok(this)
    }

    pub fn read_catch_eof(&mut self) -> Result<C::Data, Error> {
        let data = &mut [0];
        Ok(match self.r.read_u8()? {
            0 => 0xFF, // Garbage
            _ => data[0],
        }.into())
    }

    pub fn input(&mut self) -> Result<(), Error> {
        for _ in 0..2 {
            if self.range <= C::min_range() {
                self.low <<= 8;
                self.range <<= 8;
                self.low |= self.read_catch_eof()?;
            }
        }
        Ok(())
    }

    pub fn get(&mut self, chance: C::Data) -> Result<bool, Error> {
        assert!(chance > 0.into());
        assert!(chance < self.range.into());

        if self.low >= self.range - chance {
            self.low -= self.range - chance;
            self.range = chance;
            self.input()?;
            Ok(true)
        }
        else {
            self.range -= chance;
            self.input()?;
            Ok(false)
        }
    }

    pub fn read_12bit_chance(&mut self, b12: u16) -> Result<bool, Error> {
        let range = self.range;
        self.get(C::chance_12bit_chance(b12 as isize, range))
    }

    pub fn read_bit(&mut self) -> Result<bool, Error> {
        let chance = self.range >> 1;
        self.get(chance)
    }
}

pub type RacInput24<R> = RacInput<RacConfig24, R>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            from()
        }
    }
}

