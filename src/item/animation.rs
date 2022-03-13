use bevy::prelude::*;

use crate::prelude::*;

pub(super) enum ItemAnim {
    Stay(Vec2),
    Lerp(LerpAnim),
}

impl ItemAnim {
    pub fn new_lerp(from: Vec2, to: Vec2, total_ticks: u8, remaining_ticks: u8) -> Self {
        Self::Lerp(LerpAnim {
            from,
            to,
            total_ticks,
            remaining_ticks,
        })
    }

    pub fn evaluate(&mut self, tick_clock: &TickClock) -> Vec2 {
        match self {
            ItemAnim::Stay(pos) => pos.clone(),
            ItemAnim::Lerp(anim) => anim.evaluate(tick_clock),
        }
    }

    pub fn rest_position(&self) -> Vec2 {
        match self {
            ItemAnim::Stay(pos) => pos.clone(),
            ItemAnim::Lerp(LerpAnim { to, .. }) => to.clone(),
        }
    }
}

pub(super) struct LerpAnim {
    from: Vec2,
    to: Vec2,
    total_ticks: u8,
    remaining_ticks: u8,
}

impl LerpAnim {
    fn evaluate(&mut self, tick_clock: &TickClock) -> Vec2 {
        self.maybe_advance_remaining_ticks(tick_clock);
        let progress = self.progress(tick_clock);
        self.from.lerp(self.to, progress)
    }

    fn maybe_advance_remaining_ticks(&mut self, tick_clock: &TickClock) {
        if tick_clock.is_tick_this_frame() && self.remaining_ticks > 0 {
            self.remaining_ticks -= 1;
        }
    }

    fn progress(&mut self, tick_clock: &TickClock) -> f32 {
        let current_anim_tick = self.current_anim_tick() as f32;
        let sub_tick_progress = tick_clock.get_tick_progress();
        (current_anim_tick + sub_tick_progress) / self.total_ticks as f32
    }

    fn current_anim_tick(&mut self) -> u8 {
        self.total_ticks - self.remaining_ticks - 1
    }
}
