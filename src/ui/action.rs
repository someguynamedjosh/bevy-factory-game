mod execute;
mod ok;
mod update;

use bevy::prelude::*;

pub use self::update::update;
use crate::{
    buildable::{
        claw::BClaw,
        conveyor::BConveyor,
        machine::{BMachine, MachineType},
        Buildable, BuildingContext,
    },
    prelude::*,
};

pub enum Action {
    PlaceConveyor,
    PlaceClawStart,
    PlaceClawEnd { take_from: IsoPos },
    PlaceMachine(MachineType),
    Destroy,
}

pub struct ActionState {
    pub action: Action,
    pub ok: bool,
    preview: Vec<Entity>,
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
            Self::Destroy => Snapping::None,
        }
    }

    pub fn spawn_art(&self, ctx: &mut BuildingContext) -> Vec<Entity> {
        match self {
            Self::PlaceConveyor => BConveyor.spawn_art(ctx),
            Self::PlaceClawStart => BClaw {
                take_from: ctx.position,
            }
            .spawn_art(ctx),
            &Self::PlaceClawEnd { take_from } => BClaw { take_from }.spawn_art(ctx),
            &Self::PlaceMachine(typ) => BMachine(typ).spawn_art(ctx),
            Self::Destroy => vec![],
        }
    }
}

pub fn startup(mut commands: Commands) {
    commands.insert_resource(ActionState {
        action: Action::PlaceConveyor,
        ok: false,
        preview: vec![],
    })
}
