use crate::p5::P5;
use crate::IntoColor;
use crate::Sketch;
use euclid::{point2, vec2, Angle};
use raqote::{DrawOptions, DrawTarget, PathBuilder, Source};

pub struct RaqoteP5 {
    dt: DrawTarget,
    fill_color: raqote::Color,
    stroke_weight: f32,
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
            fill_color: raqote::Color::new(255, 0, 0, 0),
            stroke_weight: 1.,
        }
    }
}

impl P5 for RaqoteP5 {
    fn background<C: IntoColor>(&mut self, c: C) {
        let c: raqote::Color = c.into().into();
        self.dt.clear(c.into());
    }

    fn ellipse(&mut self, x: f32, y: f32, w: f32, h: f32) {
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

        let path = pb.finish();

        self.dt.fill(
            &path,
            &Source::Solid(self.fill_color.into()),
            &DrawOptions::default(),
        );
    }

    fn circle(&mut self, x: f32, y: f32, d: f32) {
        self.ellipse(x, y, d, d);
    }

    fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let mut pb = PathBuilder::new();
        pb.move_to(x1, y1);
        pb.line_to(x2, y2);
        let path = pb.finish();

        let mut stroke = raqote::StrokeStyle::default();
        stroke.width = self.stroke_weight;

        self.dt.stroke(
            &path,
            &Source::Solid(self.fill_color.into()),
            &stroke,
            &DrawOptions::default(),
        );
    }

    fn get_data(&self) -> &[u32] {
        self.dt.get_data()
    }
}
