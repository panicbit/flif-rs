use image::ColorVal;
use image::color_ranges::StaticColorRanges;

#[derive(Debug,Copy,Clone)]
pub struct Image {
    width: u64,
    height: u64,
    delay: Option<u16>,
    scale_shift: u8,
    num_planes: u64,
    min_val: ColorVal,
    max_val: ColorVal,
    depth: u8,
}

impl Image {
    pub fn new(
        width: u64, height: u64, min_val: ColorVal, max_val: ColorVal, delay: Option<u16>,
        scale_shift: u8, num_planes: u64
    ) -> Self {
        Image {
            width: width,
            height: height,
            delay: delay,
            scale_shift: scale_shift,
            num_planes: num_planes,
            min_val: min_val,
            max_val: max_val,
            depth: if max_val < 256 { 8 } else { 16 },
        }
    }
}

impl Image {
    pub fn min(&self) -> ColorVal { self.min_val }
    pub fn max(&self) -> ColorVal { self.max_val }
    pub fn num_planes(&self) -> u64 { self.num_planes }
    pub fn get_ranges(&self) -> StaticColorRanges {
        (0..self.num_planes() as usize)
            .map(|_| (self.min(), self.max()))
            .collect()
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum Movement {
    Static,
    Animated,
}
