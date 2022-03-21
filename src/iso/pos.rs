use bevy::prelude::Component;

use super::*;

/// Defines a coordinate on a grid of equilateral triangles. The origin at
/// 0, 0 is a triangle which appears to point left.
///
/// The grid has three axes: A, B, and C. +A points to the right, +B points
/// up to the left, and +C points down to the left. We only need two
/// coordinates to uniquely describe a position, so we store x, y
/// coordinates instead of A, B, C.
#[make_constructor]
#[derive(Clone, Copy, Component, Debug, Default, PartialEq, Eq, Hash)]
pub struct IsoPos {
    x: i32,
    /// +y points towards the top of the screen.
    y: i32,
}

impl IsoPos {
    /// Returns the IsoPos that represents a grid cell containing the
    /// specified cartesian coordinate.
    pub fn from_world_pos(pos: Vec2, snapping: Snapping) -> Self {
        if let Snapping::AlongLine { through, axis } = snapping {
            let centroid = through.centroid_pos();
            let axis_unit = axis.unit_vec();
            let distance_along_axis = (pos - centroid).dot(axis_unit);
            return Self::from_world_pos(
                centroid + axis_unit * distance_along_axis + Vec2::new(0.0, 0.01),
                Snapping::None,
            );
        } else if let Snapping::AlongAnyLine { through } = snapping {
            let closest_a = Self::from_world_pos(
                pos,
                Snapping::AlongLine {
                    through,
                    axis: IsoAxis::A,
                },
            );
            let closest_b = Self::from_world_pos(
                pos,
                Snapping::AlongLine {
                    through,
                    axis: IsoAxis::B,
                },
            );
            let closest_c = Self::from_world_pos(
                pos,
                Snapping::AlongLine {
                    through,
                    axis: IsoAxis::C,
                },
            );
            let dist_a = closest_a.centroid_pos().distance_squared(pos);
            let dist_b = closest_b.centroid_pos().distance_squared(pos);
            let dist_c = closest_c.centroid_pos().distance_squared(pos);
            let least = dist_a.min(dist_b.min(dist_c));
            if dist_a <= least {
                return closest_a;
            } else if dist_b <= least {
                return closest_b;
            } else {
                assert!(dist_c <= least);
                return closest_c;
            }
        }
        // I'm not going to bother adding comments to this because it would only become
        // more confusing. Consider debugging this function as an exercise for
        // the reader.
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
        let without_snapping = Self {
            x: x.floor() as i32,
            y: int_y,
        };
        match snapping {
            Snapping::None => without_snapping,
            Snapping::Points(dir) => {
                if without_snapping.points_in(dir) {
                    without_snapping
                } else {
                    let delta = if without_snapping.points_right() {
                        -1
                    } else {
                        1
                    };
                    let a = without_snapping.offset_a(delta);
                    let da = a.centroid_pos().distance(pos);
                    let b = without_snapping.offset_b(delta);
                    let db = b.centroid_pos().distance(pos);
                    let c = without_snapping.offset_c(delta);
                    let dc = c.centroid_pos().distance(pos);
                    if da < db && da < dc {
                        a
                    } else if db < dc && db < da {
                        b
                    } else {
                        c
                    }
                }
            }
            Snapping::AlongLine { .. } | Snapping::AlongAnyLine { .. } => unreachable!(),
        }
    }

    /// Equivalent to new(0, 0)
    pub fn origin() -> Self {
        Self::new(0, 0)
    }

    /// Returns true if the triangle at the grid position this coordinate
    /// represents appears similar to a triangle pointing left.
    pub fn points_left(self) -> bool {
        // rem_euclid is like modulus that does not reflect around (0, 0).
        (self.x + self.y).rem_euclid(2) == 0
    }

    /// The opposite of points_left.
    pub fn points_right(self) -> bool {
        !self.points_left()
    }

    pub fn points_in(self, dir: PointsDirection) -> bool {
        self.points_left() == dir.is_left()
    }

    /// Returns true if the triangular grid cell this coordinate represents
    /// has a vertex that visually appears to point in the specified
    /// direction.
    pub fn has_vertex_pointing_in(self, dir: IsoDirection) -> bool {
        // points left and negative, or points right and positive.
        self.points_left() == dir.is_negative()
    }

    /// Returns the 3 bordering positions of this position.
    pub fn surroundings(self) -> Vec<Self> {
        let delta = if self.points_left() { 1 } else { -1 };
        vec![
            self.offset_a(delta),
            self.offset_b(delta),
            self.offset_c(delta),
        ]
    }

