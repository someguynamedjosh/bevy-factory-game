use bevy::prelude::*;
use dyn_clone::DynClone;

use crate::{
    iso::{ItemContainerMap, SpatialMap},
    prelude::*,
};

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

pub struct Built {
    pub buildable: Box<dyn Buildable>,
}

pub trait Buildable: DynClone + Sync + Send + 'static {
    fn shape(&self, ctx: &mut BuildingContext) -> Vec<IsoPos>;
    fn maps(&self) -> Vec<WhichMap>;
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
    let root = spawn_root(&buildable, ctx);
    BuildableSpawner {
        buildable: &*buildable,
        root,
        ctx,
        maps,
    }
    .finish_spawning()
}

fn spawn_root(buildable: &Box<dyn Buildable>, ctx: &mut BuildingContext) -> Entity {
    let built = Built {
        buildable: dyn_clone::clone_box(&**buildable),
    };
    let root = ctx.commands.spawn((built,)).current_entity().unwrap();
    root
}

struct BuildableSpawner<'a, 'b, 'c> {
    buildable: &'a dyn Buildable,
    root: Entity,
    ctx: &'a mut BuildingContext<'b>,
    maps: &'a mut MutBuildingMaps<'c>,
}

impl<'a, 'b, 'c> BuildableSpawner<'a, 'b, 'c> {
    fn finish_spawning(mut self) -> Entity {
        self.spawn_art();
        self.spawn_others();
        self.mark_positions_on_maps();
        self.root
    }

    fn mark_positions_on_maps(&mut self) {
        let requested_maps = self.buildable.maps();
        for map in requested_maps {
            let map = map.get_from_maps_mut(self.maps);
            for pos in self.buildable.shape(self.ctx) {
                map.set_assuming_empty(pos, self.root);
            }
        }
    }

    fn spawn_art(&mut self) {
        for art in self.buildable.spawn_art(self.ctx) {
            self.ctx.commands.set_current_entity(art);
            self.ctx.commands.with(Parent(self.root));
        }
    }

    fn spawn_others(&mut self) {
        for extra in self.buildable.spawn_extras(self.ctx, self.maps) {
            self.ctx.commands.set_current_entity(extra);
            self.ctx.commands.with(Parent(self.root));
        }
    }
}

pub fn destroy_buildable(
    buildable: (Entity, &Built),
    ctx: &mut BuildingContext,
    maps: &mut MutBuildingMaps,
) {
    // The spawner parents everything to the root entity, so this will take care
    // of all art and other related entities as well as the buildable object
    // itself.
    ctx.commands.despawn_recursive(buildable.0);
    let buildable = &buildable.1.buildable;
    clear_positions_on_maps(buildable, maps, ctx);
}

fn clear_positions_on_maps(
    buildable: &Box<dyn Buildable>,
    maps: &mut MutBuildingMaps,
    ctx: &mut BuildingContext,
) {
    let requested_maps = buildable.maps();
    for map in requested_maps {
        let map = map.get_from_maps_mut(maps);
        for pos in buildable.shape(ctx) {
            map.clear(pos);
        }
    }
}
