mod base;

pub mod claw;

use bevy::prelude::*;

pub use self::base::*;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugin(claw::Plug);
    }
}
