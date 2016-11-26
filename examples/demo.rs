extern crate flif;

use std::fs::File;
use std::env;

fn main() {
    let path = env::args().nth(1).expect("usage: demo <image.flif>");
    let mut file = File::open(path).unwrap();

    let info = flif::dec::decode(&mut file).unwrap();

    println!("{:#?}", info);
}
