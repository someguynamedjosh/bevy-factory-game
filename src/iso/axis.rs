use super::*;

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
            Self::A => TAU * 0.25,
            Self::B => TAU * (0.25 + 1.0 / 3.0),
            Self::C => TAU * (0.25 - 1.0 / 3.0),
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
