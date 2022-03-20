use bevy::prelude::*;

use super::{Buildable, BuildingComponentsContext, BuildingContext, WhichMap};
use crate::{
    iso::GRID_EDGE_LENGTH,
    item::{ItemAnimator, ItemContainer},
    prelude::*,
};

#[derive(Clone)]
pub struct BClaw {
    pub take_from: IsoPos,
}

impl Buildable for BClaw {
    type ExtraData = (Entity, Entity);

    fn shape(&self, ctx: &mut BuildingContext) -> Vec<IsoPos> {
        vec![self.take_from, ctx.position]
    }

    fn maps(&self) -> Vec<WhichMap> {
        vec![]
    }

    fn extra_root_components(
        &self,
        ctx: &mut BuildingComponentsContext,
        (take_from, move_to): Self::ExtraData,
    ) {
        let (start, end) = (ctx.position, self.take_from);
        let distance = start.centroid_pos().distance(end.centroid_pos());
        let distance = distance + 0.01;
        let distance = distance / GRID_EDGE_LENGTH * 2.0;
        assert!(distance > 0.0 && distance < 255.0);
        let length = (distance + 0.3).floor() as u8;
        assert!(length >= 1);
        ctx.commands
            .insert(ClawLogic {
                take_from,
                move_to,
                held_item: None,
                length,
                current_anim_tick: 0,
                blocked: true,
            })
            .insert_bundle(PbrBundle {
                material: ctx.common_assets.claw_mat.clone(),
                mesh: ctx.common_assets.quad_mesh.clone(),
                transform: sprite_transform(),
                ..Default::default()
            });
    }

    fn spawn_extras(
        &self,
        ctx: &mut BuildingContext,
        maps: &mut super::BuildingMaps,
    ) -> (Vec<Entity>, Self::ExtraData) {
        let take_from = *maps.item_containers.get(self.take_from).unwrap();
        let move_to = *maps.item_containers.get(ctx.position).unwrap();
        (vec![], (take_from, move_to))
    }

    fn spawn_art(&self, _ctx: &mut BuildingContext) -> Vec<Entity> {
        vec![]
    }
}

#[derive(Component)]
pub struct ClawLogic {
    take_from: Entity,
    move_to: Entity,
    held_item: Option<Entity>,
    /// Length of the gantry in grid cells.
    length: u8,
    current_anim_tick: u8,
    blocked: bool,
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

fn tick(
    mut claws: Query<(&mut ClawLogic,)>,
    mut containers: Query<(&mut ItemContainer, &IsoPos)>,
    mut items: Query<&mut ItemAnimator>,
) {
    for (mut claw,) in claws.iter_mut() {
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
            } else {
                claw.blocked = true;
            }
        } else if claw.current_anim_tick == anim_length / 2 {
            let (mut to, to_pos) = containers.get_mut(claw.move_to).unwrap();
            to.try_put_from(&mut claw.held_item, *to_pos, &mut items);
            claw.blocked = claw.held_item.is_some();
        }
    }
}

fn animate(
    tick_clock: Res<TickClock>,
    mut claws: Query<(&ClawLogic, &mut Transform)>,
    item_containers: Query<(&ItemContainer, &IsoPos)>,
    mut items: Query<(&mut ItemAnimator,)>,
) {
    for (claw, mut transform) in claws.iter_mut() {
        let (from_container, from_pos) = item_containers.get(claw.take_from).unwrap();
        let (to_container, to_pos) = item_containers.get(claw.move_to).unwrap();
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

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(fstage::TICK, tick.system())
            .add_system_to_stage(fstage::ANIMATION, animate.system());
    }
}
