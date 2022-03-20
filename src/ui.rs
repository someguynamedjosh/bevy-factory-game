mod action;
mod camera;
mod cursor;
mod tooltip;

use bevy::prelude::*;

use crate::prelude::*;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_startup_system(action::startup.system())
            .add_startup_system(camera::startup.system())
            .add_startup_system(cursor::startup.system())
            .add_startup_system(tooltip::startup.system())
            .add_system_to_stage(fstage::UI_PRE, cursor::update_pre.system())
            .add_system_to_stage(fstage::UI, action::update.system())
            .add_system_to_stage(fstage::UI, camera::update.system())
            .add_system_to_stage(fstage::UI, tooltip::update.system())
            .add_system_to_stage(fstage::UI_POST, cursor::update_post.system());
    }
}