    /// Move the coordinate left or right (+A points to the right.)
    pub const fn offset_a(self, offset: i32) -> Self {
        Self {
            y: self.y + offset,
            x: self.x,
        }
    }

    /// Move the coordinate along the B axis (+ points top-left.)
    pub fn offset_b(self, offset: i32) -> Self {
        // Offsetting along the B axis involves alternating between moving -1 step along
        // Y and moving -1 step along X. Use the first pattern when moving up
        // from the origin.

        // How much we should move along the first axis (first, third, fifth... step)
        // This is basically division rounding away from zero.
        let first_pattern_amount = offset / 2 + offset % 2;
        // How much we should move along the second axis (second, fourth, sixth... step)
        // Division rounding towards zero.
        let second_pattern_amount = offset / 2;

        // if we are pointing left XOR the offset is negative.
        if self.points_left() != (offset < 0) {
            // Move along the y pattern first, then the x pattern.
            Self {
                x: self.x - second_pattern_amount,
                y: self.y - first_pattern_amount,
            }
        } else {
            // Move along the x axis first, then the y axis.
            Self {
                x: self.x - first_pattern_amount,
                y: self.y - second_pattern_amount,
            }
        }
    }

    /// Move the coordinate along the C axis (+ points bottom-left.)
    pub fn offset_c(self, offset: i32) -> Self {
        // Same thing as the B algorithm except mirrored effect on X axis and flipped
        // order.

        let first_pattern_amount = offset / 2 + offset % 2;
        let second_pattern_amount = offset / 2;

        if self.points_left() != (offset < 0) {
            Self {
                x: self.x + first_pattern_amount,
                y: self.y - second_pattern_amount,
            }
        } else {
            Self {
                x: self.x + second_pattern_amount,
                y: self.y - first_pattern_amount,
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
        // Just like the offset_a algorithm, but rotated 90 degrees.
        Self {
            x: self.x - offset,
            y: self.y,
        }
    }

    /// Move the coordinate along the axis perpendicular to +B.
    pub fn offset_perp_b(self, offset: i32) -> Self {
        // Like the offset_b algorithm, but different. (I just fiddled until it worked.)
        let first_axis_amount = offset / 2 + offset % 2;
        let second_axis_amount = offset / 2;

        if self.points_left() != (offset < 0) {
            Self {
                x: self.x + first_axis_amount,
                y: self.y - second_axis_amount - 2 * first_axis_amount,
            }
        } else {
            Self {
                x: self.x + second_axis_amount,
                y: self.y - first_axis_amount - 2 * second_axis_amount,
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
                y: self.y + second_axis_amount + 2 * first_axis_amount,
            }
        } else {
            Self {
                x: self.x + second_axis_amount,
                y: self.y + first_axis_amount + 2 * second_axis_amount,
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

    /// Simultaneously move the coordinate both parallel to and
    /// perpendicular to the provided direction.
    pub fn offset_both_direction(
        self,
        direction: IsoDirection,
        parallel_offset: i32,
        perpendicular_offset: i32,
    ) -> Self {
        self.offset_direction(direction, parallel_offset)
            .offset_perp_direction(direction, perpendicular_offset)
    }

    /// Gets the position of the centroid of the grid cell this coordinate
    /// refers to. (The centroid is the point with equal distance to all
    /// vertices of the triangular grid cell.)
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

    /// Returns a coordinate which bisects the median parallel to the given
    /// axis, with the result being that points returned by this function
    /// line up when the underlying IsoPos instances are also on an axis.
    /// This behavior does not occur with centroid_pos(), it appears jagged.
    /// This function is useful for animating things traveling on rails.
    pub fn axis_aligned_pos(self, axis: IsoAxis) -> Vec2 {
        let offset = CENTROID_TO_MEDMID_DISTANCE * if self.points_left() { -1.0 } else { 1.0 };
        self.centroid_pos() + axis.unit_vec().perp() * offset
    }

    /// Returns a transform which places a building at centroid_pos with the
    /// correct rotation.
    pub fn building_transform(self, facing: IsoAxis) -> Transform {
        let mut t = Transform::identity();
        t.translation = (self.centroid_pos(), 0.0).into();
        let pointing_angle = if self.points_left() { 0.0 } else { TAU * 0.5 };
        t.rotation = Quat::from_rotation_z(facing.facing_angle() - TAU * 0.25 + pointing_angle);
        t
    }

    #[cfg(test)]
    pub const fn as_cartesian(self) -> (i32, i32) {
        (self.x, self.y)
    }
}
