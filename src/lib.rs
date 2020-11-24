mod backend;
mod color;
mod p5;
mod sketch;

pub use color::{Color, ColorMode, ColorModel, IntoColor, HSB, HSL, RGB};
pub use p5::P5 as P5Trait;
pub use p5::{RectMode, RectRounding};
pub use sketch::Sketch;

pub type P5 = backend::raqote::RaqoteP5;
