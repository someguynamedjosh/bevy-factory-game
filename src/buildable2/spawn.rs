use bevy::prelude::*;

use super::{Buildable, BuildingContext, Built, MutBuildingMaps};

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
    let root = ctx
        .commands
        .spawn()
        .with_bundle((built, Transform::identity()))
        .current_entity()
        .unwrap();
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