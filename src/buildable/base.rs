use bevy::prelude::*;

use crate::{
    item::{ItemContainer, ItemContainerAlignment},
    prelude::*,
};

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
            .map(move |&(perp, par)| origin.offset_both_direction(facing, par, perp))
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
    pub art: Vec<Entity>,
}

#[scones::make_constructor(pub start)]
pub struct BuildingSpawner<'a, 'c1, 'c2> {
    commands: &'a mut Commands<'c1, 'c2>,

    #[value(None for start)]
    common_assets: Option<&'a CommonAssets>,
    #[value(None for start)]
    mesh: Option<Handle<Mesh>>,
    #[value(None for start)]
    material: Option<Handle<StandardMaterial>>,

    obstruction_map: &'a mut BuildingObstructionMap,
    shape: &'a Shape,
    origin: IsoPos,
    facing: IsoDirection,
}

impl<'a, 'c1, 'c2> BuildingSpawner<'a, 'c1, 'c2> {
    pub fn with_bespoke_art(self, mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        Self {
            mesh: Some(mesh),
            material: Some(material),
            ..self
        }
    }

    pub fn with_placeholder_art(self, common_assets: &'a CommonAssets) -> Self {
        Self {
            common_assets: Some(common_assets),
            ..self
        }
    }

    pub fn finish(mut self) -> BuildingResult {
        let mut art = Vec::new();
        let main_entity = self.create_main(&mut art);
        self.create_blanks(main_entity, &mut art);
        let inputs = self.create_inputs(main_entity, &mut art);
        let outputs = self.create_outputs(main_entity, &mut art);

        BuildingResult {
            inputs,
            outputs,
            origin: main_entity,
            art,
        }
    }

    fn positions(&self) -> ShapeIters<impl Iterator<Item = IsoPos>> {
        self.shape.positions(self.origin, self.facing)
    }

    fn create_main(&mut self, art: &mut Vec<Entity>) -> Entity {
        let main_entity = self.commands.spawn().current_entity().unwrap();
        self.obstruction_map
            .set_assuming_empty(self.origin, main_entity);
        if let (Some(mesh), Some(material)) = (self.mesh.take(), self.material.take()) {
            self.spawn_bespoke_art(mesh, material, art, main_entity);
        } else {
            self.maybe_spawn_placeholder_art(self.origin, TileVariant::Misc, art, main_entity);
        }
        main_entity
    }

    fn spawn_bespoke_art(
        &mut self,
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
        art: &mut Vec<Entity>,
        main_entity: Entity,
    ) {
        let main_art = self
            .commands
            .spawn()
            .with_bundle(PbrBundle {
                mesh,
                material,
                transform: self.origin.building_transform(self.facing.axis()),
                ..Default::default()
            })
            .with(Parent(main_entity))
            .current_entity()
            .unwrap();
        art.push(main_art);
    }

    fn create_blanks(&mut self, main_entity: Entity, art: &mut Vec<Entity>) {
        for pos in self.positions().blanks {
            self.obstruction_map.set_assuming_empty(pos, main_entity);
            self.maybe_spawn_placeholder_art(pos, TileVariant::Blank, art, main_entity);
        }
    }

    fn create_inputs(&mut self, main_entity: Entity, art: &mut Vec<Entity>) -> Vec<Entity> {
        let mut inputs = Vec::new();
        for pos in self.positions().inputs {
            self.obstruction_map.set_assuming_empty(pos, main_entity);
            let id = self.spawn_empty_item_container(pos, main_entity);
            inputs.push(id);
            self.maybe_spawn_placeholder_art(pos, TileVariant::Input, art, main_entity);
        }
        inputs
    }

    fn create_outputs(&mut self, main_entity: Entity, art: &mut Vec<Entity>) -> Vec<Entity> {
        let mut outputs = Vec::new();
        for pos in self.positions().outputs {
            self.obstruction_map.set_assuming_empty(pos, main_entity);
            let id = self.spawn_empty_item_container(pos, main_entity);
            outputs.push(id);
            self.maybe_spawn_placeholder_art(pos, TileVariant::Output, art, main_entity);
        }
        outputs
    }

    fn maybe_spawn_placeholder_art(
        &mut self,
        pos: IsoPos,
        variant: TileVariant,
        art: &mut Vec<Entity>,
        main_entity: Entity,
    ) {
        if let Some(ca) = self.common_assets {
            let ent = start_tile(self.commands, ca, pos, variant)
                .with(Parent(main_entity))
                .current_entity()
                .unwrap();
            art.push(ent);
        }
    }

    fn spawn_empty_item_container(&mut self, pos: IsoPos, main_entity: Entity) -> Entity {
        let id = self
            .commands
            .spawn()
            .with_bundle((
                pos,
                ItemContainer::new_empty(ItemContainerAlignment::Centroid),
            ))
            .with(Parent(main_entity))
            .current_entity()
            .unwrap();
        id
    }
}
