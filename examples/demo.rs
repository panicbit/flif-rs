extern crate flif;

use std::fs::File;
use std::env;
use flif::dec::DecoderOptions;

fn main() {
    let path = env::args().nth(1).expect("usage: demo <image.flif>");
    let mut file = File::open(path).unwrap();

    let options = DecoderOptions::default();
    flif::dec::decode(&mut file, options).unwrap();
}
