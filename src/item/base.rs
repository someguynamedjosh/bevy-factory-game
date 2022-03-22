use bevy::prelude::Component;

use super::{Element, ReferenceItem};

#[derive(Clone, Component, Debug, PartialEq, Eq, Hash)]
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
}

impl From<Vec<Element>> for Item {
    fn from(elements: Vec<Element>) -> Self {
        Self { elements }
    }
}
