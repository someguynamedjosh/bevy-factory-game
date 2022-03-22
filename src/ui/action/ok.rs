use bevy::prelude::*;
use maplit::hashmap;

use super::{Action, ActionState};
use crate::{
    buildable::{
        claw::BClaw,
        conveyor::BConveyor,
        storage::{ItemList, Storage},
        Buildable, BuildingMaps,
    },
    item::ReferenceItem,
    ui::cursor::CursorState,
};

pub fn update_action_ok(
    action_state: &mut ResMut<ActionState>,
    maps: &BuildingMaps,
    cursor_state: &Res<CursorState>,
    storages: &Query<(&mut Storage,)>,
) {
    let position = cursor_state.world_pos;
    let (prereqs_ok, required_items) = match &action_state.action {
        Action::PlaceConveyor => (
            !maps.buildings.is_occupied(position),
            BConveyor.cost(position),
        ),
        Action::PlaceClawStart => (
            !maps.claws.is_occupied(position) && cursor_state.hovered_container.is_some(),
            BClaw {
                take_from: position,
            }
            .cost(position),
        ),
        &Action::PlaceClawEnd { take_from } => (
            !maps.claws.is_occupied(position)
                && cursor_state.hovered_container.is_some()
                && position != take_from,
            BClaw { take_from }.cost(position),
        ),
        Action::PlaceBuildable(bld) => {
            let shape = bld.shape(position, cursor_state.direction);
            let space_ok = (|| {
                for p in shape {
                    if maps.buildings.is_occupied(p) {
                        return false;
                    }
                }
                true
            })();
            (space_ok, bld.cost(position))
        }
        Action::Destroy => (
            maps.buildings.is_occupied(position) || maps.claws.is_occupied(position),
            ItemList::new(),
        ),
    };
    let mut required_items_not_in_storage = required_items.clone();
    for (storage,) in storages.iter() {
        storage.subtract_available_inventory_from(&mut required_items_not_in_storage);
    }
    action_state.ok = prereqs_ok && required_items_not_in_storage.total_count() == 0;
    action_state.required_items = required_items;
}
