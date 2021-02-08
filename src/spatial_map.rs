use crate::prelude::*;
use bevy::prelude::*;
use std::{collections::HashMap, ops::{Deref, DerefMut}};

pub struct SpatialMap<Data> {
    contents: HashMap<IsoPos, Data>,
}

impl<Data> SpatialMap<Data> {
    pub fn new() -> Self {
        Self {
            contents: HashMap::new(),
        }
    }

    pub fn set(&mut self, pos: IsoPos, data: Data) {
        self.contents.insert(pos, data);
    }

    /// Like set(), but asserts that the cell is not occupied.
    pub fn set_empty(&mut self, pos: IsoPos, data: Data) {
        assert!(!self.is_occupied(pos));
        self.set(pos, data);
    }

    pub fn clear(&mut self, pos: IsoPos) -> Option<Data> {
        self.contents.remove(&pos)
    }

    pub fn get(&self, pos: IsoPos) -> Option<&Data> {
        self.contents.get(&pos)
    }

    pub fn is_occupied(&self, pos: IsoPos) -> bool {
        self.contents.contains_key(&pos)
    }

    pub fn occupy(&mut self, pos: IsoPos)
    where
        Data: Default,
    {
        self.contents.insert(pos, Default::default());
    }
}

macro_rules! map_newtype {
    ($name:ident, $data:ty) => {
        pub struct $name(SpatialMap<$data>);
        impl Default for $name {
            fn default() -> Self {
                Self(SpatialMap::new())
            }
        }
        impl Deref for $name {
            type Target = SpatialMap<Entity>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

map_newtype!(ConveyorMap, Entity);
map_newtype!(ItemContainerMap, Entity);
map_newtype!(BuildingObstructionMap, Entity);

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ConveyorMap::default());
        app.add_resource(ItemContainerMap::default());
        app.add_resource(BuildingObstructionMap::default());
    }
}
