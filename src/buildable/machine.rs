mod buildable;
mod logic;
mod shape;
mod typee;

use bevy::prelude::{App, Plugin};

pub use self::{buildable::BMachine, typee::*};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugin(logic::Plug);
    }
}
