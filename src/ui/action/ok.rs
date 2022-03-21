use bevy::prelude::*;

use super::{Action, ActionState};
use crate::{buildable::BuildingMaps, ui::cursor::CursorState};

pub fn update_action_ok(
    action_state: &mut ResMut<ActionState>,
    maps: &BuildingMaps,
    cursor_state: &Res<CursorState>,
) {
    action_state.ok = match &action_state.action {
        Action::PlaceConveyor => !maps.buildings.is_occupied(cursor_state.world_pos),
        Action::PlaceClawStart => {
            !maps.claws.is_occupied(cursor_state.world_pos)
                && cursor_state.hovered_container.is_some()
        }
        &Action::PlaceClawEnd { take_from } => {
            !maps.claws.is_occupied(cursor_state.world_pos)
                && cursor_state.hovered_container.is_some()
                && cursor_state.world_pos != take_from
        }
        Action::PlaceMachine(typ) => {
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
        Action::Destroy => {
            maps.buildings.is_occupied(cursor_state.world_pos)
                || maps.claws.is_occupied(cursor_state.world_pos)
        }
    };
}
