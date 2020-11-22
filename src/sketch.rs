use crate::p5::P5 as P5Trait;
use crate::P5;
use minifb::{Key, Window, WindowOptions};

pub trait Sketch {
    /// The setup() function is called once when the program starts. It's used to define
    /// initial environment properties such as screen size and background color and to load
    /// media such as images and fonts as the program starts. There can only be one setup()
    /// function for each program and it shouldn't be called again after its initial execution.
    ///
    /// The default implementation of [`setup`](crate::sketch::Sketch::setup) is empty.
    fn setup(&mut self, _p5: &mut P5) {}

    fn draw(&mut self, _p5: &mut P5) {}

    fn title(&self) -> &'static str {
        "p5-rs Window"
    }

    fn width(&self) -> usize {
        400
    }

    fn height(&self) -> usize {
        400
    }

    fn run(&mut self)
    where
        Self: std::marker::Sized,
    {
        let mut window = Window::new(
            self.title(),
            self.width(),
            self.height(),
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        let mut p5 = P5::new(self);
        self.setup(&mut p5);

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(
            (1_000_000. / p5.frame_rate) as u64,
        )));

        while window.is_open() && !window.is_key_down(Key::Escape) {
            // TODO: This isn't specified in the p5 trait, why can we assume it exists?
            p5.frame_count += 1;
            p5.reset_matrix();
            self.draw(&mut p5);

            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            window
                .update_with_buffer(p5.get_data(), self.width(), self.height())
                .unwrap();
        }
    }
}
