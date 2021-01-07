use crate::item::{Item, ItemHolder};
use crate::prelude::*;
use bevy::prelude::*;

pub struct Claw {
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

impl Claw {
    fn anim_length(&self) -> u8 {
        // A 1 length claw has to traverse 4 segments, 2 length 6 segments, 3/8, etc.
        (self.length + 1) * 2 * SEGMENT_DURATION
    }
}

pub fn spawn_claw(
    commands: &mut Commands,
    common_assets: &Res<CommonAssets>,
    from: Entity,
    to: Entity,
    length: u8,
) -> Entity {
    commands
        .spawn(SpriteBundle {
            material: common_assets.claw_mat.clone(),
            ..Default::default()
        })
        .with(Claw {
            // We can't guarantee that there is an item ready to pick up when we spawn.
            blocked: true,
            current_anim_tick: 0,
            held_item: None,
            take_from: from,
            move_to: to,
            length,
        })
        .current_entity()
        .unwrap()
}

fn tick(
    tick_clock: Res<TickClock>,
    mut claws: Query<(&mut Claw,)>,
    mut holders: Query<(&mut ItemHolder,)>,
) {
    if !tick_clock.is_tick_this_frame() {
        return;
    }

    for (mut claw,) in claws.iter_mut() {
        let anim_length = claw.anim_length();
        if !claw.blocked {
            claw.current_anim_tick = (claw.current_anim_tick + 1) % anim_length;
        }
        claw.blocked = false;
        if claw.current_anim_tick == 0 {
            // Trying to pick up an item.
            let mut from = holders
                .get_component_mut::<ItemHolder>(claw.take_from)
                .unwrap();
            if let Some(item) = from.try_take() {
                claw.held_item = Some(item);
            } else {
                claw.blocked = true;
            }
        } else if claw.current_anim_tick == anim_length / 2 {
            let mut to = holders
                .get_component_mut::<ItemHolder>(claw.move_to)
                .unwrap();
            to.try_put_from(&mut claw.held_item);
            claw.blocked = claw.held_item.is_some();
        }
    }
}

fn animate(
    tick_clock: Res<TickClock>,
    mut claws: Query<(&Claw, &mut Transform)>,
    item_holders: Query<(&ItemHolder, &IsoPos)>,
    mut items: Query<(&mut Item,)>,
) {
    for (claw, mut transform) in claws.iter_mut() {
        let (from_holder, from_pos) = item_holders.get(claw.take_from).unwrap();
        let (to_holder, to_pos) = item_holders.get(claw.move_to).unwrap();
        let from_pos = from_holder.alignment.get_item_pos(*from_pos);
        let to_pos = to_holder.alignment.get_item_pos(*to_pos);
        let anim_length = claw.anim_length();
        let current_tick = claw.current_anim_tick;
        let mut progress = current_tick as f32;
        if !claw.blocked {
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
                .get_component_mut::<Item>(item)
                .unwrap()
                .anim_stationary_exact(position_now);
        }
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(MAIN_STAGE, tick.system())
            .add_system_to_stage(MAIN_STAGE, animate.system());
    }
}
