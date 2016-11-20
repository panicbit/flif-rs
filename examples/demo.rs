extern crate flif;

use std::fs::File;
use std::env;
use flif::Options;

fn main() {
    let path = env::args().nth(1).expect("usage: demo <image.flif>");
    let mut file = File::open(path).unwrap();

    let first_callback_quality = 100;
    let options = Options::default();
    flif::dec::decode(&mut file, (), first_callback_quality, options).unwrap();
}
