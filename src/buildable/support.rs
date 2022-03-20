use bevy::{prelude::*, ecs::{system::{EntityCommands, SystemParam}, query::WorldQuery}};

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

pub struct BuildingComponentsContext<'a, 'c1, 'c2> {
    pub commands: &'a mut EntityCommands<'c1, 'c2, 'a>,
    pub position: IsoPos,
    pub direction: IsoDirection,
    pub common_assets: &'a CommonAssets,
}

#[derive(SystemParam)]
pub struct BuildingMaps<'w, 's> {
    pub buildings: ResMut<'w, BuildingMap>,
    pub item_containers: ResMut<'w, ItemContainerMap>,
    pub conveyors: ResMut<'w, ConveyorMap>,
    #[allow(dead_code)]
    s: Query<'w, 's, ()>,
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
        maps: &'b mut BuildingMaps,
    ) -> &'b mut SpatialMap<Entity> {
        match self {
            WhichMap::Buildings => &mut **maps.buildings,
            WhichMap::Conveyors => &mut **maps.conveyors,
            WhichMap::ItemContainers => &mut **maps.item_containers,
        }
    }
}
