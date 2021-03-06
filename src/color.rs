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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Enum representing a Color Model. See https://en.wikipedia.org/wiki/Color_model#HSL_and_HSV for
/// more information.
pub enum ColorModel {
    RGB,
    HSL,
    HSB,
}

/// The `ColorMode` describes how the paramters for fill(), stroke(), background(), and color() are
/// interpreted as color data. The `ColorModel` specifies how the 3 values should be interepreted
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ColorMode {
    pub model: ColorModel,
    pub max_1: f32,
    pub max_2: f32,
    pub max_3: f32,
    pub max_a: f32,
}

pub const RGB: ColorMode = ColorMode {
    model: ColorModel::RGB,
    max_1: 255.,
    max_2: 255.,
    max_3: 255.,
    max_a: 255.,
};

const RGB_01: ColorMode = ColorMode {
    model: ColorModel::RGB,
    max_1: 1.,
    max_2: 1.,
    max_3: 1.,
    max_a: 1.,
};

pub const HSB: ColorMode = ColorMode {
    model: ColorModel::HSB,
    max_1: 360.,
    max_2: 100.,
    max_3: 100.,
    max_a: 1.,
};

pub const HSL: ColorMode = ColorMode {
    model: ColorModel::HSL,
    max_1: 360.,
    max_2: 100.,
    max_3: 100.,
    max_a: 1.,
};

impl ColorMode {
    pub fn new(model: ColorModel, max: f32) -> Self {
        ColorMode {
            model,
            max_1: max,
            max_2: max,
            max_3: max,
            max_a: max,
        }
    }

    pub fn with_maxes(model: ColorModel, max_1: f32, max_2: f32, max_3: f32, max_a: f32) -> Self {
        ColorMode {
            model,
            max_1,
            max_2,
            max_3,
            max_a,
        }
    }
}

/// A trait for all the different types that can be converted into colors, in order to mimic the
/// function overloading present in p5 (i.e. allowing you to specify just 1 value and it being
/// interpreted as the grayscale color).
pub trait IntoColor {
    fn into_color(self, mode: ColorMode) -> Color;
}

/// Converts a single `f32` into a grayscale color. Note that this will _always_ produce a
/// grayscale color, regardless of whether the curret color mode is RGB, HSB, or HSL. However, this
/// will factor in the current `max_3` value in the `ColorMode`, as this appears to be the current
/// behavior in p5.
impl IntoColor for f32 {
    fn into_color(self, mode: ColorMode) -> Color {
        let brightness = (self / mode.max_3 * 255.) as u8;
        Color::new(brightness, brightness, brightness, 255)
    }
}

/// Converts a single `u8` into a grayscale color. Note that this will _always_ produce a
/// grayscale color, regardless of the current `color_mode`. Since `u8`s are always in the range
/// 0-255, this will not factor in the currently set `max` values.
impl IntoColor for u8 {
    fn into_color(self, _mode: ColorMode) -> Color {
        Color::new(self, self, self, 255)
    }
}

/// Converts a tuple of 3 floating-point numbers into a `Color`.
impl IntoColor for (f32, f32, f32) {
    fn into_color(self, mode: ColorMode) -> Color {
        IntoColor::into_color([self.0, self.1, self.2, mode.max_a], mode)
    }
}

/// Converts a array of 3 floating-point numbers into a `Color`.
impl IntoColor for [f32; 3] {
    fn into_color(self, mode: ColorMode) -> Color {
        IntoColor::into_color([self[0], self[1], self[2], mode.max_a], mode)
    }
}

/// Converts a tuple of 4 floating-point numbers into a `Color`.
impl IntoColor for (f32, f32, f32, f32) {
    fn into_color(self, mode: ColorMode) -> Color {
        IntoColor::into_color([self.0, self.1, self.2, self.3], mode)
    }
}

