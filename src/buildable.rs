mod base;
pub mod claw;
pub mod conveyor;
pub mod destroyer;
pub mod machine;
mod spawn;
pub mod spawner;
mod support;
pub mod storage;
pub mod drill;

use bevy::prelude::{App, Plugin};

pub use self::{base::*, spawn::*, support::*};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugin(claw::Plug)
            .add_plugin(conveyor::Plug)
            .add_plugin(destroyer::Plug)
            .add_plugin(machine::Plug)
            .add_plugin(spawner::Plug)
            .add_plugin(storage::Plug);
    }
}
