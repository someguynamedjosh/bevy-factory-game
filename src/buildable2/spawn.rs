use bevy::prelude::*;

use super::{
    Buildable, BuildingComponentsContext, BuildingContext, Built, DynBuildable, MutBuildingMaps,
};

pub fn spawn_buildable(
    buildable: Box<dyn DynBuildable>,
    ctx: &mut BuildingContext,
    maps: &mut MutBuildingMaps,
) -> Entity {
    let built = Built {
        buildable: dyn_clone::clone_box(&*buildable),
    };
    buildable.spawn_self(built, ctx, maps)
}

fn set_positions_on_maps(
    buildable: &Box<dyn DynBuildable>,
    maps: &mut MutBuildingMaps,
    ctx: &mut BuildingContext,
    root: Entity,
) {
    let requested_maps = buildable.maps();
    for map in requested_maps {
        let map = map.get_from_maps_mut(maps);
        for pos in buildable.shape(ctx) {
            map.set(pos, root);
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
    ctx.commands.entity(buildable.0).despawn_recursive();
    let buildable = &buildable.1.buildable;
    clear_positions_on_maps(buildable, maps, ctx);
}

fn clear_positions_on_maps(
    buildable: &Box<dyn DynBuildable>,
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
