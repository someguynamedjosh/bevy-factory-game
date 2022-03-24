use bevy::{
    ecs::system::{EntityCommands, SystemParam},
    prelude::*,
};

use crate::{
    iso::{ItemContainerMap, SpatialMap, ClawMap, },
    prelude::*, resource_nodes::ResourceNodeMap,
};

pub struct BuildingContext<'a, 'c1, 'c2> {
    pub commands: &'a mut Commands<'c1, 'c2>,
    pub position: IsoPos,
    pub direction: IsoDirection,
    pub common_assets: &'a CommonAssets,
}

pub struct BuildingComponentsContext<'a, 'c1, 'c2> {
    pub commands: &'a mut EntityCommands<'c1, 'c2, 'a>,
    pub position: IsoPos,
    pub direction: IsoDirection,
    pub common_assets: &'a CommonAssets,
}

#[derive(SystemParam)]
pub struct BuildingMaps<'w, 's> {
    pub buildings: ResMut<'w, BuildingMap>,
    pub claws: ResMut<'w, ClawMap>,
    pub item_containers: ResMut<'w, ItemContainerMap>,
    pub conveyors: ResMut<'w, ConveyorMap>,
    pub resource_nodes: ResMut<'w, ResourceNodeMap>,
    #[allow(dead_code)]
    s: Query<'w, 's, ()>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WhichMap {
    Buildings,
    Claws,
    Conveyors,
    ItemContainers,
}

impl WhichMap {
    pub fn get_from_maps_mut<'b>(self, maps: &'b mut BuildingMaps) -> &'b mut SpatialMap<Entity> {
        match self {
            WhichMap::Buildings => &mut **maps.buildings,
            WhichMap::Claws => &mut **maps.claws,
            WhichMap::Conveyors => &mut **maps.conveyors,
            WhichMap::ItemContainers => &mut **maps.item_containers,
        }
    }
}
