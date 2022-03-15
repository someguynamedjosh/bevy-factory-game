use bevy::{prelude::*, ecs::schedule::ShouldRun};

use crate::{iso::GRID_TRIANGLE_RADIUS, prelude::*};

/// How big a pixel of a sprite should be.
const _SPRITE_SCALE: f32 = GRID_TRIANGLE_RADIUS / 64.0;
pub const SPRITE_SCALE: Vec3 = Vec3::new(_SPRITE_SCALE, _SPRITE_SCALE, _SPRITE_SCALE);
pub const SPRITE_TRANSFORM: Transform = Transform {
    translation: Vec3::zero(),
    rotation: Quat::identity(),
    scale: SPRITE_SCALE,
};

pub mod fstage {
    pub const UI: &'static str = "factory_ui";
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
    #[cfg(not(feature = "quarter-speed"))]
    const TICK_SPEED: f32 = 60.0 / 360.0;
    #[cfg(feature = "quarter-speed")]
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

fn only_on_tick(tick_clock: Res<TickClock>) -> ShouldRun {
    if tick_clock.is_tick_this_frame() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum TileVariant {
    Blank,
    Input,
    Output,
    Misc,
}

pub fn start_tile<'c>(
    commands: &'c mut Commands,
    common_assets: &CommonAssets,
    pos: IsoPos,
    variant: TileVariant,
) {
    commands
        .spawn()
        .with_bundle(SpriteBundle {
            material: common_assets.tiles[variant as usize].clone(),
            transform: pos.building_transform(Default::default()) * SPRITE_TRANSFORM,
            visible: Visible {
                is_transparent: true,
                is_visible: true,
            },
            ..Default::default()
        })
        .with(pos);
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_stage_after("update", fstage::UI, SystemStage::serial())
            .add_stage_after(fstage::UI, fstage::SETUP, SystemStage::serial())
            .add_stage_after(
                fstage::SETUP,
                fstage::TICK,
                SystemStage::serial().with_run_criteria(only_on_tick.system()),
            )
            .add_stage_after(fstage::TICK, fstage::ANIMATION, SystemStage::serial())
            .add_resource(TickClock::default())
            .add_system_to_stage(fstage::SETUP, update_clock.system());
    }
}
