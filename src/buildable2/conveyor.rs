use bevy::prelude::*;

use super::{Buildable, BuildingContext, WhichMap};
use crate::{
    item::{ItemAnimator, ItemContainer, ItemContainerAlignment},
    prelude::*,
};

#[derive(Clone)]
pub struct BConveyor;

impl Buildable for BConveyor {
    fn shape(&self, ctx: &mut BuildingContext) -> Vec<IsoPos> {
        vec![ctx.position]
    }

    fn maps(&self) -> Vec<WhichMap> {
        vec![WhichMap::Buildings, WhichMap::Conveyors]
    }

    fn extra_root_components(&self, ctx: &mut BuildingContext) {
        ctx.commands
            .with(ConveyorLogic::default())
            .with(ItemContainer::new_empty(
                ItemContainerAlignment::AxisAligned(ctx.direction.axis()),
            ))
            .with(SetupNeeded);
    }

    fn spawn_art(&self, ctx: &mut BuildingContext) -> Vec<Entity> {
        // xor
        let material = if ctx.position.points_left() != ctx.direction.is_negative() {
            ctx.common_assets.conveyor_mat.0.clone()
        } else {
            ctx.common_assets.conveyor_mat.1.clone()
        };
        let transform = ctx.position.building_transform(ctx.direction.axis()) * SPRITE_TRANSFORM;
        ctx.commands.spawn().with_bundle(SpriteBundle {
            material,
            transform,
            ..Default::default()
        });
        vec![ctx.commands.current_entity().unwrap()]
    }
}

#[derive(Default)]
struct ConveyorLogic {
    upstream: Option<Entity>,
    incoming_timer: u8,
    outgoing_timer: u8,
}
/// Conveyors that do not have any downstream.
struct TailConveyor;
/// It takes this many ticks for an item to ride one unit of a conveyor.
const DURATION: u8 = 6;

fn setup(
    commands: &mut Commands,
    all_conveyors: Query<(Entity, &IsoPos, &IsoDirection), With<ConveyorLogic>>,
    mut unlinked_conveyors: Query<
        (Entity, &mut ConveyorLogic, &IsoPos, &IsoDirection),
        With<SetupNeeded>,
    >,
) {
    let mut check_has_setup_needed = Vec::new();
    for (id, mut conveyor, pos, facing) in unlinked_conveyors.iter_mut() {
        let upstream_pos = pos.offset_direction(*facing, -1);
        let downstream_pos = pos.offset_direction(*facing, 1);
        let mut has_downstream = false;
        for (cid, cpos, cfacing) in all_conveyors.iter() {
            // If they are in our upstream position and we are in their downstream
            // position...
            if *cpos == upstream_pos {
                let candidate_downstream_pos = cpos.offset_direction(*cfacing, 1);
                if candidate_downstream_pos == *pos {
                    conveyor.upstream = Some(cid);
                    // They have a downstream now, they cannot be
                    commands.remove_one::<TailConveyor>(cid);
                }
            }
            // If they are in our downstream position and we are in their upstream
            // position...
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
    mut all_conveyors: Query<(&IsoPos, &mut ConveyorLogic, &mut ItemContainer)>,
    mut all_items: Query<&mut ItemAnimator>,
) {
    for (current,) in tail_conveyors.iter() {
        tick_conveyor(&mut all_conveyors, current, &mut all_items);
    }
}

fn tick_conveyor(
    all_conveyors: &mut Query<(&IsoPos, &mut ConveyorLogic, &mut ItemContainer)>,
    current: Entity,
    all_items: &mut Query<&mut ItemAnimator>,
) {
    let (pos, mut conveyor, mut item_container) = all_conveyors.get_mut(current).unwrap();
    let empty = item_container.item().is_none();
    // True if the downstream belt could have taken an item we have but didn't.
    let not_taken = conveyor.incoming_timer == 0;
    conveyor.incoming_timer = conveyor.incoming_timer.saturating_sub(1);
    conveyor.outgoing_timer = conveyor.outgoing_timer.saturating_sub(1);
    let alignment = item_container.alignment();
    let upstream = if let Some(upstream) = conveyor.upstream {
        upstream
    } else {
        return;
    };
    item_container.set_blocked(false);
    if empty {
        let pos = pos.clone();
        let (_, mut upstream, mut up_container) = all_conveyors.get_mut(upstream).unwrap();
        if let Some(ientity) = up_container.try_take() {
            let mut item = all_items.get_mut(ientity).unwrap();
            item.anim_to_container(pos, alignment, DURATION);
            upstream.outgoing_timer = DURATION;

            let (_, mut this, mut this_container) = all_conveyors.get_mut(current).unwrap();
            this.incoming_timer = DURATION - 1;
            this_container.put_item(ientity);
            this_container.set_blocked(true);
        }
    } else if not_taken {
        item_container.item().map(|e| {
            let mut item = all_items.get_mut(e).unwrap();
            item.anim_stationary_in_container(*pos, alignment);
        });
    }

    let (_pos, conveyor, mut item_container) = all_conveyors.get_mut(current).unwrap();
    // Don't allow placing items into the conveyor or moving items out of the
    // conveyor if there are items partially inside the conveyor.
    item_container.set_blocked(conveyor.incoming_timer > 0 || conveyor.outgoing_timer > 0);

    tick_conveyor(all_conveyors, upstream, all_items);
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(fstage::SETUP, setup.system())
            .add_system_to_stage(fstage::TICK, tick.system());
    }
}
