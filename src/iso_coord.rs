use crate::prelude::*;
use scones::make_constructor;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    A,
    B,
    C,
}

/// The distance from any vertex of a grid cell to its centroid.
pub const GRID_TRIANGLE_RADIUS: f32 = 74.0;

// cos 30deg = (edge / 2) / radius
// cos 30deg = 0.8660254
/// How long each edge of a grid cell is.
pub const GRID_EDGE_LENGTH: f32 = 2.0 * GRID_TRIANGLE_RADIUS * 0.8660254;

// median^2 + (edge / 2)^2 = edge^2
// median^2 = edge^2 - edge^2 * 0.25
// median = sqrt(edge^2 * 0.75)
// median = edge * sqrt(0.75)
// sqrt(0.75) = 0.8660254, huh.
/// The length of the median running between the midpoint of an edge and the opposite vertex.
pub const GRID_MEDIAN_LENGTH: f32 = GRID_EDGE_LENGTH * 0.8660254;

/// The difference in x coordinate between the centroids of two cells stacked on top of each other.
pub const OPPOSING_DISTANCE: f32 = GRID_TRIANGLE_RADIUS * 2.0 - GRID_MEDIAN_LENGTH;

impl Axis {
    pub fn positive_direction(self) -> Direction {
        match self {
            Self::A => Direction::PosA,
            Self::B => Direction::PosB,
            Self::C => Direction::PosC,
        }
    }

    pub fn negative_direction(self) -> Direction {
        match self {
            Self::A => Direction::NegA,
            Self::B => Direction::NegB,
            Self::C => Direction::NegC,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    PosA,
    NegC,
    PosB,
    NegA,
    PosC,
    NegB,
}

impl Direction {
    pub fn axis(self) -> Axis {
        match self {
            Self::PosA | Self::NegA => Axis::A,
            Self::PosB | Self::NegB => Axis::B,
            Self::PosC | Self::NegC => Axis::C,
        }
    }

    pub fn is_positive(self) -> bool {
        match self {
            Self::PosA | Self::PosB | Self::PosC => true,
            Self::NegA | Self::NegB | Self::NegC => false,
        }
    }

    pub fn is_negative(self) -> bool {
        !self.is_positive()
    }

    /// Returns the direction which points 60 degrees clockwise from this one.
    pub fn clockwise(self) -> Direction {
        match self {
            Self::PosA => Self::NegB,
            Self::NegC => Self::PosA,
            Self::PosB => Self::NegC,
            Self::NegA => Self::PosB,
            Self::PosC => Self::NegA,
            Self::NegB => Self::PosC,
        }
    }

    /// Returns the direction which points 60 degrees counter-clockwise from this one.
    pub fn counter_clockwise(self) -> Direction {
        match self {
            Self::PosA => Self::NegC,
            Self::NegC => Self::PosB,
            Self::PosB => Self::NegA,
            Self::NegA => Self::PosC,
            Self::PosC => Self::NegB,
            Self::NegB => Self::PosA,
        }
    }
}

/// Defines a coordinate on a grid of equilateral triangles. The origin at 0, 0 is a triangle which
/// appears to point left.
///
/// The grid has three axes: A, B, and C. +A points to the right, +B points up to the left, and +C
/// points down to the left. We only need two coordinates to uniquely describe a position, so we
/// store x, y coordinates instead of A, B, C.
#[make_constructor]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IsoCoord {
    x: i32,
    /// +y points towards the top of the screen.
    y: i32,
}

impl IsoCoord {
    /// Equivalent to new(0, 0)
    pub fn origin() -> Self {
        Self::new(0, 0)
    }

    /// Returns true if the triangle at the grid position this coordinate represents appears similar
    /// to a triangle pointing left.
    pub fn points_left(self) -> bool {
        // rem_euclid is like modulus that does not reflect around (0, 0).
        (self.x + self.y).rem_euclid(2) == 0
    }

    /// The opposite of points_left.
    pub fn points_right(self) -> bool {
        !self.points_left()
    }

    /// Move the coordinate left or right (+A points to the right.)
    pub const fn offset_a(self, offset: i32) -> Self {
        Self {
            x: self.x + offset,
            y: self.y,
        }
    }

    /// Move the coordinate along the B axis (+ points top-left.)
    pub fn offset_b(self, offset: i32) -> Self {
        // Offsetting along the B axis involves alternating steps up the x and y axes.
        // On a grid cell which points left, the next step is to increase the x coord, followed by
        // the y coord.

        // How much we should move along the first axis (first, third, fifth... step)
        // This is basically division rounding away from zero.
        let first_axis_amount = offset / 2 + offset % 2;
        // How much we should move along the second axis (second, fourth, sixth... step)
        // Division rounding towards zero.
        let second_axis_amount = offset / 2;

        // if we are pointing left XOR the offset is negative.
        if self.points_left() != (offset < 0) {
            // Move along the y axis first, then the x axis.
            Self {
                x: self.x - second_axis_amount,
                y: self.y + first_axis_amount,
            }
        } else {
            // Move along the x axis first, then the y axis.
            Self {
                x: self.x - first_axis_amount,
                y: self.y + second_axis_amount,
            }
        }
    }

