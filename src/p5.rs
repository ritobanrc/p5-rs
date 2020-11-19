use crate::color::IntoColor;

pub trait P5 {
    fn background<C: IntoColor>(&mut self, c: C);

    /// Draws an ellipse (oval) to the screen. By default `x` and `y` specify the center of the
    /// ellipse, and `w` and `h` are the width and height, respectively. The origin may be changed
    /// with the `ellipse_mode` function.
    fn ellipse(&mut self, x: f32, y: f32, w: f32, h: f32);

    /// Draws a circle to the screen. A circle is a simple closed shape. It is the set of all points
    /// in a plane that are at a given distance from a given point, the centre. This function is a
    /// special case of the ellipse() function, where the width and height of the ellipse are the
    /// same. Height and width of the ellipse correspond to the diameter of the circle. By default,
    /// the first two parameters set the location of the centre of the circle, the third sets the
    /// diameter of the circle.
    fn circle(&mut self, x: f32, y: f32, d: f32);

    /// Draws a line (a direct path between two points) to the screen, with a default width of 1
    /// pixel. This width can be modified by using the stroke_weight() function. A line cannot be
    /// filled, therefore the fill() function will not affect the color of a line. So to color a
    /// line, use the stroke() function.
    fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32);

    /// Draws a point, a coordinate in space at the dimension of one pixel. The first parameter
    /// is the horizontal value for the point, the second param is the vertical value for the
    /// point. The color of the point is changed with the stroke() function. The size of the
    /// point can be changed with the stroke_weight() function.
    fn point(&mut self, x: f32, y: f32);

    /// Sets the width of the stroke used for lines, points and the border around shapes. All
    /// widths are set in units of pixels.
    fn stroke_weight(&mut self, weight: f32);

    /// Disables drawing the stroke (outline). If both [`no_stroke`](crate::p5::P5::no_stroke)
    /// and no_fill are called, nothing will be drawn to the screen.
    fn no_stroke(&mut self);

    fn get_data(&self) -> &[u32];
}
