use bevy::prelude::*;

use super::{Action, ActionState};
use crate::{
    buildable::{
        self,
        claw::BClaw,
        conveyor::BConveyor,
        destroy_buildable,
        machine::{BMachine, MachineType},
        spawn_buildable, BuildingContext, BuildingMaps, Built, DynBuildable,
    },
    prelude::*,
    ui::cursor::CursorState,
};

pub fn execute_action(
    commands: &mut Commands,
    cursor_state: &CursorState,
    common_assets: &CommonAssets,
    action_state: &mut ResMut<ActionState>,
    mut maps: BuildingMaps,
    built: Query<&Built>,
) {
    let mut ctx = BuildingContext {
        commands,
        position: cursor_state.world_pos,
        direction: cursor_state.direction,
        common_assets: &*common_assets,
    };
    match &action_state.action {
        Action::PlaceConveyor => execute_place_conveyor(&mut ctx, &mut maps),
        Action::PlaceClawStart => execute_place_claw_start(cursor_state, action_state),
        &Action::PlaceClawEnd { take_from } => {
            execute_place_claw_end(cursor_state, take_from, &mut ctx, &mut maps, action_state)
        }
        Action::PlaceBuildable(bld) => execute_place_buildable(bld, ctx, maps),
        Action::Destroy => execute_destroy(built, ctx, maps),
    }
}

fn execute_destroy(built: Query<&Built>, mut ctx: BuildingContext, mut maps: BuildingMaps) {
    let pos = ctx.position;
    let ent = *maps.claws.get(pos).or(maps.buildings.get(pos)).unwrap();
    let built = built.get(ent).unwrap();
    destroy_buildable((ent, built), &mut ctx, &mut maps)
}

fn execute_place_machine(typ: &MachineType, mut ctx: BuildingContext, mut maps: BuildingMaps) {
    buildable::spawn_buildable(Box::new(BMachine(*typ)), &mut ctx, &mut maps);
}

fn execute_place_buildable(bld: &Box<dyn DynBuildable>, mut ctx: BuildingContext, mut maps: BuildingMaps) {
    buildable::spawn_buildable(dyn_clone::clone_box(&**bld), &mut ctx, &mut maps);
}

fn execute_place_claw_end(
    cursor_state: &CursorState,
    take_from: IsoPos,
    ctx: &mut BuildingContext,
    maps: &mut BuildingMaps,
    action_state: &mut ResMut<ActionState>,
) {
    if let Some(_) = cursor_state.hovered_container {
        spawn_buildable(Box::new(BClaw { take_from }), ctx, maps);
        action_state.action = Action::PlaceClawStart;
    }
}

fn execute_place_claw_start(
    cursor_state: &CursorState,
    action_state: &mut ResMut<ActionState>,
) {
    if let Some(_) = cursor_state.hovered_container {
        action_state.action = Action::PlaceClawEnd {
            take_from: cursor_state.world_pos,
        };
    }
}

fn execute_place_conveyor(ctx: &mut BuildingContext, maps: &mut BuildingMaps) {
    buildable::spawn_buildable(Box::new(BConveyor), ctx, maps);
}
