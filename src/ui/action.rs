mod execute;
mod ok;
mod update;

use bevy::prelude::*;

pub use self::update::update;
use crate::{buildable::machine::MachineType, prelude::*};

pub enum Action {
    PlaceConveyor,
    PlaceClawStart,
    PlaceClawEnd { take_from: IsoPos },
    PlaceMachine(MachineType),
}

pub struct ActionState {
    pub action: Action,
    pub ok: bool,
}

impl Action {
    pub fn get_snapping(&self, selected_direction: IsoDirection) -> Snapping {
        match self {
            Self::PlaceConveyor => Snapping::None,
            Self::PlaceClawStart => Snapping::None,
            Self::PlaceClawEnd {
                take_from: start_pos,
                ..
            } => Snapping::AlongAnyLine {
                through: *start_pos,
            },
            Self::PlaceMachine(..) => Snapping::require_edge_pointing_in(selected_direction),
        }
    }
}

pub fn startup(mut commands: Commands) {
    commands.insert_resource(ActionState {
        action: Action::PlaceConveyor,
        ok: false,
    })
}
