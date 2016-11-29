extern crate flif;
extern crate env_logger;

use std::fs::File;
use std::env;

fn main() {
    env_logger::init().unwrap();
    let path = env::args().nth(1).expect("usage: demo <image.flif>");
    let mut file = File::open(path).unwrap();

    let info = flif::dec::decode(&mut file).unwrap();

    println!("{:#?}", info);
}
