mod buildable;
mod logic;
mod shape;
mod typee;

use bevy::prelude::{App, Plugin};

pub use self::{
    buildable::{spawn_placeholder_art, BMachine},
    shape::*,
    typee::*,
};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugin(logic::Plug);
    }
}
