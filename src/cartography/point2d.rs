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

    /// Gets the value of the "y" field.
    pub fn y(&self) -> i64 {
        self.y
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

    /// Gets the eight points surrounding the current point in two-dimensional space.
    ///
    /// Panics if integer underflow or overflow would occur (i.e. if one of the fields of self is
    /// equal to the minimum or maximum value of the type).
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

    /// Gets the four points that are up, down, left and right from the current point in
    /// two-dimensional space.
    ///
    /// Panics if integer underflow or overflow would occur (i.e. if one of the fields of self is
    /// equal to the minimum or maximum value of the type).
    pub fn get_adjacent_points(&self) -> Vec<Point2D> {
        vec![
            self.peek_shift(0, -1),
            self.peek_shift(1, 0),
            self.peek_shift(0, 1),
            self.peek_shift(-1, 0),
        ]
    }

    /// Gets the Manhattan distance between the two points.
    pub fn get_manhattan_distance(&self, other: &Point2D) -> u64 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }

    /// Gets the Manhattan distance of the current point from the origin (0, 0, 0).
    pub fn get_manhattan_distance_origin(&self) -> u64 {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }

    /// Calculates the absolute value of the co-ordinates with respect to the origin (0, 0, 0).
    pub fn get_absolute_value(&self) -> f64 {
        (self.x.pow(2) as f64 + self.y.pow(2) as f64).sqrt()
    }

    /// Gets the absolute value of the vector leading from the current [`Point2D`] to the other
    /// [`Point2D`].
    pub fn get_absolute_value_from(&self, other: &Point2D) -> f64 {
        ((other.x() - self.x()).pow(2) as f64 + (other.y() - self.y()).pow(2) as f64).sqrt()
    }
}
