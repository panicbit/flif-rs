#[derive(Debug)]
pub struct Metadata {
    /// name of the chunk (every chunk is assumed to be unique, 4 ascii letters plus terminating 0)
    pub name: [u8; 4],
    /// length of the chunk contents
    // length: isize,
    pub contents: Vec<u8>,
}

#[derive(Debug)]
pub struct MetadataOptions {
    icc: bool,
    exif: bool,
    xmp: bool,
}

#[derive(Debug,Copy,Clone)]
pub struct Image;

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum Movement {
    Static,
    Animated,
}
