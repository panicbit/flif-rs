pub mod image;
pub mod color_ranges;
pub mod transforms;

pub use self::image::Image;
pub use self::color_ranges::ColorRanges;
pub use self::transforms::Transform;

pub type ColorVal = i32;
