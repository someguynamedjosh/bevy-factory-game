use super::{Element, Item};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ReferenceItem {
    Magnetite,
    PureFerrous,
    IronLump,
    Animite,
    PureAnimus,
}

impl ReferenceItem {
    pub fn all() -> &'static [Self] {
        &[
            Self::Magnetite,
            Self::PureFerrous,
            Self::IronLump,
            Self::Animite,
            Self::PureAnimus,
        ]
    }

    pub fn as_item(&self) -> Item {
        match self {
            Self::Magnetite => vec![Element::Impurity, Element::Ferrous, Element::Impurity].into(),
            Self::PureFerrous => vec![Element::Ferrous].into(),
            Self::IronLump => vec![Element::Ferrous, Element::Ferrous].into(),
            Self::Animite => vec![Element::Impurity, Element::Animus, Element::Impurity].into(),
            Self::PureAnimus => vec![Element::Animus].into(),
        }
    }
}
