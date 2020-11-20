use crate::p5::{RectMode, P5};
use crate::IntoColor;
use crate::Sketch;
use euclid::{point2, vec2, Angle};
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
        }
    }

    fn draw_path(&mut self, path: raqote::Path) {
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
        let c: raqote::Color = c.into().into();
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
        let path = create_ellipse_path(x, y, self.stroke_weight, self.stroke_weight);
        self.dt.fill(
            &path,
            &Source::Solid(self.stroke_color.into()),
            &DrawOptions::default(),
        );
    }

    fn stroke_weight(&mut self, weight: f32) {
        self.stroke_weight = weight;
    }

    fn no_stroke(&mut self) {
        self.stroke_weight = 0.0;
    }

    fn stroke<C: IntoColor>(&mut self, color: C) {
        self.stroke_color = color.into().into();
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

    fn no_fill(&mut self) {
        self.fill_color = None;
    }

    fn get_data(&self) -> &[u32] {
        self.dt.get_data()
    }
}
