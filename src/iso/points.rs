use super::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PointsDirection {
    Left,
    Right,
}

impl PointsDirection {
    pub fn pick(left: bool) -> Self {
        if left {
            Self::Left
        } else {
            Self::Right
        }
    }

    pub fn is_left(&self) -> bool {
        *self == Self::Left
    }
}
