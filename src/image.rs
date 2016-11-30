
#[derive(Debug,Copy,Clone)]
pub struct Image {
    delay: Option<u16>,
}

impl Image {
    pub fn new(delay: Option<u16>) -> Self {
        Image {
            delay: delay
        }
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum Movement {
    Static,
    Animated,
}
