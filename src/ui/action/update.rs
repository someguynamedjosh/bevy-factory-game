use bevy::prelude::*;

use super::{Action, ActionState};
use crate::{
    buildable::{machine::MachineType, BuildingMaps},
    prelude::*,
    ui::cursor::CursorState,
};

pub fn update(
    commands: Commands,
    common_assets: Res<CommonAssets>,
    mut action_state: ResMut<ActionState>,
    cursor_state: Res<CursorState>,
    input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    maps: BuildingMaps,
) {
    super::ok::update_action_ok(&mut action_state, &maps, &cursor_state);
    if input.just_pressed(MouseButton::Left) && action_state.ok {
        super::execute::execute_action(
            commands,
            cursor_state,
            common_assets,
            &mut action_state,
            maps,
        );
    }
    handle_change_action_input(key_input, action_state);
}

fn handle_change_action_input(
    key_input: Res<Input<KeyCode>>,
    mut action_state: ResMut<ActionState>,
) {
    if key_input.just_pressed(KeyCode::Key1) {
        action_state.action = Action::PlaceConveyor
    }
    if key_input.just_pressed(KeyCode::Key2) {
        action_state.action = Action::PlaceClawStart;
    }
    if key_input.just_pressed(KeyCode::Key3) {
        action_state.action = Action::PlaceMachine(MachineType::Purifier);
    }
    if key_input.just_pressed(KeyCode::Key4) {
        action_state.action = Action::PlaceMachine(MachineType::Joiner);
    }
}
