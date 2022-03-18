mod base;
mod conveyor;
mod machine;
mod spawn;
mod support;

use bevy::prelude::{App, Plugin};

pub use self::{base::*, conveyor::*, machine::*, spawn::*, support::*};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugin(conveyor::Plug).add_plugin(machine::Plug);
    }
}
