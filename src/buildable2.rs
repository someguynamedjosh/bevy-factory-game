use bevy::prelude::*;
use dyn_clone::DynClone;

use crate::{iso::ItemContainerMap, prelude::*};

pub struct BuildingContext<'a> {
    pub commands: &'a mut Commands,
    pub position: IsoPos,
    pub direction: IsoDirection,
}

pub struct MutBuildingMaps<'a> {
    pub buildings: &'a mut BuildingObstructionMap,
    pub item_containers: &'a mut ItemContainerMap,
    pub conveyors: &'a mut ConveyorMap,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Map {
    Buildings,
    Conveyors,
    ItemContainers,
}

pub trait Buildable: DynClone + Sync + Send + 'static {
    fn shape(&self, ctx: &mut BuildingContext) -> Vec<IsoPos>;
    fn maps(&self) -> Vec<Map>;
    fn extra_root_components(&self, ctx: &mut BuildingContext);
    #[allow(unused_variables)]
    fn spawn_extras(&self, ctx: &mut BuildingContext, maps: &mut MutBuildingMaps) -> Vec<Entity> {
        vec![]
    }
    fn spawn_art(&self, ctx: &mut BuildingContext) -> Vec<Entity>;
}

pub fn spawn_buildable(
    buildable: Box<dyn Buildable>,
    ctx: &mut BuildingContext,
    maps: &mut MutBuildingMaps,
) -> Entity {
    let built = Built {
        buildable: dyn_clone::clone_box(&*buildable),
    };
    let root = ctx.commands.spawn((built,)).current_entity().unwrap();
    for extra in buildable.spawn_extras(ctx, maps) {
        ctx.commands.set_current_entity(extra);
        ctx.commands.with(Parent(root));
    }
    for art in buildable.spawn_art(ctx) {
        ctx.commands.set_current_entity(art);
        ctx.commands.with(Parent(root));
    }
    let requested_maps = buildable.maps();
    for pos in buildable.shape(ctx) {
        if requested_maps.contains(&Map::Buildings) {
            maps.buildings.set_assuming_empty(pos, root);
        }
        if requested_maps.contains(&Map::Conveyors) {
            maps.conveyors.set_assuming_empty(pos, root);
        }
        if requested_maps.contains(&Map::ItemContainers) {
            maps.item_containers.set_assuming_empty(pos, root);
        }
    }
    root
}

pub struct Built {
    pub buildable: Box<dyn Buildable>,
}
