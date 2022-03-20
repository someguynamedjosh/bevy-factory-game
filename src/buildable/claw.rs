mod buildable;
mod logic;

use bevy::prelude::*;

pub use self::buildable::BClaw;
use super::{Buildable, BuildingComponentsContext, BuildingContext, WhichMap};
use crate::prelude::*;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(fstage::TICK, logic::tick.system())
            .add_system_to_stage(fstage::ANIMATION, logic::animate.system());
    }
}