    /// Move the coordinate along the C axis (+ points bottom-left.)
    pub fn offset_c(self, offset: i32) -> Self {
        // Same thing as the B algorithm except mirrored effect on Y axis.

        let first_axis_amount = offset / 2 + offset % 2;
        let second_axis_amount = offset / 2;

        if self.points_left() != (offset < 0) {
            Self {
                x: self.x - second_axis_amount,
                y: self.y - first_axis_amount,
            }
        } else {
            Self {
                x: self.x - first_axis_amount,
                y: self.y - second_axis_amount,
            }
        }
    }

    /// Move the coordinate along an arbitrary axis.
    pub fn offset_axis(self, axis: Axis, offset: i32) -> Self {
        match axis {
            Axis::A => self.offset_a(offset),
            Axis::B => self.offset_b(offset),
            Axis::C => self.offset_c(offset),
        }
    }

    /// Move the coordinate in an arbitrary direction.
    pub fn offset_direction(self, direction: Direction, offset: i32) -> Self {
        self.offset_axis(
            direction.axis(),
            if direction.is_positive() {
                offset
            } else {
                -offset
            },
        )
    }

    /// Gets the position of the centroid of the grid cell this coordinate refers to. (The centroid
    /// is the point with equal distance to all vertices of the triangular grid cell.)
    pub fn centroid_pos(self) -> Vec2 {
        Vec2::new(
            self.x as f32 * GRID_MEDIAN_LENGTH
                + if self.points_left() {
                    OPPOSING_DISTANCE
                } else {
                    0.0
                },
            self.y as f32 * GRID_EDGE_LENGTH * 0.5,
        )
    }

    /// Returns a transform which places a building at centroid_pos with the correct rotation.
    pub fn building_transform(self, facing: Axis) -> Transform {
        let mut t = Transform::identity();
        t.translation = (self.centroid_pos(), 0.0).into();
        let facing_angle = match facing {
            Axis::A => 0.0,
            Axis::B => PI * 2.0 / 3.0,
            Axis::C => -PI * 2.0 / 3.0,
        };
        let pointing_angle = if self.points_left() { 0.0 } else { PI };
        t.rotation = Quat::from_rotation_z(facing_angle + pointing_angle);
        t
    }

    #[cfg(test)]
    pub const fn as_cartesian(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pointiness() {
        assert!(IsoCoord::new(0, 0).points_left());
        assert!(IsoCoord::new(0, 1).points_right());
        assert!(IsoCoord::new(1, 0).points_right());
        assert!(IsoCoord::new(-1, 0).points_right());
        assert!(IsoCoord::new(-2, 0).points_left());
        for i in -10..10 {
            assert!(IsoCoord::new(i, i).points_left());
        }
    }

    #[test]
    fn axis_offsets() {
        assert_eq!(IsoCoord::new(0, 0).offset_a(1), IsoCoord::new(1, 0));
        assert_eq!(IsoCoord::new(0, 0).offset_a(-2), IsoCoord::new(-2, 0));

        assert_eq!(IsoCoord::new(0, 0).offset_b(4), IsoCoord::new(-2, 2));
        assert_eq!(IsoCoord::new(0, 0).offset_b(5), IsoCoord::new(-2, 3));
        assert_eq!(IsoCoord::new(-1, 0).offset_b(5), IsoCoord::new(-4, 2));
        assert_eq!(IsoCoord::new(1, 0).offset_b(-2), IsoCoord::new(2, -1));

        assert_eq!(IsoCoord::new(0, 0).offset_c(4), IsoCoord::new(-2, -2));
        assert_eq!(IsoCoord::new(0, 0).offset_c(5), IsoCoord::new(-2, -3));
        assert_eq!(IsoCoord::new(-1, 0).offset_c(5), IsoCoord::new(-4, -2));
        assert_eq!(IsoCoord::new(1, 0).offset_c(-2), IsoCoord::new(2, 1));

        let mut direction = Direction::PosA;
        let mut pos = IsoCoord::origin();
        // Move an equal number of steps in each of the 6 directions, should take us back to the
        // place we started from.
        for _ in 0..6 {
            pos = pos.offset_direction(direction, 12);
            direction = direction.clockwise();
        }
        assert_eq!(direction, Direction::PosA);
        assert_eq!(pos, IsoCoord::origin());
    }
}
