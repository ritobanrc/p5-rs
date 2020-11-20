mod backend;
mod color;
mod p5;
mod sketch;

pub use color::{Color, IntoColor};
pub use p5::P5 as P5Trait;
pub use p5::{RectMode, RectRounding};
pub use sketch::Sketch;

type P5 = backend::raqote::RaqoteP5;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        struct BasicTest;

        impl Sketch for BasicTest {
            fn setup(&mut self, _p5: &mut P5) {}
            fn draw(&mut self, _p5: &mut P5) {}
        }

        BasicTest.run();
    }

    #[test]
    fn background() {
        struct BackgroundTest;

        impl Sketch for BackgroundTest {
            fn setup(&mut self, p5: &mut P5) {
                p5.background((1., 0., 0.));
            }
        }

        BackgroundTest.run();
    }

    #[test]
    fn changing_background() {
        struct BackgroundTest(f32);

        impl Sketch for BackgroundTest {
            fn setup(&mut self, _p5: &mut P5) {}
            fn draw(&mut self, p5: &mut P5) {
                p5.background((self.0.sin(), self.0.cos(), 0.));
                self.0 += 0.01;
            }
        }

        BackgroundTest(0.).run();
    }

    #[test]
    fn ellipse() {
        struct EllipseTest;

        impl Sketch for EllipseTest {
            fn setup(&mut self, p5: &mut P5) {
                p5.background(220);
                p5.ellipse(200., 200., 100., 50.);
            }
        }

        EllipseTest.run();
    }

    #[test]
    fn line() {
        struct LineTest;

        impl Sketch for LineTest {
            fn setup(&mut self, p5: &mut P5) {
                p5.background(220);
                p5.line(20., 20., 380., 380.);
            }
        }

        LineTest.run();
    }

    #[test]
    fn point() {
        struct PointTest;

        impl Sketch for PointTest {
            fn setup(&mut self, p5: &mut P5) {
                p5.background(220);
                for i in 0..20u8 {
                    p5.stroke_weight(f32::from(i));
                    p5.point(f32::from(i) * 20. + 10., 200.);
                }
            }
        }

        PointTest.run();
    }

    #[test]
    fn stroke() {
        struct StrokeTest;

        impl Sketch for StrokeTest {
            fn setup(&mut self, p5: &mut P5) {
                p5.background(220);
                for i in 0..5u8 {
                    for j in 0..5u8 {
                        let (i, j) = (f32::from(i), f32::from(j));

                        // pick a row to test `no_stroke` on
                        if j == 3. {
                            p5.no_stroke();
                        } else {
                            p5.stroke_weight(i * 2.);
                        }

                        let x = i * 80. + 35.;
                        let y = j * 80. + 35.;

                        p5.stroke((0., x / 400., y / 400.));

                        p5.circle(x, y, 60.);
                    }
                }
            }
        }

        StrokeTest.run();
    }

    #[test]
    fn quad() {
        struct QuadTest;

        impl Sketch for QuadTest {
            fn setup(&mut self, p5: &mut P5) {
                p5.background(220);
                for i in 0..3u8 {
                    let x = f32::from(i) * 100.;
                    p5.stroke_weight(f32::from(i));
                    p5.quad(38. + x, 31., 86. + x, 20., 69. + x, 63., 30. + x, 76.);
                }
            }
        }

        QuadTest.run();
    }

    #[test]
    fn rect() {
        struct RectTest;

        impl Sketch for RectTest {
            fn setup(&mut self, p5: &mut P5) {
                p5.background(220);

                p5.rect(30., 20., 55., 55., None);
                p5.rect(100., 20., 75., 55., None);
                p5.rect(200., 20., 55., 55., RectRounding::equal(20.));
                p5.rect(300., 20., 55., 55., RectRounding::new(20., 15., 5., 30.));

                p5.rect_mode(RectMode::Corners);

                p5.rect(30., 100., 85., 155., None);
                p5.rect(100., 100., 175., 155., None);
                p5.rect(200., 100., 255., 155., RectRounding::equal(20.));
                p5.rect(300., 100., 355., 155., RectRounding::new(20., 15., 5., 30.));

                p5.rect_mode(RectMode::Center);

                p5.rect(30., 200., 55., 55., None);
                p5.rect(100., 200., 75., 55., None);
                p5.rect(200., 200., 55., 55., RectRounding::equal(20.));
                p5.rect(300., 200., 55., 55., RectRounding::new(20., 15., 5., 30.));

                p5.rect_mode(RectMode::Radius);

                p5.rect(30., 300., 25., 25., None);
                p5.rect(100., 300., 35., 25., None);
                p5.rect(200., 300., 25., 25., RectRounding::equal(20.));
                p5.rect(300., 300., 25., 25., RectRounding::new(20., 15., 5., 30.));
            }
        }

        RectTest.run();
    }
}
