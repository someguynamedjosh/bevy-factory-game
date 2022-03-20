use bevy::prelude::*;

use super::cursor::CursorState;
use crate::{
    buildable::{
        self,
        claw::BClaw,
        conveyor::BConveyor,
        machine::{BMachine, MachineType},
        spawn_buildable, BuildingContext, BuildingMaps,
    },
    prelude::*,
};

pub enum Action {
    PlaceConveyor,
    PlaceClaw,
    PlaceClawEnd { take_from: IsoPos },
    Machine(MachineType),
}

pub struct ActionState {
    pub action: Action,
    pub ok: bool,
}

impl Action {
    pub fn get_snapping(&self, selected_direction: IsoDirection) -> Snapping {
        match self {
            Self::PlaceConveyor => Snapping::None,
            Self::PlaceClaw => Snapping::None,
            Self::PlaceClawEnd {
                take_from: start_pos,
                ..
            } => Snapping::AlongAnyLine {
                through: *start_pos,
            },
            Self::Machine(..) => Snapping::require_edge_pointing_in(selected_direction),
        }
    }
}

pub fn startup(mut commands: Commands) {
    commands.insert_resource(ActionState {
        action: Action::PlaceConveyor,
        ok: false,
    })
}

pub fn update(
    mut commands: Commands,
    common_assets: Res<CommonAssets>,
    mut action_state: ResMut<ActionState>,
    cursor_state: Res<CursorState>,
    input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    mut maps: BuildingMaps,
) {
    action_state.ok = match &action_state.action {
        Action::PlaceConveyor => !maps.buildings.is_occupied(cursor_state.world_pos),
        Action::PlaceClaw | Action::PlaceClawEnd { .. } => cursor_state.hovered_container.is_some(),
        Action::Machine(typ) => {
            let shape = typ.get_shape();
            (|| {
                let iters = shape.positions(cursor_state.world_pos, cursor_state.direction);
                for p in iters.blanks.chain(iters.inputs.chain(iters.outputs)) {
                    if maps.buildings.is_occupied(p) {
                        return false;
                    }
                }
                true
            })()
        }
    };
    if input.just_pressed(MouseButton::Left) && action_state.ok {
        let mut ctx = BuildingContext {
            commands: &mut commands,
            position: cursor_state.world_pos,
            direction: cursor_state.direction,
            common_assets: &*common_assets,
        };
        match &action_state.action {
            Action::PlaceConveyor => {
                buildable::spawn_buildable(Box::new(BConveyor), &mut ctx, &mut maps);
            }
            Action::PlaceClaw => {
                if let Some(_) = cursor_state.hovered_container {
                    action_state.action = Action::PlaceClawEnd {
                        take_from: cursor_state.world_pos,
                    };
                }
            }
            &Action::PlaceClawEnd { take_from } => {
                if let Some(_) = cursor_state.hovered_container {
                    spawn_buildable(Box::new(BClaw { take_from }), &mut ctx, &mut maps);
                    action_state.action = Action::PlaceClaw;
                }
            }
            Action::Machine(typ) => {
                buildable::spawn_buildable(Box::new(BMachine(*typ)), &mut ctx, &mut maps);
            }
        }
    }
    if key_input.just_pressed(KeyCode::Key1) {
        action_state.action = Action::PlaceConveyor
    }
    if key_input.just_pressed(KeyCode::Key2) {
        action_state.action = Action::PlaceClaw;
    }
    if key_input.just_pressed(KeyCode::Key3) {
        action_state.action = Action::Machine(MachineType::Purifier);
    }
    if key_input.just_pressed(KeyCode::Key4) {
        action_state.action = Action::Machine(MachineType::Joiner);
    }
}
