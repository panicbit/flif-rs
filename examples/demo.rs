extern crate flif;
extern crate env_logger;

use std::fs::File;
use std::env;

fn main() {
    env_logger::init().unwrap();
    let path = env::args().nth(1).expect("usage: demo <image.flif>");
    let mut file = File::open(path).unwrap();

    let builder = flif::dec::decode(&mut file).unwrap();

    println!("{:#?}", builder.info());

    flif::dec::decode_image(builder, Default::default()).unwrap();

}
