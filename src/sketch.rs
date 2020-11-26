use crate::p5::P5 as P5Trait;
use crate::{Key, P5};
use minifb::{Window, WindowOptions};

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

    fn key_pressed(&mut self, _p5: &mut P5, _keys: Vec<Key>) {}
    fn key_released(&mut self, _p5: &mut P5, _keys: Vec<Key>) {}
    fn key_typed(&mut self, _p5: &mut P5, _chars: Vec<char>) {}

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

            p5.keys = window
                .get_keys()
                .and_then(|keys| if keys.len() == 0 { Some(keys) } else { None });

            if let Some(ref keys) = p5.keys {
                if keys.len() > 0 {
                    p5.key_code = Some(keys[keys.len() - 1]);
                }
            }

            if let Some(keys_pressed) = window.get_keys_pressed(minifb::KeyRepeat::No) {
                if keys_pressed.len() > 0 {
                    self.key_pressed(&mut p5, keys_pressed);
                }
            }

            if let Some(keys_released) = window.get_keys_released() {
                if keys_released.len() > 0 {
                    self.key_released(&mut p5, keys_released);
                }
            }

            // TODO: Deal with Caps Lock
            let caps = window.is_key_down(Key::LeftShift) || window.is_key_down(Key::RightShift);
            let chars: Vec<_> = p5
                .keys
                .iter()
                .flatten()
                .filter_map(|&k| key_to_char(k, caps))
                .collect();

            if chars.len() > 0 {
                self.key_typed(&mut p5, chars);
            }

            self.draw(&mut p5);

            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            window
                .update_with_buffer(p5.get_data(), self.width(), self.height())
                .unwrap();
        }
    }
}

/// Attempts to convert a `Key` into the corresponding character, because minifb doesn't expose a
/// sensible API for directly getting characters from keys (the `InputCallback` API is far too
/// cumbersome).
///
/// TODO: Make this respect caps for numeric keys, and make it work for symbols
fn key_to_char(key: Key, caps: bool) -> Option<char> {
    match key {
        key if key >= Key::A && key <= Key::Z => {
            let n = (key as u8) - (Key::A as u8);
            match caps {
                true => Some((n + b'A') as char),
                false => Some((n + b'a') as char),
            }
        }
        key if key >= Key::Key0 && key <= Key::Key9 => {
            let n = (key as u8) - (Key::Key0 as u8);
            Some((n + b'0') as char)
        }
        _ => None,
    }
}
