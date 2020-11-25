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

    /// Multiplies the current matrix by the one specified through the parameters. This is a
    /// powerful operation that can perform the equivalent of translate, scale, shear and rotate
    /// all at once. You can learn more about transformation matrices on
    /// [Wikipedia](https://en.wikipedia.org/wiki/Transformation_matrix).
    ///
    /// The naming here corresponds to the [`Transform2D::new`](euclid::Transform2D::new) function,
    /// but should be the same as the WHATWG specification used by p5.js.
    ///
    /// Additionally, note that the matrix is reset each frame. This is normally more natural since
    /// you don't want transformations to add up over frames, since otherwise, the drawings would
    /// rapidly fly off the screen.
    fn apply_matrix(&mut self, m11: f32, m12: f32, m21: f32, m22: f32, m31: f32, m32: f32);

    /// Specifies an amount to displace objects within the display window. The x parameter specifies
    /// left/right translation, the y parameter specifies up/down translation.

    /// Transformations are cumulative and apply to everything that happens after and subsequent calls
    /// to the function accumulates the effect. For example, calling translate(50, 0) and then
    /// translate(20, 0) is the same as translate(70, 0). If translate() is called within draw(), the
    /// transformation is reset when the loop begins again. This function can be further controlled by
    /// using push() and pop().
    fn translate(&mut self, x: f32, y: f32) {
        self.apply_matrix(1., 0., 0., 1., x, y);
    }

    /// Rotates a shape by the amount specified by the angle parameter.
    ///
    /// Objects are always rotated around their relative position to the origin and positive numbers
    /// rotate objects in a clockwise direction. Transformations apply to everything that happens
    /// after and subsequent calls to the function accumulates the effect. For example, calling
    /// rotate(HALF_PI) and then rotate(HALF_PI) is the same as rotate(PI). All tranformations are
    /// reset when draw() begins again.
    fn rotate(&mut self, angle: f32) {
        // TODO: Angle mode
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        self.apply_matrix(cos_a, sin_a, -sin_a, cos_a, 0., 0.);
    }

    /// Increases or decreases the size of a shape by expanding or contracting vertices. Objects
    /// always scale from their relative origin to the coordinate system. Scale values are
    /// specified as decimal percentages. For example, the function call scale(2.0) increases the
    /// dimension of a shape by 200%.
    ///
    /// Transformations apply to everything that happens after and subsequent calls to the function
    /// multiply the effect. For example, calling scale(2.0) and then scale(1.5) is the same as
    /// scale(3.0). If scale() is called within draw(), the transformation is reset when the loop
    /// begins again.
    fn scale(&mut self, scale: f32) {
        self.apply_matrix(scale, 0., 0., scale, 0., 0.);
    }

    /// Shears a shape around the x-axis by the amount specified by the angle parameter. Angles
    /// should be specified in the current angleMode. Objects are always sheared around their
    /// relative position to the origin and positive numbers shear objects in a clockwise
    /// direction.
    ///
    /// Transformations apply to everything that happens after and subsequent calls to
    /// the function accumulates the effect. For example, calling shearX(PI/2) and then
    /// shearX(PI/2) is the same as shearX(PI). If shearX() is called within the draw(), the
    /// transformation is reset when the loop begins again.
    fn shear_x(&mut self, angle: f32) {
        let mut t = angle.tan();
        if t.abs() > 1000. {
            t = 0.; // awful hack, but otherwise, raqote overflows when rendering.
        }
        self.apply_matrix(1., 0., t, 1., 0., 0.);
    }

    /// Shears a shape around the y-axis the amount specified by the angle parameter. Angles should
    /// be specified in the current angleMode. Objects are always sheared around their relative
    /// position to the origin and positive numbers shear objects in a clockwise direction.
    ///
    /// Transformations apply to everything that happens after and subsequent calls to the function
    /// accumulates the effect. For example, calling shearY(PI/2) and then shearY(PI/2) is the same
    /// as shearY(PI). If shearY() is called within the draw(), the transformation is reset when
    /// the loop begins again.
    fn shear_y(&mut self, angle: f32) {
        let mut t = angle.tan();
        if t.abs() > 1_000. {
            t = 0.;
        }
        self.apply_matrix(1., t, 0., 1., 0., 0.);
    }

    /// Specifies the number of frames to be displayed every second. For example, the function call
    /// frame_rate(30) will attempt to refresh 30 times a second. If the processor is not fast
    /// enough to maintain the specified rate, the frame rate will not be achieved. The default
    /// frame rate is 60 fps.
    ///
    /// TODO: detect frame rate based on monitor refresh rate like p5.js
    /// TODO: allow for updating frame rate outside of `setup`.
    fn frame_rate(&mut self, fps: f32);

    /// colorMode() changes the way p5.js interprets color data. By default, the parameters for
    /// fill(), stroke(), background(), and color() are defined by values between 0 and 255
    /// using the RGB color model.
    fn color_mode(&mut self, mode: crate::ColorMode);

    fn fill<C: IntoColor>(&mut self, color: C);

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
