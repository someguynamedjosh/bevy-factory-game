mod base;
pub mod conveyor;
mod spawn;
mod support;

use bevy::prelude::{App, Plugin};

pub use self::{base::*, spawn::*, support::*};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugin(conveyor::Plug);
    }
}
