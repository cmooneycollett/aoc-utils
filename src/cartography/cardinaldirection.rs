/// Represents the four compass cardinal directions.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl CardinalDirection {
    /// Determines the cardinal direction resulting from rotating clockwise by 90 degrees the
    /// specified number of times.
    pub fn rotate90_clockwise(&self, n: u64) -> CardinalDirection {
        let mut new_direction = *self;
        for _ in 0..n {
            new_direction = match new_direction {
                CardinalDirection::North => CardinalDirection::East,
                CardinalDirection::East => CardinalDirection::South,
                CardinalDirection::South => CardinalDirection::West,
                CardinalDirection::West => CardinalDirection::North,
            }
        }
        new_direction
    }

    /// Determines the cardinal direction resulting from rotating counter-clockwise by 90 degrees
    /// the specified number of times.
    pub fn rotate90_counterclockwise(&self, n: u64) -> CardinalDirection {
        let mut new_direction = *self;
        for _ in 0..n {
            new_direction = match new_direction {
                CardinalDirection::North => CardinalDirection::West,
                CardinalDirection::East => CardinalDirection::North,
                CardinalDirection::South => CardinalDirection::East,
                CardinalDirection::West => CardinalDirection::South,
            }
        }
        new_direction
    }
}
