use bevy::prelude::*;
use itertools::Itertools;

use crate::{
    buildable::{BuildingSpawner, Shape},
    item::{Element, ItemContainer},
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
            _ => 12,
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
        // (T, ||) -> the origin will always have a vertex pointing +T (side pointing
        // -T) If the direction is up, || is up, and T is left.
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
        assets: &CommonAssets,
    ) -> Option<(Handle<Mesh>, Handle<StandardMaterial>)> {
        match self {
            // Self::Furnace => Some((assets.furnace_mesh.clone(), assets.clay_mat.clone())),
            _ => None,
        }
    }
}
