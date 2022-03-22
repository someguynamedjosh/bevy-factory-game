use bevy::prelude::Component;

use super::{Element, ReferenceItem};

#[derive(Clone, Component, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Item {
    elements: Vec<Element>,
}

impl Item {
    pub fn as_elements(&self) -> &[Element] {
        &self.elements[..]
    }

    pub fn into_elements(self) -> Vec<Element> {
        self.elements
    }

    pub fn with_modified_elements<O, M>(self, modifier: M) -> Self
    where
        O: Iterator<Item = Element>,
        M: Fn(std::vec::IntoIter<Element>) -> O,
    {
        modifier(self.elements.into_iter())
            .collect::<Vec<_>>()
            .into()
    }

    pub fn as_known_item(&self) -> Option<ReferenceItem> {
        for known in ReferenceItem::all() {
            if &known.as_item() == self {
                return Some(*known);
            }
        }
        None
    }

    /// Mass in Kilograms, equivalent to the sum of the masses of this item's elements.
    pub fn mass(&self) -> u32 {
        self.elements.iter().copied().map(Element::mass).sum()
    }

    /// Volume in Liters, equivalent to 2 plus the sum of the volumes of this item's elements.
    pub fn volume(&self) -> u32 {
        2 + self.elements.iter().copied().map(Element::volume).sum::<u32>()
    }
}

impl From<Vec<Element>> for Item {
    fn from(elements: Vec<Element>) -> Self {
        Self { elements }
    }
}
