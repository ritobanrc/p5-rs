# p5-rs
A WIP library that attempts to mimic the p5.js API in Rust, for fast, easy, and accessible creative coding. 

## Getting Started
While this library isn't available on crates.io yet, once it is, using it should be as simple as adding `p5-rs = "0.1.0"` to your `Cargo.toml`. 

## Example

```rust
fn main() {
    struct LineTest;

    impl Sketch for LineTest {
        fn setup(&mut self, p5: &mut P5) {
            p5.background(255);
            p5.line(20., 20., 380., 380.);
        }
        fn draw(&mut self, _p5: &mut P5) {}
    }

    LineTest.run();
}
```

