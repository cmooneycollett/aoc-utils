/// Represents a single point location in three-dimensional Euclidean space with integer
/// co-ordinates (i64).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3D {
    /// Creates a new Point3D.
    pub fn new(x: i64, y: i64, z: i64) -> Point3D {
        Self { x, y, z }
    }

    /// Gets the value of the "x" field.
    pub fn x(&self) -> i64 {
        self.x
    }

    /// Gets the value of the "y" field.
    pub fn y(&self) -> i64 {
        self.y
    }

    /// Gets the value of the "z" field.
    pub fn z(&self) -> i64 {
        self.z
    }

    /// Shifts the point by the given deltas.
    pub fn shift(&mut self, dx: i64, dy: i64, dz: i64) {
        self.x += dx;
        self.y += dy;
        self.z += dz;
    }

    /// Returns the Point3D that would result by shifting the current point by the given deltas.
    pub fn peek_shift(&self, dx: i64, dy: i64, dz: i64) -> Point3D {
        Point3D::new(self.x + dx, self.y + dy, self.z + dz)
    }

    /// Gets the points surroudning the current point in three-dimensional space.
    ///
    /// Panics if integer underflow or overflow would occur (i.e., if one of the fields of the
    /// current point is equal to the minimum or maximum value of i64).
    pub fn get_surrounding_points(&self) -> Vec<Point3D> {
        vec![
            // Bottom layer
            self.peek_shift(0, -1, -1),
            self.peek_shift(1, -1, -1),
            self.peek_shift(1, 0, -1),
            self.peek_shift(1, 1, -1),
            self.peek_shift(0, 1, -1),
            self.peek_shift(-1, 1, -1),
            self.peek_shift(-1, 0, -1),
            self.peek_shift(-1, -1, -1),
            self.peek_shift(0, 0, -1),
            // Middle layer
            self.peek_shift(0, -1, 0),
            self.peek_shift(1, -1, 0),
            self.peek_shift(1, 0, 0),
            self.peek_shift(1, 1, 0),
            self.peek_shift(0, 1, 0),
            self.peek_shift(-1, 1, 0),
            self.peek_shift(-1, 0, 0),
            self.peek_shift(-1, -1, 0),
            // Top layer
            self.peek_shift(0, -1, 1),
            self.peek_shift(1, -1, 1),
            self.peek_shift(1, 0, 1),
            self.peek_shift(1, 1, 1),
            self.peek_shift(0, 1, 1),
            self.peek_shift(-1, 1, 1),
            self.peek_shift(-1, 0, 1),
            self.peek_shift(-1, -1, 1),
            self.peek_shift(0, 0, 1),
        ]
    }

    /// Gets the points that are adjacent to the current point in three-dimensional space. Excludes
    /// the diagonal points that would be included in the surrounding points.
    ///
    /// Panics if integer underflow or overflow would occur (i.e., if one of the fields of the
    /// current point is equal to the minimum or maximum value of i64).
    pub fn get_adjacent_points(&self) -> Vec<Point3D> {
        vec![
            self.peek_shift(0, -1, 0),
            self.peek_shift(1, 0, 0),
            self.peek_shift(0, 1, 0),
            self.peek_shift(-1, 0, 0),
            self.peek_shift(0, 0, -1),
            self.peek_shift(0, 0, 1),
        ]
    }

    /// Gets the Manhattan distance between the current point and another [`Point3D`].
    pub fn get_manhattan_distance(&self, other: &Point3D) -> u64 {
        (self.x - other.x).unsigned_abs()
            + (self.y - other.y).unsigned_abs()
            + (self.z - other.z).unsigned_abs()
    }

    /// Gets the Manhattan distance of the current point from the origin (0, 0, 0).
    pub fn get_manhattan_distance_origin(&self) -> u64 {
        self.x.unsigned_abs() + self.y.unsigned_abs() + self.z.unsigned_abs()
    }

    /// Calculates the absolute value of the co-ordinates with respect to the origin (0, 0, 0).
    pub fn get_absolute_value(&self) -> f64 {
        (self.x.pow(2) as f64 + self.y.pow(2) as f64 + self.z.pow(2) as f64).sqrt()
    }

    /// Gets the absolute value of the vector leading from the current [`Point3D`] to the other
    /// [`Point3D`].
    pub fn get_absolute_value_from(&self, other: &Point3D) -> f64 {
        ((other.x - self.x).pow(2) as f64
            + (other.y - self.y).pow(2) as f64
            + (other.z - self.z).pow(2) as f64)
            .sqrt()
    }
}
