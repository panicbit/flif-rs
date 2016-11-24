
#[derive(Debug,Copy,Clone)]
pub struct Image;

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum Movement {
    Static,
    Animated,
}
