use bevy::prelude::*;

pub mod fstage {
    pub const SETUP: &'static str = "factory_setup";
    pub const TICK: &'static str = "factory_tick";
    pub const ANIMATION: &'static str = "factory_animation";
}

pub struct SetupNeeded;

#[derive(Default)]
pub struct TickClock {
    tick_progress: f32,
    tick_this_frame: bool,
}

impl TickClock {
    #[cfg(not(feature="quarter-speed"))]
    const TICK_SPEED: f32 = 60.0 / 360.0;
    #[cfg(feature="quarter-speed")]
    const TICK_SPEED: f32 = 60.0 / 360.0 * 4.0;

    pub fn get_tick_progress(&self) -> f32 {
        self.tick_progress / Self::TICK_SPEED
    }

    pub fn is_tick_this_frame(&self) -> bool {
        self.tick_this_frame
    }

    fn advance(&mut self, dt: f32) {
        self.tick_progress += dt;
        self.tick_this_frame = self.tick_progress >= Self::TICK_SPEED;
        self.tick_progress %= Self::TICK_SPEED;
    }
}

fn update_clock(time: Res<Time>, mut tick_clock: ResMut<TickClock>) {
    tick_clock.advance(time.delta_seconds());
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_after(stage::UPDATE, fstage::SETUP, SystemStage::serial())
            .add_stage_after(fstage::SETUP, fstage::TICK, SystemStage::serial())
            .add_stage_after(fstage::TICK, fstage::ANIMATION, SystemStage::serial())
            .add_resource(TickClock::default())
            .add_system_to_stage(fstage::SETUP, update_clock.system());
    }
}
