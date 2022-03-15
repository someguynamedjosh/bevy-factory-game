mod axis;
mod constants;
mod direction;
mod points;
mod pos;
mod snapping;
mod spatial_map;
mod tests;

use bevy::prelude::{App, Plugin};

pub use self::{
    axis::*, constants::*, direction::*, points::*, pos::*, snapping::*, spatial_map::*,
};
use crate::prelude::*;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugin(spatial_map::Plug);
    }
}
