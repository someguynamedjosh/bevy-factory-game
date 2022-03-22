#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Element {
    Animus,
    Ferrous,
    Impurity,
}

impl Element {
    /// Mass in Kilograms.
    pub fn mass(self) -> u32 {
        match self {
            Element::Animus => 4,
            Element::Ferrous => 8,
            Element::Impurity => 6,
        }
    }

    /// Volume in Liters.
    pub fn volume(self) -> u32 {
        match self {
            Element::Animus => 1,
            Element::Ferrous => 1,
            Element::Impurity => 2,
        }
    }
}
