use crate::prelude::*;

/// The distance from any vertex of a grid cell to its centroid.
pub const GRID_TRIANGLE_RADIUS: f32 = 62.0;

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

pub const CENTROID_TO_MEDMID_DISTANCE: f32 = GRID_MEDIAN_LENGTH / 2.0 - GRID_TRIANGLE_RADIUS;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsoAxis {
    A,
    B,
    C,
}

impl Default for IsoAxis {
    fn default() -> Self {
        Self::A
    }
}

impl IsoAxis {
    pub fn positive_direction(self) -> IsoDirection {
        match self {
            Self::A => IsoDirection::PosA,
            Self::B => IsoDirection::PosB,
            Self::C => IsoDirection::PosC,
        }
    }

    pub fn negative_direction(self) -> IsoDirection {
        match self {
            Self::A => IsoDirection::NegA,
            Self::B => IsoDirection::NegB,
            Self::C => IsoDirection::NegC,
        }
    }

    pub fn facing_angle(self) -> f32 {
        match self {
            Self::A => 0.0,
            Self::B => (PI * 2.0 / 3.0),
            Self::C => (-PI * 2.0 / 3.0),
        }
    }

    pub fn unit_vec(self) -> Vec2 {
        let angle = self.facing_angle();
        (angle.cos(), angle.sin()).into()
    }

