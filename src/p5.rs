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

    fn stroke<C: IntoColor>(&mut self, color: C);

    /// Draws a quad on the canvas. A quad is a quadrilateral, a four sided polygon. It is similar
    /// to a rectangle, but the angles between its edges are not constrained to ninety degrees. The
    /// first pair of parameters (x1,y1) sets the first vertex and the subsequent pairs should
    /// proceed clockwise or counter-clockwise around the defined shape.
    fn quad(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32);

    /// Draws a rectangle on the canvas. A rectangle is a four-sided closed shape with every angle
    /// at ninety degrees. By default, the first two parameters set the location of the upper-left
    /// corner, the third sets the width, and the fourth sets the height. The way these parameters
    /// are interpreted, may be changed with the rect_mode() function.
    ///
    /// The final parameter, `rounding` specificies how the corners of the rectangle. If `None`, no
    /// rounding will be applied, otherwise, the rounding for each corner is specified in the
    /// `RectRounding` struct.
    fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, rounding: Option<RectRounding>);

    fn rect_mode(&mut self, mode: RectMode);

    /// Draws a square to the screen. A square is a four-sided shape with every angle at ninety
    /// degrees, and equal side size. This function is a special case of the rect() function, where
    /// the width and height are the same, and the parameter is called "s" for side size. By
    /// default, the first two parameters set the location of the upper-left corner, the third sets
    /// the side size of the square. The way these parameters are interpreted, may be changed with
    /// the rect_mode() function.
    fn square(&mut self, x: f32, y: f32, s: f32, rounding: Option<RectRounding>) {
        self.rect(x, y, s, s, rounding)
    }

    /// Draws a trangle to the canvas. A triangle is a shape created by connecting three points.
    /// The first two arguments specify the first point, the middle two arguments specify the
    /// second point, and the last two arguments specify the third point.
    fn triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32);

    fn reset_matrix(&mut self);

    fn no_fill(&mut self);

    fn get_data(&self) -> &[u32];
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RectMode {
    Corner,
    Corners,
    Center,
    Radius,
}

/// Represents the rounding for each corner of a rectangle
pub struct RectRounding {
    pub tl: f32,
    pub tr: f32,
    pub br: f32,
    pub bl: f32,
}

impl RectRounding {
    /// Creates a [`RectRounding`](crate::p5::RectRounding) object with the rounding for each
    /// corner. Returns an option for ergonomics with the `rect` method, is always guaranteed
    /// to be `Some`.
    pub fn new(tl: f32, tr: f32, br: f32, bl: f32) -> Option<Self> {
        Some(RectRounding { tl, tr, br, bl })
    }

    /// Creates a [`RectRounding`](crate::p5::RectRounding) object with the same rounding for each
    /// corner.
    pub fn equal(radius: f32) -> Option<Self> {
        Some(
            Self::new(radius, radius, radius, radius)
                .expect("RectRounding::new should never return None!"),
        )
    }
}
