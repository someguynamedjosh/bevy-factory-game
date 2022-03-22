use bevy::prelude::*;
use itertools::Itertools;
use maplit::hashmap;

use crate::{
    buildable::{machine::shape::Shape, storage::ItemList},
    item::{Element, ReferenceItem},
    prelude::*,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MachineType {
    Purifier,
    Joiner,
}

impl MachineType {
    pub fn processing_time(self) -> u8 {
        match self {
            Self::Purifier => 40,
            Self::Joiner => 20,
        }
    }

    pub fn process(self, items: Vec<Item>) -> Vec<Item> {
        match self {
            Self::Purifier => {
                let (item,) = items.into_iter().collect_tuple().unwrap();
                vec![item.with_modified_elements(|xs| xs.filter(|x| x != &Element::Impurity))]
            }
            Self::Joiner => {
                let (a, b) = items.into_iter().collect_tuple().unwrap();
                let item = [a.into_elements(), b.into_elements()].concat().into();
                vec![item]
            }
        }
    }

    pub fn get_shape(self) -> &'static Shape {
        // coordinates are in the form (perp, par) -> the origin will always
        // have a vertex pointing +perp (side pointing -perp) If the direction
        // is up, par is up, and perp is left.
        match self {
            Self::Purifier => &Shape {
                blanks: &[(0, 1), (0, -1), (1, 1), (1, -1)],
                inputs: &[(1, 0)],
                outputs: &[(-1, 0)],
            },
            Self::Joiner => &Shape {
                blanks: &[],
                inputs: &[(0, 1), (0, -1)],
                outputs: &[(-1, 0)],
            },
        }
    }

    pub fn get_appearence(
        self,
        _assets: &CommonAssets,
    ) -> Option<(Handle<Mesh>, Handle<StandardMaterial>)> {
        match self {
            // Self::Furnace => Some((assets.furnace_mesh.clone(), assets.clay_mat.clone())),
            _ => None,
        }
    }

    pub(crate) fn get_cost(&self) -> ItemList {
        ItemList::from_counts(match self {
            Self::Joiner => hashmap![
                ReferenceItem::IronLump.as_item() => 4,
                ReferenceItem::PureAnimus.as_item() => 2,
            ],
            Self::Purifier => hashmap![
                ReferenceItem::IronLump.as_item() => 6,
                ReferenceItem::PureAnimus.as_item() => 1,
            ],
        })
    }
}
