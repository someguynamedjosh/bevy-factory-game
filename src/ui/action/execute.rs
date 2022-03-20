use bevy::prelude::*;

use super::{Action, ActionState};
use crate::{
    buildable::{
        self,
        claw::BClaw,
        conveyor::BConveyor,
        machine::{BMachine, MachineType},
        spawn_buildable, BuildingContext, BuildingMaps,
    },
    prelude::*,
    ui::cursor::CursorState,
};

pub fn execute_action(
    mut commands: Commands,
    cursor_state: Res<CursorState>,
    common_assets: Res<CommonAssets>,
    action_state: &mut ResMut<ActionState>,
    mut maps: BuildingMaps,
) {
    let mut ctx = BuildingContext {
        commands: &mut commands,
        position: cursor_state.world_pos,
        direction: cursor_state.direction,
        common_assets: &*common_assets,
    };
    match &action_state.action {
        Action::PlaceConveyor => execute_place_conveyor(&mut ctx, &mut maps),
        Action::PlaceClawStart => execute_place_claw_start(&cursor_state, action_state),
        &Action::PlaceClawEnd { take_from } => {
            execute_place_claw_end(cursor_state, take_from, &mut ctx, &mut maps, action_state)
        }
        Action::PlaceMachine(typ) => execute_place_machine(typ, ctx, maps),
    }
}

fn execute_place_machine(typ: &MachineType, mut ctx: BuildingContext, mut maps: BuildingMaps) {
    buildable::spawn_buildable(Box::new(BMachine(*typ)), &mut ctx, &mut maps);
}

fn execute_place_claw_end(
    cursor_state: Res<CursorState>,
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
    cursor_state: &Res<CursorState>,
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
