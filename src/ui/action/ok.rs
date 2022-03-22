use bevy::prelude::*;
use maplit::hashmap;

use super::{Action, ActionState};
use crate::{
    buildable::{
        storage::{ItemList, Storage},
        BuildingMaps,
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
    let (prereqs_ok, required_items) = match &action_state.action {
        Action::PlaceConveyor => (
            !maps.buildings.is_occupied(cursor_state.world_pos),
            ItemList::from_counts(hashmap![
                (ReferenceItem::Magnetite.as_item()) => 1,
            ]),
        ),
        Action::PlaceClawStart => (
            !maps.claws.is_occupied(cursor_state.world_pos)
                && cursor_state.hovered_container.is_some(),
            ItemList::new(),
        ),
        &Action::PlaceClawEnd { take_from } => (
            !maps.claws.is_occupied(cursor_state.world_pos)
                && cursor_state.hovered_container.is_some()
                && cursor_state.world_pos != take_from,
            ItemList::new(),
        ),
        Action::PlaceBuildable(bld) => {
            let shape = bld.shape(cursor_state.world_pos, cursor_state.direction);
            let space_ok = (|| {
                for p in shape {
                    if maps.buildings.is_occupied(p) {
                        return false;
                    }
                }
                true
            })();
            (space_ok, ItemList::new())
        }
        Action::Destroy => (
            maps.buildings.is_occupied(cursor_state.world_pos)
                || maps.claws.is_occupied(cursor_state.world_pos),
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
