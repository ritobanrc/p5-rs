use p5_rs::*;

struct KeyboardExample;

impl Sketch for KeyboardExample {
    fn key_pressed(&mut self, _p5: &mut P5, keys: Vec<Key>) {
        println!("pressed {:?}", keys);
    }
    fn key_released(&mut self, _p5: &mut P5, keys: Vec<Key>) {
        println!("released {:?}", keys);
    }
    fn key_typed(&mut self, _p5: &mut P5, keys: Vec<char>) {
        println!("typed {:?}", keys);
    }
}

fn main() {
    KeyboardExample.run()
}
