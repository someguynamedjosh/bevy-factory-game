use super::{Element, Item};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceItem {
    IronOre,
    IronNugget,
}

impl ReferenceItem {
    pub fn all() -> &'static [Self] {
        &[Self::IronOre, Self::IronNugget]
    }

    pub fn as_item(&self) -> Item {
        match self {
            Self::IronOre => {
                vec![Element::Impurity, Element::Ferrite, Element::Impurity].into()
            }
            Self::IronNugget => vec![Element::Ferrite, Element::Ferrite].into(),
        }
    }
}
