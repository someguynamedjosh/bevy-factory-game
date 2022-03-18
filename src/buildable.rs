mod base;

pub mod claw;
pub mod machine_type;
pub mod util;

use bevy::prelude::*;

pub use self::base::*;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugin(util::Plug).add_plugin(claw::Plug);
    }
}
