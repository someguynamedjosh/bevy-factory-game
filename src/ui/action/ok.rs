use bevy::prelude::*;
use maplit::hashmap;

use super::{Action, ActionState};
use crate::{
    buildable::{
        claw::BClaw,
        conveyor::BConveyor,
        storage::{ItemList, Storage},
        Buildable, BuildingDetails, BuildingMaps,
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
    let direction = cursor_state.direction;
    let (prereqs_ok, deets) = match &action_state.action {
        Action::PlaceConveyor => (
            !maps.buildings.is_occupied(position),
            BConveyor.details(position, direction, maps),
        ),
        Action::PlaceClawStart => (
            !maps.claws.is_occupied(position) && cursor_state.hovered_container.is_some(),
            BClaw {
                take_from: position,
            }
            .details(position, direction, maps),
        ),
        &Action::PlaceClawEnd { take_from } => (
            !maps.claws.is_occupied(position)
                && cursor_state.hovered_container.is_some()
                && position != take_from,
            BClaw { take_from }.details(position, direction, maps),
        ),
        Action::PlaceBuildable(bld) => {
            let deets = bld.details(position, direction, maps);
            let shape = deets.as_ref().map(|x| &x.shape[..]).unwrap_or(&[]);
            let space_ok = (|| {
                for &p in shape {
                    if maps.buildings.is_occupied(p) {
                        return false;
                    }
                }
                true
            })();
            (space_ok, deets)
        }
        Action::Destroy => (
            maps.buildings.is_occupied(position) || maps.claws.is_occupied(position),
            Some(BuildingDetails {
                shape: vec![],
                maps: vec![],
                cost: ItemList::new(),
            }),
        ),
    };
    action_state.ok = prereqs_ok;
    if let Some(deets) = deets {
        let required_items = deets.cost;
        let mut required_items_not_in_storage = required_items.clone();
        for (storage,) in storages.iter() {
            storage.subtract_available_inventory_from(&mut required_items_not_in_storage);
        }
        action_state.ok &= required_items_not_in_storage.total_count() == 0;
        action_state.required_items = required_items;
    } else {
        action_state.ok = false;
        action_state.required_items = ItemList::new();
    }
}
