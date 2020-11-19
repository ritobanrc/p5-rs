mod backend;
mod color;
mod p5;
mod sketch;

pub use color::{Color, IntoColor};
pub use p5::P5 as P5Trait;
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
                            p5.stroke_weight(i);
                        }

                        let x = i * 80. + 35.;
                        let y = j * 80. + 35.;

                        //p5.stroke((0., i, j));

                        p5.circle(x, y, 60.);
                    }
                }
            }
        }

        StrokeTest.run();
    }
}
