
#[derive(Debug,Copy,Clone)]
pub struct Image {
    width: u64,
    height: u64,
    delay: Option<u16>,
}

impl Image {
    pub fn new(width: u64, height: u64, delay: Option<u16>) -> Self {
        Image {
            width: width,
            height: height,
            delay: delay,
        }
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum Movement {
    Static,
    Animated,
}
