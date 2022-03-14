mod base;
pub mod conveyor;
mod spawn;
mod support;

use bevy::prelude::{AppBuilder, Plugin};

pub use self::{base::*, spawn::*, support::*};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(conveyor::Plug);
    }
}
