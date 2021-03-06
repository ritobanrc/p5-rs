use crate::p5::{RectMode, P5};
use crate::Sketch;
use crate::{ColorMode, IntoColor};
use euclid::{point2, vec2, Angle, Transform2D, UnknownUnit};
use font_kit::font::Font;
use raqote::{DrawOptions, DrawTarget, PathBuilder, Source};

/// A structure that contains all the internal state necessary for drawing with the raqote backend.
pub struct RaqoteP5 {
    /// The raqote [`DrawTarget`](raqote::DrawTarget).
    dt: DrawTarget,
    /// The fill color used to fill in shapes. If [`None`](std::option::Option), the shape is
    /// transparent.
    fill_color: Option<raqote::Color>,
    /// The color used to draw lines and borders around shapes.
    stroke_color: raqote::Color,
    /// The width of the stroke used for lines, points and the border around shapes.
    stroke_weight: f32,
    /// The current [`RectMode`](crate::p5::RectMode). The default is RectMode::Corner.
    rect_mode: RectMode,
    /// The current transformation that should be applied to shapes.
    transform: Transform2D<f32, UnknownUnit, UnknownUnit>,
    /// The current color mode
    color_mode: ColorMode,
    /// The variable frame_count contains the number of frames that have been displayed since the program started. Inside setup() the value is 0, after the first iteration of draw it is 1, etc.
    pub frame_count: usize,
    pub frame_rate: f32,
    pub(crate) keys: Option<Vec<crate::Key>>,
    /// If `Some`, contains the ASCII character value of the most recent key on the keyboard that was typed, _mostly_ respecting capitalization (please file a bug report if you find a sitaution where it doesn't). If this is `None`, that may mean that no key was pressed, or that the key is not an ascii character.
    pub key: Option<char>,
    /// If `Some`, contains the most recent key pressed on the keyboard as a [`Key`](crate::Key). Instead of a separate `keyIsPressed` variable, this uses an `Option`.
    pub key_code: Option<crate::Key>,

    /// Sets/gets the current font size. This size will be used in all subsequent calls to the text() function. Font size is measured in _points_.
    text_size: f32,
    /// The current font
    font: Font,
}

impl From<crate::Color> for raqote::Color {
    fn from(c: crate::Color) -> raqote::Color {
        raqote::Color::new(c.a, c.r, c.g, c.b)
    }
}

impl RaqoteP5 {
    pub fn new<S: Sketch>(sketch: &S) -> RaqoteP5 {
        RaqoteP5 {
            dt: DrawTarget::new(sketch.width() as i32, sketch.height() as i32),
            fill_color: Some(raqote::Color::new(255, 255, 255, 255)),
            stroke_color: raqote::Color::new(255, 0, 0, 0),
            stroke_weight: 1.,
            rect_mode: RectMode::Corner,
            transform: Transform2D::identity(),
            color_mode: crate::RGB,
            frame_count: 0,
            // TODO: p5js docs say the default framerate is based on the monitor refresh rate, but we hard code it to be 60.
            frame_rate: 60.,
            keys: None,
            key: None,
            key_code: None,
            text_size: 32., // this is what the default text size looks like in p5.js
            font: font_kit::source::SystemSource::new()
                .select_best_match(
                    &[font_kit::family_name::FamilyName::SansSerif],
                    &font_kit::properties::Properties::default(),
                )
                .expect("Default sans-serif font not found")
                .load()
                .expect("Failed to load default sans-serif font"),
        }
    }

    fn transform_path(&self, path: raqote::Path) -> raqote::Path {
        // Hack because raqote uses an old version of euclid, so we copy the data inside the
        // transform.
        let transform = raqote::Transform::from_row_major_array(self.transform.to_array());
        path.transform(&transform)
    }

    /// Draws a path correctly using the stroke weight, stroke color, fill color, etc.
    /// attribiutes. Also transforms `path` using `self.transform` before drawing.
    fn draw_path(&mut self, path: raqote::Path) {
        let path = self.transform_path(path);
        if self.stroke_weight != 0.0 {
            let stroke_style = {
                let mut s = raqote::StrokeStyle::default();
                s.width = self.stroke_weight;
                s
            };

            self.dt.stroke(
                &path,
                &self.stroke_color.into(),
                &stroke_style,
                &DrawOptions::default(),
            );
        }

        if let Some(fill_color) = self.fill_color {
            self.dt.fill(
                &path,
                &Source::Solid(fill_color.into()),
                &DrawOptions::default(),
            );
        }
    }
}

