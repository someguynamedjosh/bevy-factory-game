use crate::item::{ItemAnimator, ItemContainer, ItemContainerAlignment};
use crate::prelude::*;
use bevy::prelude::*;

#[derive(Default)]
pub struct Conveyor {
    upstream: Option<Entity>,
    incoming_timer: u8,
    outgoing_timer: u8,
}
/// Conveyors that do not have any downstream.
struct TailConveyor;
/// It takes this many ticks for an item to ride one unit of a conveyor.
const DURATION: u8 = 4;

pub fn spawn_conveyor(
    commands: &mut Commands,
    common_assets: &Res<CommonAssets>,
    origin: IsoPos,
    facing: IsoDirection,
    start_with_item: bool,
) -> Entity {
    let alignment = ItemContainerAlignment::AxisAligned(facing.axis());
    let item = if start_with_item {
        Some(crate::item::spawn_item(
            commands,
            common_assets,
            Item::MetalRubble,
            origin,
            alignment,
        ))
    } else {
        None
    };
    // xor
    let material = if origin.points_left() != facing.is_negative() {
        common_assets.conveyor_mat.0.clone()
    } else {
        common_assets.conveyor_mat.1.clone()
    };
    commands
        .spawn(SpriteBundle {
            material,
            transform: origin.building_transform(facing.axis()),
            ..Default::default()
        })
        .with(origin)
        .with(facing)
        .with(ItemContainer::new_maybe_preloaded(alignment, item))
        .with(Conveyor::default())
        .with(SetupNeeded)
        .current_entity()
        .unwrap()
}

fn setup(
    commands: &mut Commands,
    all_conveyors: Query<(Entity, &IsoPos, &IsoDirection), With<Conveyor>>,
    mut unlinked_conveyors: Query<
        (Entity, &mut Conveyor, &IsoPos, &IsoDirection),
        With<SetupNeeded>,
    >,
) {
    let mut check_has_setup_needed = Vec::new();
    for (id, mut conveyor, pos, facing) in unlinked_conveyors.iter_mut() {
        let upstream_pos = pos.offset_direction(*facing, -1);
        let downstream_pos = pos.offset_direction(*facing, 1);
        let mut has_downstream = false;
        for (cid, cpos, cfacing) in all_conveyors.iter() {
            // If they are in our upstream position and we are in their downstream position...
            if *cpos == upstream_pos {
                let candidate_downstream_pos = cpos.offset_direction(*cfacing, 1);
                if candidate_downstream_pos == *pos {
                    conveyor.upstream = Some(cid);
                    // They have a downstream now, they cannot be
                    commands.remove_one::<TailConveyor>(cid);
                }
            }
            // If they are in our downstream position and we are in their upstream position...
            if *cpos == downstream_pos {
                check_has_setup_needed.push(cid);
                let candidate_upstream_pos = cpos.offset_direction(*cfacing, -1);
                if candidate_upstream_pos == *pos {
                    has_downstream = true;
                }
            }
        }
        commands.remove_one::<SetupNeeded>(id);
        if !has_downstream {
            commands.insert_one(id, TailConveyor);
        }
    }
    for id in check_has_setup_needed {
        if !unlinked_conveyors.get_mut(id).is_ok() {
            commands.insert_one(id, SetupNeeded);
        }
    }
}

fn tick(
    tail_conveyors: Query<(Entity,), With<TailConveyor>>,
    mut all_conveyors: Query<(&IsoPos, &mut Conveyor, &mut ItemContainer)>,
    mut all_items: Query<&mut ItemAnimator>,
) {
    for (mut current,) in tail_conveyors.iter() {
        loop {
            let (pos, mut conveyor, mut item_container) = all_conveyors.get_mut(current).unwrap();
            let empty = item_container.item.is_none();
            // True if the downstream belt could have taken an item we have but didn't.
            let not_taken = conveyor.incoming_timer == 0;
            conveyor.incoming_timer = conveyor.incoming_timer.saturating_sub(1);
            conveyor.outgoing_timer = conveyor.outgoing_timer.saturating_sub(1);
            // Don't allow placing items into the conveyor or moving items out of the conveyor if
            // there are items partially inside the conveyor.
            item_container.blocked = conveyor.incoming_timer > 0 || conveyor.outgoing_timer > 0;
            let alignment = item_container.alignment;
            let upstream = if let Some(upstream) = conveyor.upstream {
                upstream
            } else {
                break;
            };
            // if we are empty that also means our incoming_timer has to be at zero.
            if empty {
                let pos = pos.clone();
                let (_, mut upstream, mut up_container) = all_conveyors.get_mut(upstream).unwrap();
                if let Some(ientity) = up_container.try_take() {
                    let mut item = all_items.get_mut(ientity).unwrap();
                    item.anim_to_container(pos, alignment, DURATION);
                    upstream.outgoing_timer = DURATION;

                    let (_, mut this, mut this_container) = all_conveyors.get_mut(current).unwrap();
                    this.incoming_timer = DURATION - 1;
                    this_container.item = Some(ientity);
                    this_container.blocked = true;
                }
            } else if not_taken {
                item_container.item.map(|e| {
                    let mut item = all_items.get_mut(e).unwrap();
                    item.anim_stationary_in_container(*pos, alignment);
                });
            }
            current = upstream;
        }
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(fstage::SETUP, setup.system())
            .add_system_to_stage(fstage::TICK, tick.system());
    }
}
