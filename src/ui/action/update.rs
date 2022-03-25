use bevy::prelude::*;

use super::{Action, ActionState};
use crate::{
    buildable::{
        drill::BDrill,
        machine::{BMachine, MachineType},
        storage::{BSmallWarehouse, ItemList, Storage},
        BuildingContext, BuildingMaps, Built,
    },
    prelude::*,
    ui::cursor::CursorState,
};

pub fn update(
    mut commands: Commands,
    common_assets: Res<CommonAssets>,
    mut action_state: ResMut<ActionState>,
    cursor_state: Res<CursorState>,
    input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    maps: BuildingMaps,
    built: Query<&Built>,
    storages: Query<(&mut Storage,)>,
) {
    super::ok::update_action_ok(&mut action_state, &maps, &cursor_state, &storages);
    if input.pressed(MouseButton::Left) && action_state.ok {
        super::execute::execute_action(
            &mut commands,
            &cursor_state,
            &common_assets,
            &mut action_state,
            maps,
            built,
            storages,
        );
    }
    handle_change_action_input(key_input, &mut action_state);
    update_preview(
        &mut commands,
        &mut action_state,
        &cursor_state,
        &common_assets,
    );
}

fn handle_change_action_input(key_input: Res<Input<KeyCode>>, action_state: &mut ActionState) {
    if key_input.just_pressed(KeyCode::Grave) {
        action_state.action = Action::Destroy
    }
    if key_input.just_pressed(KeyCode::Key1) {
        action_state.action = Action::PlaceConveyor
    }
    if key_input.just_pressed(KeyCode::Key2) {
        action_state.action = Action::PlaceClawStart;
    }
    if key_input.just_pressed(KeyCode::Key3) {
        action_state.action = Action::PlaceBuildable(Box::new(BMachine(MachineType::Purifier)));
    }
    if key_input.just_pressed(KeyCode::Key4) {
        action_state.action = Action::PlaceBuildable(Box::new(BMachine(MachineType::Joiner)));
    }
    if key_input.just_pressed(KeyCode::Key5) {
        action_state.action = Action::PlaceBuildable(Box::new(BSmallWarehouse(ItemList::new())));
    }
    if key_input.just_pressed(KeyCode::Key6) {
        action_state.action = Action::PlaceBuildable(Box::new(BDrill));
    }
}

fn update_preview(
    commands: &mut Commands,
    action_state: &mut ActionState,
    cursor_state: &CursorState,
    common_assets: &CommonAssets,
) {
    for ent in std::mem::take(&mut action_state.preview) {
        commands.entity(ent).despawn_recursive();
    }
    let ctx = &mut BuildingContext {
        commands,
        position: cursor_state.world_pos,
        direction: cursor_state.direction,
        common_assets,
    };
    if action_state.ok {
        action_state.preview = action_state.action.spawn_art(ctx);
    }
}
