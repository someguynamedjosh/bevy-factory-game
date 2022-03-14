use super::*;

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

    /// Returns the direction which points 60 degrees clockwise from this
    /// one.
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

    /// Returns the direction which points 60 degrees counter-clockwise from
    /// this one.
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
