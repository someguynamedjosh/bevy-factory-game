use bevy::prelude::*;

use super::{animation::ItemAnim, ItemContainerAlignment};
use crate::prelude::*;

pub struct ItemAnimator {
    anim: ItemAnim,
}

impl ItemAnimator {
    pub fn new(start_pos: Vec2) -> Self {
        Self {
            anim: ItemAnim::Stay(start_pos),
        }
    }

    pub fn current_rest_position(&self) -> Vec2 {
        self.anim.rest_position()
    }

    pub fn anim_to_container(
        &mut self,
        pos: IsoPos,
        alignment: ItemContainerAlignment,
        anim_duration: u8,
    ) {
        let target_pos = alignment.get_item_pos(pos);
        self.anim = ItemAnim::new_lerp(
            self.current_rest_position(),
            target_pos,
            anim_duration,
            anim_duration,
        );
    }

    pub fn anim_stationary_in_container(&mut self, pos: IsoPos, alignment: ItemContainerAlignment) {
        self.anim = ItemAnim::Stay(alignment.get_item_pos(pos));
    }

    pub fn anim_stationary_exact(&mut self, pos: Vec2) {
        self.anim = ItemAnim::Stay(pos);
    }
}

pub(super) fn animate_items(
    tick_clock: Res<TickClock>,
    mut items: Query<(&mut Transform, &mut ItemAnimator)>,
) {
    for (mut transform, mut item) in items.iter_mut() {
        let pos = item.anim.evaluate(&*tick_clock);
        transform.translation = (pos, 0.1).into();
    }
}
