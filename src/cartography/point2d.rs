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

    /// Gets the eight points surrounding the current point in two-dimensional space. Panics if
    /// integer underflow or overflow would occur (i.e. if one of the fields of self is equal to
    /// the minimum or maximum value of the type).
    pub fn get_surrounding_points(&self) -> Vec<Point2D> {
        vec![
            self.peek_shift(0, -1),
            self.peek_shift(1, -1),
            self.peek_shift(1, 0),
            self.peek_shift(1, 1),
            self.peek_shift(0, 1),
            self.peek_shift(-1, 1),
            self.peek_shift(-1, 0),
            self.peek_shift(-1, -1),
        ]
    }

    /// Gets the Manhattan distance between the two points.
    pub fn get_manhattan_distance(&self, other: &Point2D) -> u64 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }
}