    pub fn perp_unit_vec(self) -> Vec2 {
        self.unit_vec().perp()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsoDirection {
    PosA,
    NegC,
    PosB,
    NegA,
    PosC,
    NegB,
}

impl Default for IsoDirection {
    fn default() -> Self {
        Self::PosA
    }
}

impl IsoDirection {
    pub fn axis(self) -> IsoAxis {
        match self {
            Self::PosA | Self::NegA => IsoAxis::A,
            Self::PosB | Self::NegB => IsoAxis::B,
            Self::PosC | Self::NegC => IsoAxis::C,
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

    pub fn unit_vec(self) -> Vec2 {
        self.axis().unit_vec() * if self.is_positive() { 1.0 } else { -1.0 }
    }

    pub fn perp_unit_vec(self) -> Vec2 {
        self.unit_vec().perp()
    }

    /// Returns the direction which points 60 degrees clockwise from this one.
    pub fn clockwise(self) -> IsoDirection {
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
    pub fn counter_clockwise(self) -> IsoDirection {
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct IsoPos {
    x: i32,
    /// +y points towards the top of the screen.
    y: i32,
}

impl IsoPos {
    /// Returns the IsoPos that represents a grid cell containing the specified cartesian 
    /// coordinate.
    pub fn from_world_pos(pos: Vec2) -> Self {
        // I'm not going to bother adding comments to this because it would only become more
        // confusing. Consider debugging this function as an exercise for the reader.
        let x = (pos.x + GRID_MEDIAN_LENGTH - GRID_TRIANGLE_RADIUS) / GRID_MEDIAN_LENGTH;
        let odd_y_percent = x.rem_euclid(2.0);
        let odd_y_percent = if odd_y_percent > 1.0 {
            2.0 - odd_y_percent
        } else {
            odd_y_percent
        };
        let y = pos.y / (GRID_EDGE_LENGTH / 2.0);
        let pos_y_percent = (y + 1.0).rem_euclid(2.0) - 1.0;
        let even_y = (y / 2.0).round() as i32 * 2;
        let int_y = if pos_y_percent <= -odd_y_percent {
            even_y - 1
        } else if pos_y_percent >= odd_y_percent {
            even_y + 1
        } else {
            even_y
        };
        Self {
            x: x.floor() as i32,
            y: int_y,
        }
    }

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

    /// Returns true if the triangular grid cell this coordinate represents has a vertex that
    /// visually appears to point in the specified direction.
    pub fn has_vertex_pointing_in(self, dir: IsoDirection) -> bool {
        // points left and negative, or points right and positive.
        self.points_left() == dir.is_negative()
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
        // Offsetting along the B axis involves alternating between moving 1 step along Y and moving
        // -1 step along X, 2 steps along Y. Use the first pattern when moving up from the origin.

        // How much we should move along the first axis (first, third, fifth... step)
        // This is basically division rounding away from zero.
        let first_pattern_amount = offset / 2 + offset % 2;
        // How much we should move along the second axis (second, fourth, sixth... step)
        // Division rounding towards zero.
        let second_pattern_amount = offset / 2;

        // if we are pointing left XOR the offset is negative.
        if self.points_left() != (offset < 0) {
            // Move along the y axis first, then the x axis.
            Self {
                x: self.x - second_pattern_amount,
                y: self.y + first_pattern_amount + 2 * second_pattern_amount,
            }
        } else {
            // Move along the x pattern first, then the y pattern.
            Self {
                x: self.x - first_pattern_amount,
                y: self.y + second_pattern_amount + 2 * first_pattern_amount,
            }
        }
    }

    /// Move the coordinate along the C axis (+ points bottom-left.)
    pub fn offset_c(self, offset: i32) -> Self {
        // Same thing as the B algorithm except mirrored effect on Y axis.

        let first_pattern_amount = offset / 2 + offset % 2;
        let second_pattern_amount = offset / 2;

        if self.points_left() != (offset < 0) {
            Self {
                x: self.x - second_pattern_amount,
                y: self.y - first_pattern_amount - 2 * second_pattern_amount,
            }
        } else {
            Self {
                x: self.x - first_pattern_amount,
                y: self.y - second_pattern_amount - 2 * first_pattern_amount,
            }
        }
    }

    /// Move the coordinate along an arbitrary axis.
    pub fn offset_axis(self, axis: IsoAxis, offset: i32) -> Self {
        match axis {
            IsoAxis::A => self.offset_a(offset),
            IsoAxis::B => self.offset_b(offset),
            IsoAxis::C => self.offset_c(offset),
        }
    }

    /// Move the coordinate in an arbitrary direction.
    pub fn offset_direction(self, direction: IsoDirection, offset: i32) -> Self {
        self.offset_axis(
            direction.axis(),
            if direction.is_positive() {
                offset
            } else {
                -offset
            },
        )
    }

    /// Move the coordinate along the axis perpendicular to +A.
    pub const fn offset_perp_a(self, offset: i32) -> Self {
        // Just like the offset_a algorithm, but with x and y swapped.
        Self {
            x: self.x,
            y: self.y + offset,
        }
    }

    /// Move the coordinate along the axis perpendicular to +B.
    pub fn offset_perp_b(self, offset: i32) -> Self {
        // Like the offset_b algorithm, but different. (I just fiddled until it worked.)
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

    /// Move the coordinate along the axis perpendicular to +C.
    pub fn offset_perp_c(self, offset: i32) -> Self {
        // Like the offset_b algorithm, but different. (I just fiddled until it worked.)
        let first_axis_amount = offset / 2 + offset % 2;
        let second_axis_amount = offset / 2;

        if self.points_left() != (offset < 0) {
            Self {
                x: self.x + first_axis_amount,
                y: self.y - second_axis_amount,
            }
        } else {
            Self {
                x: self.x + second_axis_amount,
                y: self.y - first_axis_amount,
            }
        }
    }

    /// Move the coordinate along an arbitrary perpendicular axis.
    pub fn offset_perp_axis(self, perp_axis: IsoAxis, offset: i32) -> Self {
        match perp_axis {
            IsoAxis::A => self.offset_perp_a(offset),
            IsoAxis::B => self.offset_perp_b(offset),
            IsoAxis::C => self.offset_perp_c(offset),
        }
    }

    /// Move the coordinate in an arbitrary perpendicular direction.
    pub fn offset_perp_direction(self, perp_direction: IsoDirection, offset: i32) -> Self {
        self.offset_perp_axis(
            perp_direction.axis(),
            if perp_direction.is_positive() {
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

    /// Returns a coordinate which bisects the median perpendicular to the given axis, with the
    /// result being that points returned by this function line up when the underlying IsoPos
    /// instances are also on an axis. This behavior does not occur with centroid_pos(), it appears
    /// jagged. This function is useful for animating things traveling on rails.
    pub fn axis_aligned_pos(self, axis: IsoAxis) -> Vec2 {
        let offset = CENTROID_TO_MEDMID_DISTANCE * if self.points_left() { 1.0 } else { -1.0 };
        self.centroid_pos() + axis.unit_vec() * offset
    }

    /// Returns a transform which places a building at centroid_pos with the correct rotation.
    pub fn building_transform(self, facing: IsoAxis) -> Transform {
        let mut t = Transform::identity();
        t.translation = (self.centroid_pos(), 0.0).into();
        let pointing_angle = if self.points_left() { 0.0 } else { PI };
        t.rotation = Quat::from_rotation_z(facing.facing_angle() + pointing_angle);
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
        assert!(IsoPos::new(0, 0).points_left());
        assert!(IsoPos::new(0, 1).points_right());
        assert!(IsoPos::new(1, 0).points_right());
        assert!(IsoPos::new(-1, 0).points_right());
        assert!(IsoPos::new(-2, 0).points_left());
        for i in -10..10 {
            assert!(IsoPos::new(i, i).points_left());
        }
    }

    #[test]
    fn axis_offsets() {
        assert_eq!(IsoPos::new(0, 0).offset_a(1), IsoPos::new(1, 0));
        assert_eq!(IsoPos::new(0, 0).offset_a(-2), IsoPos::new(-2, 0));

        assert_eq!(IsoPos::new(0, 0).offset_b(4), IsoPos::new(-2, 6));
        assert_eq!(IsoPos::new(0, 0).offset_b(5), IsoPos::new(-2, 7));
        assert_eq!(IsoPos::new(-1, 0).offset_b(5), IsoPos::new(-4, 8));
        assert_eq!(IsoPos::new(1, 0).offset_b(-2), IsoPos::new(2, -3));

        assert_eq!(IsoPos::new(0, 0).offset_c(4), IsoPos::new(-2, -6));
        assert_eq!(IsoPos::new(0, 0).offset_c(5), IsoPos::new(-2, -7));
        assert_eq!(IsoPos::new(-1, 0).offset_c(5), IsoPos::new(-4, -8));
        assert_eq!(IsoPos::new(1, 0).offset_c(-2), IsoPos::new(2, 3));

        let mut direction = IsoDirection::PosA;
        let mut pos = IsoPos::origin();
        // Move an equal number of steps in each of the 6 directions, should take us back to the
        // place we started from.
        for _ in 0..6 {
            pos = pos.offset_direction(direction, 12);
            direction = direction.clockwise();
        }
        assert_eq!(direction, IsoDirection::PosA);
        assert_eq!(pos, IsoPos::origin());
    }
}
