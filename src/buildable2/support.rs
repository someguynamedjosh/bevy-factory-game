use bevy::prelude::*;

use crate::{
    iso::{ItemContainerMap, SpatialMap},
    prelude::*,
};

pub struct BuildingContext<'a, 'c1, 'c2> {
    pub commands: &'a mut Commands<'c1, 'c2>,
    pub position: IsoPos,
    pub direction: IsoDirection,
    pub common_assets: &'a CommonAssets,
}

pub struct MutBuildingMaps<'a> {
    pub buildings: &'a mut BuildingObstructionMap,
    pub item_containers: &'a mut ItemContainerMap,
    pub conveyors: &'a mut ConveyorMap,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WhichMap {
    Buildings,
    Conveyors,
    ItemContainers,
}

impl WhichMap {
    pub fn get_from_maps_mut<'b>(
        self,
        maps: &'b mut MutBuildingMaps,
    ) -> &'b mut SpatialMap<Entity> {
        match self {
            WhichMap::Buildings => &mut **maps.buildings,
            WhichMap::Conveyors => &mut **maps.conveyors,
            WhichMap::ItemContainers => &mut **maps.item_containers,
        }
    }
}
