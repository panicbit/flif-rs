#[macro_use]
extern crate quick_error;
extern crate podio;
extern crate flate2;

mod image;
pub mod dec;
mod varint;
mod format;
mod metadata;
mod maniac;

pub use image::*;