/// Converts an array of 4 floating-point numbers into a `Color`.
impl IntoColor for [f32; 4] {
    fn into_color(self, mode: ColorMode) -> Color {
        fn clamp01(x: f32) -> f32 {
            match x {
                _ if x < 0. => 0.,
                _ if x > 1. => 1.,
                x => x,
            }
        }

        let scaled: [f32; 4] = [
            clamp01(self[0] / mode.max_1),
            clamp01(self[1] / mode.max_2),
            clamp01(self[2] / mode.max_3),
            clamp01(self[3] / mode.max_a),
        ];

        match mode.model {
            ColorModel::RGB => IntoColor::into_color(
                [
                    (scaled[0] * 255.) as u8,
                    (scaled[1] * 255.) as u8,
                    (scaled[2] * 255.) as u8,
                    (scaled[3] * 255.) as u8,
                ],
                RGB,
            ),
            ColorModel::HSB => {
                let rgb = hsb_to_rgb(scaled[0] * 360., scaled[1], scaled[2]);
                IntoColor::into_color([rgb[0], rgb[1], rgb[2], scaled[3]], RGB_01)
            }
            ColorModel::HSL => {
                let rgb = hsl_to_rgb(scaled[0] * 360., scaled[1], scaled[2]);
                IntoColor::into_color([rgb[0], rgb[1], rgb[2], scaled[3]], RGB_01)
            }
        }
    }
}

/// Converts four `u8`s in RGBA order into a color. Since `u8`s are always in the range
/// 0-255, this IGNORES the `mode` -- always assuming RGB.
impl IntoColor for [u8; 4] {
    fn into_color(self, _mode: ColorMode) -> Color {
        Color::new(self[0], self[1], self[2], self[3])
    }
}

/// Assumes `h` in [0, 360], `s` in [0, 1], `b` in [0, 1].
/// See https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_RGB
fn hsb_to_rgb(h: f32, s: f32, b: f32) -> [f32; 3] {
    let f = |n| {
        let k: f32 = (n + h / 60.) % 6.;
        b - b * s * k.min(4. - k).min(1.).max(0.)
    };

    [f(5.), f(3.), f(1.)]
}

/// Assumes `h` in [0, 360], `s` in [0, 1], `l` in [0, 1].
/// See https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_RGB
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> [f32; 3] {
    let f = |n| {
        let k: f32 = (n + h / 30.) % 12.;
        let a = s * l.min(1. - l);
        l - a * (k - 3.).min(9. - k).min(1.).max(-1.)
    };

    [f(0.), f(8.), f(4.)]
}

/// Assumes `h` in [0, 360], `s` in [0, 1], `b` in [0, 1].
/// This is an alternative implementation of HSB to RGB. It should produce equivalent results to
/// the more efficient implementation above.
/// See https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_RGB
#[allow(dead_code)]
fn hsb_to_rgb_alt(h: f32, s: f32, b: f32) -> [f32; 3] {
    let c = b * s;
    let h_prime = h / 60.;
    let x = c * (1. - (h_prime % 2. - 1.).abs());

    let (r1, g1, b1) = match h_prime {
        h if 0. <= h && h <= 1. => (c, x, 0.),
        h if 1. < h && h <= 2. => (x, c, 0.),
        h if 2. < h && h <= 3. => (0., c, x),
        h if 3. < h && h <= 4. => (0., x, c),
        h if 4. < h && h <= 5. => (x, 0., c),
        h if 5. < h && h <= 6. => (c, 0., x),
        _ => panic!("Invalid hue value: {:?}", h),
    };

    let m = b - c;
    [r1 + m, g1 + m, b1 + m]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hsb_to_rgb_test() {
        let rgb = vec![
            (1., 1., 1.),
            (0.5, 0.5, 0.5),
            (0., 0., 0.),
            (1., 0., 0.),
            (0.75, 0.75, 0.),
            (0., 0.5, 0.),
            (0.5, 1., 1.),
            (0.5, 0.5, 1.),
        ];
        let hsb = vec![
            (0., 0., 1.),
            (0., 0., 0.5),
            (0., 0., 0.),
            (0., 1., 1.),
            (60., 1., 0.75),
            (120., 1., 0.5),
            (180., 0.5, 1.),
            (240., 0.5, 1.),
        ];

        for (rgb, hsb) in rgb.into_iter().zip(hsb) {
            let rgb_result = hsb_to_rgb(hsb.0, hsb.1, hsb.2);
            let rgb_result_2 = hsb_to_rgb_alt(hsb.0, hsb.1, hsb.2);

            let error = (rgb.0 - rgb_result[0]).powi(2)
                + (rgb.1 - rgb_result[1]).powi(2)
                + (rgb.2 - rgb_result[2]).powi(2);

            let error2 = (rgb.0 - rgb_result_2[0]).powi(2)
                + (rgb.1 - rgb_result_2[1]).powi(2)
                + (rgb.2 - rgb_result_2[2]).powi(2);

            assert!(error < 0.001);
            assert!(error2 < 0.001);
        }
    }
}
