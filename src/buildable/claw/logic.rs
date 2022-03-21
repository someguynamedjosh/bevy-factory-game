use bevy::prelude::*;

use crate::{
    item::{ItemAnimator, ItemContainer},
    prelude::*,
};

#[derive(Component)]
pub(super) struct ClawLogic {
    pub(super) take_from: Entity,
    pub(super) move_to: Entity,
    pub(super) held_item: Option<Entity>,
    /// Length of the gantry in grid cells.
    pub(super) length: u8,
    pub(super) current_anim_tick: u8,
    pub(super) blocked: bool,
}
// How long it takes for the claw to traverse a segment of its path.
const SEGMENT_DURATION: u8 = 2;

impl ClawLogic {
    /// How many ticks it takes to make a two-way trip.
    fn anim_length(&self) -> u8 {
        // A 1 length claw has to traverse 2 segments, 2 length 3 segments, 3/4, etc.
        (self.length + 1) * SEGMENT_DURATION * 2
    }
}

pub(super) fn tick(
    mut claws: Query<(&mut ClawLogic, &mut Handle<StandardMaterial>)>,
    mut containers: Query<(&mut ItemContainer, &IsoPos)>,
    mut items: Query<&mut ItemAnimator>,
    common_assets: Res<CommonAssets>,
) {
    for (mut claw, mut mat) in claws.iter_mut() {
        let anim_length = claw.anim_length();
        if !claw.blocked {
            claw.current_anim_tick = (claw.current_anim_tick + 1) % anim_length;
        }
        claw.blocked = false;
        if claw.current_anim_tick == 0 {
            // Trying to pick up an item.
            let mut from = containers
                .get_component_mut::<ItemContainer>(claw.take_from)
                .unwrap();
            if let Some(item) = from.try_take() {
                claw.held_item = Some(item);
                *mat = common_assets.claw_mat.1.clone();
            } else {
                claw.blocked = true;
            }
        } else if claw.current_anim_tick == anim_length / 2 {
            if let Ok((mut to, to_pos)) = containers.get_mut(claw.move_to) {
                to.try_put_from(&mut claw.held_item, *to_pos, &mut items);
                claw.blocked = claw.held_item.is_some();
                if !claw.blocked {
                    *mat = common_assets.claw_mat.0.clone();
                }
            }
        }
    }
}

pub(super) fn animate(
    tick_clock: Res<TickClock>,
    mut claws: Query<(&ClawLogic, &mut Transform)>,
    item_containers: Query<(&ItemContainer, &IsoPos)>,
    mut items: Query<(&mut ItemAnimator,)>,
) {
    for (claw, mut transform) in claws.iter_mut() {
        let from = item_containers.get(claw.take_from);
        let to = item_containers.get(claw.move_to);
        if from.is_err() || to.is_err() {
            continue;
        }
        let (from_container, from_pos) = from.unwrap();
        let (to_container, to_pos) = to.unwrap();
        let from_pos = from_container.alignment().get_item_pos(*from_pos);
        let to_pos = to_container.alignment().get_item_pos(*to_pos);
        let anim_length = claw.anim_length();
        let current_tick = claw.current_anim_tick;
        let mut progress = current_tick as f32;
        if !claw.blocked && !cfg!(feature = "no-interpolation") {
            progress += tick_clock.get_tick_progress();
        }
        progress /= anim_length as f32 / 2.0;
        if progress > 1.0 {
            progress = 2.0 - progress;
        }
        let position_now = from_pos.lerp(to_pos, progress);
        transform.translation = (position_now, 0.2).into();
        if let Some(item) = claw.held_item {
            items
                .get_component_mut::<ItemAnimator>(item)
                .unwrap()
                .anim_stationary_exact(position_now);
        }
    }
}
