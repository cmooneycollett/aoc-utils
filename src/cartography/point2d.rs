/// Represents a single two-dimensional point in Euclidean space with integer co-ordinates.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point2D {
    x: i64,
    y: i64,
}

impl Point2D {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    /// Gets the value of the "x" field.
    pub fn x(&self) -> i64 {
        self.x
    }

    /// Sets the value of the "x" field.
    pub fn set_x(&mut self, x: i64) {
        self.x = x;
    }

    /// Gets the value of the "y" field.
    pub fn y(&self) -> i64 {
        self.y
    }

    /// Sets the value of the "y" field.
    pub fn set_y(&mut self, y: i64) {
        self.y = y;
    }

    /// Shifts the point by the given deltas in the x- and y-directions.
    pub fn shift(&mut self, dx: i64, dy: i64) {
        self.x += dx;
        self.y += dy;
    }

    /// Returns the Point2D that results by shifting the current point by the given deltas in the
    /// x- and y-directions.
    pub fn peek_shift(&self, dx: i64, dy: i64) -> Point2D {
        Point2D::new(self.x + dx, self.y + dy)
    }
}
