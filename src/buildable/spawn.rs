use bevy::prelude::*;

use super::{BuildingContext, BuildingDetails, BuildingMaps, Built, DynBuildable};

pub fn spawn_buildable(
    buildable: Box<dyn DynBuildable>,
    ctx: &mut BuildingContext,
    maps: &mut BuildingMaps,
) -> Entity {
    let built = Built {
        buildable: dyn_clone::clone_box(&*buildable),
        position: ctx.position,
        direction: ctx.direction,
    };
    let root = buildable.spawn_self(built, ctx, maps);
    set_positions_on_maps(&buildable, maps, ctx, root);
    root
}

fn set_positions_on_maps(
    buildable: &Box<dyn DynBuildable>,
    maps: &mut BuildingMaps,
    ctx: &mut BuildingContext,
    root: Entity,
) {
    let BuildingDetails {
        maps: requested_maps,
        shape,
        ..
    } = buildable
        .details(ctx.position, ctx.direction, maps)
        .unwrap();
    for map in requested_maps {
        let map = map.get_from_maps_mut(maps);
        for &pos in &shape {
            map.set(pos, root);
        }
    }
}

pub fn destroy_buildable(
    buildable: (Entity, &Built),
    ctx: &mut BuildingContext,
    maps: &mut BuildingMaps,
) {
    // The spawner parents everything to the root entity, so this will take care
    // of all art and other related entities as well as the buildable object
    // itself.
    ctx.position = buildable.1.position;
    ctx.direction = buildable.1.direction;
    ctx.commands.entity(buildable.0).despawn_recursive();
    let buildable = &buildable.1.buildable;
    buildable.on_destroy(ctx, maps);
    clear_positions_on_maps(buildable, maps, ctx);
}

fn clear_positions_on_maps(
    buildable: &Box<dyn DynBuildable>,
    maps: &mut BuildingMaps,
    ctx: &mut BuildingContext,
) {
    let BuildingDetails {
        maps: requested_maps,
        shape,
        ..
    } = buildable
        .details(ctx.position, ctx.direction, maps)
        .unwrap();
    for map in requested_maps {
        let map = map.get_from_maps_mut(maps);
        for &pos in &shape {
            map.clear(pos);
        }
    }
}
