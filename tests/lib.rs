use p5_rs::*;

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

#[test]
fn triangle() {
    struct TriangleTest;

    impl Sketch for TriangleTest {
        fn setup(&mut self, p5: &mut P5) {
            p5.background(220);
            p5.triangle(30., 75., 58., 20., 86., 75.);
        }
    }

    TriangleTest.run();
}

#[test]
fn matrix() {
    struct MatrixTest;

    impl Sketch for MatrixTest {
        fn draw(&mut self, p5: &mut P5) {
            let step = (p5.frame_count % 20) as f32;
            p5.background(200);
            p5.apply_matrix(1., 0., 0., 1., 40. + step, 50.);
            p5.rect(0., 0., 50., 50., None);
            p5.reset_matrix();

            p5.apply_matrix(1. / step, 0., 0., 1. / step, 200., 50.);
            p5.rect_mode(RectMode::Center);
            p5.rect(0., 0., 50., 50., None);
            p5.reset_matrix();

            let angle = step / 20. * 2. * std::f32::consts::PI;
            let cos_a = angle.cos();
            let sin_a = angle.sin();
            p5.apply_matrix(cos_a, sin_a, -sin_a, cos_a, 300., 50.);
            p5.rect_mode(RectMode::Center);
            p5.rect(0., 0., 50., 50., None);
            p5.reset_matrix();
        }
    }

    MatrixTest.run();
}

#[test]
fn transforms() {
    struct TransformsTest;

    impl Sketch for TransformsTest {
        fn draw(&mut self, p5: &mut P5) {
            let step = p5.frame_count as f32 / 10. % 20.;
            p5.background(200);
            p5.rect_mode(RectMode::Center);

            p5.translate(40., step + 50.);
            p5.rect(0., 0., 50., 50., None);
            p5.reset_matrix();

            p5.scale(1. / step);
            p5.translate(200., 50.);
            p5.rect(0., 0., 50., 50., None);
            p5.reset_matrix();

            let angle = step / 20. * 2. * std::f32::consts::PI;
            p5.rotate(angle);
            p5.translate(300., 50.);
            p5.rect(0., 0., 50., 50., None);
            p5.reset_matrix();

            p5.shear_x(angle);
            p5.translate(200., 200.);
            p5.rect(0., 0., 50., 50., None);
            p5.reset_matrix();

            p5.shear_y(angle);
            p5.translate(200., 300.);
            p5.rect(0., 0., 50., 50., None);
            p5.reset_matrix();
        }
    }

    TransformsTest.run();
}

#[test]
fn colors() {
    struct ColorsTest;

    impl Sketch for ColorsTest {
        fn draw(&mut self, p5: &mut P5) {
            p5.background(0);

            p5.translate(10., 10.);

            p5.color_mode(ColorMode::new(ColorModel::RGB, 100.));
            p5.stroke_weight(1.);
            for i in 0..100 {
                for j in 0..100 {
                    p5.stroke((i as f32, j as f32, 0.));
                    p5.point(i as f32, j as f32);
                }
            }

            p5.translate(120., 0.);
            p5.color_mode(HSB);
            p5.color_mode(ColorMode::new(ColorModel::HSB, 100.));
            for i in 0..100 {
                for j in 0..100 {
                    p5.stroke((i as f32, j as f32, 100.));
                    p5.point(i as f32, j as f32);
                }
            }

            p5.translate(120., 0.);
            p5.no_fill();
            p5.color_mode(RGB);
            p5.stroke_weight(4.);
            p5.stroke((255., 0., 10., 75.));
            p5.ellipse(40., 40., 50., 50.);
            p5.ellipse(50., 50., 40., 40.);

            p5.reset_matrix();
            p5.translate(0., 120.);
            p5.stroke(21.);
            p5.fill((220., 15., 0., 150.));
            p5.ellipse(40., 40., 50., 50.);
            p5.ellipse(50., 50., 40., 40.);
        }
    }

    ColorsTest.run();
}

#[test]
fn text() {
    struct TextTest;

    impl Sketch for TextTest {
        fn setup(&mut self, p5: &mut P5) {
            p5.text_size(30.);
            p5.text_font("Roboto");
            p5.text("This is Roboto!", 100., 100.);

            p5.text_font("Linux Libertine");
            p5.text("This is Libertine!", 100., 200.);

            p5.text_font("Fira Code");
            p5.text("This is Fira Code!", 100., 300.);
        }
    }

    TextTest.run();
}