fn create_ellipse_path(x: f32, y: f32, w: f32, h: f32) -> raqote::Path {
    let arc = lyon_geom::Arc {
        center: point2(x, y),
        radii: vec2(w / 2., h / 2.),
        start_angle: Angle::zero(),
        sweep_angle: Angle::two_pi(),
        x_rotation: Angle::zero(),
    };

    let mut pb = PathBuilder::new();

    let start = arc.from();
    pb.line_to(start.x, start.y);

    arc.for_each_quadratic_bezier(&mut |q| {
        pb.quad_to(q.ctrl.x, q.ctrl.y, q.to.x, q.to.y);
    });

    pb.finish()
}

impl P5 for RaqoteP5 {
    fn background<C: IntoColor>(&mut self, c: C) {
        let c: raqote::Color = c.into_color(self.color_mode).into();
        self.dt.clear(c.into());
    }

    fn ellipse(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.draw_path(create_ellipse_path(x, y, w, h));
    }

    fn circle(&mut self, x: f32, y: f32, d: f32) {
        self.ellipse(x, y, d, d);
    }

    fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        if self.stroke_weight != 0. {
            let mut pb = PathBuilder::new();
            pb.move_to(x1, y1);
            pb.line_to(x2, y2);
            let path = pb.finish();

            let mut stroke = raqote::StrokeStyle::default();
            stroke.width = self.stroke_weight;

            self.dt.stroke(
                &path,
                &Source::Solid(self.stroke_color.into()),
                &stroke,
                &DrawOptions::default(),
            );
        } else {
            eprintln!("Warning -- `P5::line` -- `stroke_weight` is 0., so calling `line`  doesn't do anything. Consider calling `P5::stroke_weight` with a non-zero stroke weight.");
        }
    }

    fn point(&mut self, x: f32, y: f32) {
        if self.stroke_weight == 1. && self.stroke_color.a() == 255 {
            let point = self.transform.transform_point(point2(x, y));
            let idx = point.y as i32 * self.dt.width() + point.x as i32;
            // Safety: A struct with only one field has the same layout as that field. A raqote::Color is just a u32.
            self.dt.get_data_mut()[idx as usize] =
                unsafe { std::mem::transmute(self.stroke_color) };
        } else {
            // TODO: Using an ellipse here is _incredibly_ innefficient for small strokeweights.
            // Additionally, when alpha != 1, but strokeweight == 1, it may even be incorrect, as
            // anti-aliasing would cause it to be drawn less brightly than it should be.
            let path = create_ellipse_path(x, y, self.stroke_weight, self.stroke_weight);
            self.dt.fill(
                &self.transform_path(path),
                &Source::Solid(self.stroke_color.into()),
                &DrawOptions::default(),
            );
        }
    }

    fn stroke_weight(&mut self, weight: f32) {
        self.stroke_weight = weight;
    }

    fn no_stroke(&mut self) {
        self.stroke_weight = 0.0;
    }

    fn stroke<C: IntoColor>(&mut self, color: C) {
        self.stroke_color = color.into_color(self.color_mode).into();
    }

    fn fill<C: IntoColor>(&mut self, color: C) {
        self.fill_color = Some(color.into_color(self.color_mode).into());
    }

    fn quad(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32) {
        let mut pb = PathBuilder::new();
        pb.move_to(x1, y1);
        pb.line_to(x2, y2);
        pb.line_to(x3, y3);
        pb.line_to(x4, y4);
        pb.close();
        let path = pb.finish();

        self.draw_path(path);
    }

    fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, rounding: Option<crate::p5::RectRounding>) {
        let mut pb = PathBuilder::new();

        let corners = match self.rect_mode {
            RectMode::Corner => [(x, y), (x + w, y), (x + w, y + h), (x, y + h)],
            RectMode::Corners => [(x, y), (w, y), (w, h), (x, h)],
            RectMode::Center => [
                (x - w / 2., y - h / 2.),
                (x + w / 2., y - h / 2.),
                (x + w / 2., y + h / 2.),
                (x - w / 2., y + h / 2.),
            ],
            RectMode::Radius => [
                (x - w, y - h),
                (x + w, y - h),
                (x + w, y + h),
                (x - w, y + h),
            ],
        };

        for (i, corner) in corners.iter().enumerate() {
            if let Some(rounding) = &rounding {
                let rad = match i {
                    0 => rounding.tl,
                    1 => rounding.tr,
                    2 => rounding.br,
                    3 => rounding.bl,
                    _ => unreachable!("There are only 4 corners, but is `i` more than 3"),
                };

                if rad == 0. {
                    break; // just do the default thing
                }

                let center = match i {
                    0 => (corner.0 + rad, corner.1 + rad),
                    1 => (corner.0 - rad, corner.1 + rad),
                    2 => (corner.0 - rad, corner.1 - rad),
                    3 => (corner.0 + rad, corner.1 - rad),
                    _ => unreachable!("There are only 4 corners, but is `i` more than 3"),
                };

                let start = Angle::pi() + Angle::frac_pi_2() * (i as f32);

                let tl_arc = lyon_geom::Arc {
                    center: point2(center.0, center.1),
                    radii: vec2(rad, rad),
                    start_angle: start,
                    sweep_angle: Angle::frac_pi_2(),
                    x_rotation: Angle::zero(),
                };

                let start = tl_arc.from();
                pb.line_to(start.x, start.y);

                tl_arc.for_each_quadratic_bezier(&mut |q| {
                    pb.quad_to(q.ctrl.x, q.ctrl.y, q.to.x, q.to.y)
                });

                continue;
            }

            // if there's no rounding, just move along the corners
            pb.line_to(corner.0, corner.1);
        }

        pb.close();

        let path = pb.finish();
        self.draw_path(path);
    }

    fn rect_mode(&mut self, mode: RectMode) {
        self.rect_mode = mode;
    }

    fn triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        let mut pb = PathBuilder::new();
        pb.move_to(x1, y1);
        pb.line_to(x2, y2);
        pb.line_to(x3, y3);
        pb.close();
        let path = pb.finish();

        self.draw_path(path);
    }

    fn reset_matrix(&mut self) {
        self.transform = Transform2D::identity();
    }

    fn apply_matrix(&mut self, m11: f32, m12: f32, m21: f32, m22: f32, m31: f32, m32: f32) {
        self.transform = self
            .transform
            .then(&Transform2D::new(m11, m12, m21, m22, m31, m32));
    }

    fn no_fill(&mut self) {
        self.fill_color = None;
    }

    fn frame_rate(&mut self, fps: f32) {
        self.frame_rate = fps;
    }

    fn color_mode(&mut self, mode: ColorMode) {
        self.color_mode = mode;
    }

    fn key_is_down(&self, key: crate::Key) -> bool {
        // TODO: Instead of calling `contains`, directly use the `window.is_key_down`
        // function
        self.keys.as_ref().map_or(false, |keys| keys.contains(&key))
    }

    fn text(&mut self, s: &str, x: f32, y: f32) {
        if let Some(fill_color) = self.fill_color {
            let mut options = DrawOptions::new();
            options.antialias = raqote::AntialiasMode::Gray;
            self.dt.draw_text(
                &self.font,
                self.text_size,
                s,
                raqote::Point::new(x, y),
                &Source::Solid(fill_color.into()),
                &options,
            );
        }
    }

    fn text_size(&mut self, size: f32) {
        self.text_size = size;
    }

    // TODO: Better error handling here
    fn text_font(&mut self, family_name: &str) {
        use font_kit::{family_name::FamilyName, properties::Properties, source::SystemSource};
        let family_name = match family_name {
            "serif" => FamilyName::Serif,
            "sans-serif" => FamilyName::SansSerif,
            "monospace" => FamilyName::Monospace,
            "cursive" => FamilyName::Cursive,
            "fantasy" => FamilyName::Fantasy,
            x => FamilyName::Title(x.to_owned()),
        };

        self.font = SystemSource::new()
            .select_best_match(&[family_name], &Properties::default())
            .expect("Invalid font specified")
            .load()
            .expect("Failed to load font.");
    }

    fn get_data(&self) -> &[u32] {
        self.dt.get_data()
    }
}
