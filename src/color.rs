/// Stores a color as a premultiplied RGBA value.
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }
}

enum ColorModel {
    RGB,
    HSL,
    HSB,
}

/// A trait for all the different types that can be converted into colors, in order to mimic the
/// function overloading present in p5 (i.e. allowing you to specify just 1 value and it being
/// interpreted as the grayscale color).
pub trait IntoColor {
    fn into(self) -> Color;
}

impl IntoColor for f32 {
    fn into(self) -> Color {
        assert!(self >= 0. && self <= 1.);
        let brightness = (self * 255.) as u8;
        Color::new(brightness, brightness, brightness, 255)
    }
}

impl IntoColor for u8 {
    fn into(self) -> Color {
        Color::new(self, self, self, 255)
    }
}

/// Converts a triple of 3 floating-point numbers between 0 and 1 into a `Color`. Any values less
/// than 0 or greater than 1 are clamped to 0 or 1.
impl IntoColor for (f32, f32, f32) {
    fn into(self) -> Color {
        fn clamp01(x: f32) -> f32 {
            match x {
                _ if x < 0. => 0.,
                _ if x > 1. => 1.,
                x => x,
            }
        }
        let (v1, v2, v3) = (
            (clamp01(self.0) * 255.) as u8,
            (clamp01(self.1) * 255.) as u8,
            (clamp01(self.2) * 255.) as u8,
        );

        Color::new(v1, v2, v3, 255)
    }
}
