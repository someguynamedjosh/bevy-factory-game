use crate::{
    item::{ItemContainer, ItemContainerAlignment},
    prelude::*,
};
use bevy::prelude::*;

/// Defines the visual/physical structure of a machine.
#[derive(Debug)]
pub struct Shape {
    pub blanks: &'static [(i32, i32)],
    pub inputs: &'static [(i32, i32)],
    pub outputs: &'static [(i32, i32)],
}

pub struct BuildingResult {
    pub inputs: Vec<Entity>,
    pub outputs: Vec<Entity>,
    pub origin: Entity,
}

pub fn spawn_building(
    commands: &mut Commands,
    common_assets: &Res<CommonAssets>,
    shape: &Shape,
    origin: IsoPos,
    facing: IsoDirection,
) -> BuildingResult {
    for &(par, perp) in shape.blanks {
        start_tile(
            commands,
            common_assets,
            origin.offset_both_direction(facing, par, perp),
            TileVariant::Blank,
        );
    }

    let mut inputs = Vec::new();
    for &(par, perp) in shape.inputs {
        let id = start_tile(
            commands,
            common_assets,
            origin.offset_both_direction(facing, par, perp),
            TileVariant::Input,
        )
        .with(ItemContainer::new_empty(ItemContainerAlignment::Centroid))
        .current_entity()
        .unwrap();
        inputs.push(id);
    }
    let mut outputs = Vec::new();
    for &(par, perp) in shape.outputs {
        let id = start_tile(
            commands,
            common_assets,
            origin.offset_both_direction(facing, par, perp),
            TileVariant::Output,
        )
        .with(ItemContainer::new_empty(ItemContainerAlignment::Centroid))
        .current_entity()
        .unwrap();
        outputs.push(id);
    }
    let origin = start_tile(commands, common_assets, origin, TileVariant::Misc)
        .current_entity()
        .unwrap();

    BuildingResult {
        inputs,
        outputs,
        origin,
    }
}
