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

pub struct ShapeIters<T> {
    pub blanks: T,
    pub inputs: T,
    pub outputs: T,
}

impl Shape {
    fn positions_impl(
        def: &'static [(i32, i32)],
        origin: IsoPos,
        facing: IsoDirection,
    ) -> impl Iterator<Item = IsoPos> {
        def.iter()
            .map(move |&(par, perp)| origin.offset_both_direction(facing, par, perp))
    }

    pub fn positions(
        &self,
        origin: IsoPos,
        facing: IsoDirection,
    ) -> ShapeIters<impl Iterator<Item = IsoPos>> {
        ShapeIters {
            blanks: Self::positions_impl(self.blanks, origin, facing),
            inputs: Self::positions_impl(self.inputs, origin, facing),
            outputs: Self::positions_impl(self.outputs, origin, facing),
        }
    }
}

pub struct BuildingResult {
    pub inputs: Vec<Entity>,
    pub outputs: Vec<Entity>,
    pub origin: Entity,
}

pub fn spawn_building_with_placeholder_art(
    commands: &mut Commands,
    common_assets: &Res<CommonAssets>,
    obstruction_map: &mut ResMut<BuildingObstructionMap>,
    shape: &Shape,
    origin: IsoPos,
    facing: IsoDirection,
) -> BuildingResult {
    let main_entity = start_tile(commands, common_assets, origin, TileVariant::Misc)
        .current_entity()
        .unwrap();
    obstruction_map.set_empty(origin, main_entity);
    let tile_pos_iters = shape.positions(origin, facing);

    for pos in tile_pos_iters.blanks {
        obstruction_map.set_empty(pos, main_entity);
        start_tile(commands, common_assets, pos, TileVariant::Blank);
    }

    let mut inputs = Vec::new();
    for pos in tile_pos_iters.inputs {
        obstruction_map.set_empty(pos, main_entity);
        let id = start_tile(commands, common_assets, pos, TileVariant::Input)
            .with(ItemContainer::new_empty(ItemContainerAlignment::Centroid))
            .current_entity()
            .unwrap();
        inputs.push(id);
    }
    let mut outputs = Vec::new();
    for pos in tile_pos_iters.outputs {
        obstruction_map.set_empty(pos, main_entity);
        let id = start_tile(commands, common_assets, pos, TileVariant::Output)
            .with(ItemContainer::new_empty(ItemContainerAlignment::Centroid))
            .current_entity()
            .unwrap();
        outputs.push(id);
    }

    BuildingResult {
        inputs,
        outputs,
        origin: main_entity,
    }
}

pub fn spawn_building(
    commands: &mut Commands,
    obstruction_map: &mut ResMut<BuildingObstructionMap>,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    shape: &Shape,
    origin: IsoPos,
    facing: IsoDirection,
) -> BuildingResult {
    let main_entity = commands
        .spawn(PbrBundle {
            mesh,
            material,
            transform: origin.building_transform(facing.axis()),
            ..Default::default()
        })
        .current_entity()
        .unwrap();
    obstruction_map.set_empty(origin, main_entity);
    let tile_pos_iters = shape.positions(origin, facing);

    for pos in tile_pos_iters.blanks {
        obstruction_map.set_empty(pos, main_entity);
    }

    let mut inputs = Vec::new();
    for pos in tile_pos_iters.inputs {
        obstruction_map.set_empty(pos, main_entity);
        let id = commands
            .spawn((
                pos,
                ItemContainer::new_empty(ItemContainerAlignment::Centroid),
            ))
            .current_entity()
            .unwrap();
        inputs.push(id);
    }
    let mut outputs = Vec::new();
    for pos in tile_pos_iters.outputs {
        obstruction_map.set_empty(pos, main_entity);
        let id = commands
            .spawn((
                pos,
                ItemContainer::new_empty(ItemContainerAlignment::Centroid),
            ))
            .current_entity()
            .unwrap();
        outputs.push(id);
    }

    BuildingResult {
        inputs,
        outputs,
        origin: main_entity,
    }
}
