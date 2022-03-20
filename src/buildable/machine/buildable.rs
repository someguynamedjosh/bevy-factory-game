use std::iter;

use bevy::prelude::*;

use super::{logic::MachineLogic, shape::Shape, typee::MachineType};
use crate::{
    buildable::{Buildable, BuildingComponentsContext, BuildingContext, BuildingMaps, WhichMap},
    item::{ItemContainer, ItemContainerAlignment},
    prelude::*,
};

#[derive(Clone)]
pub struct BMachine(pub MachineType);

pub struct MachineIo {
    inputs: Vec<Entity>,
    outputs: Vec<Entity>,
}

impl Buildable for BMachine {
    type ExtraData = MachineIo;

    fn shape(&self, ctx: &mut BuildingContext) -> Vec<IsoPos> {
        let p = self.0.get_shape().positions(ctx.position, ctx.direction);
        p.blanks
            .chain(p.inputs)
            .chain(p.outputs)
            .chain(iter::once(ctx.position))
            .collect()
    }

    fn maps(&self) -> Vec<WhichMap> {
        vec![WhichMap::Buildings]
    }

    fn spawn_extras(
        &self,
        ctx: &mut BuildingContext,
        maps: &mut BuildingMaps,
    ) -> (Vec<Entity>, MachineIo) {
        let mut io = MachineIo {
            inputs: vec![],
            outputs: vec![],
        };
        let mut all = Vec::new();
        let p = self.0.get_shape().positions(ctx.position, ctx.direction);
        for pos in p.inputs {
            let ent = ctx
                .commands
                .spawn()
                .insert(pos)
                .insert(ItemContainer::new_empty(ItemContainerAlignment::Centroid))
                .id();
            maps.item_containers.set_assuming_empty(pos, ent);
            io.inputs.push(ent);
            all.push(ent);
        }
        for pos in p.outputs {
            let ent = ctx
                .commands
                .spawn()
                .insert(pos)
                .insert(ItemContainer::new_empty(ItemContainerAlignment::Centroid))
                .id();
            maps.item_containers.set_assuming_empty(pos, ent);
            io.outputs.push(ent);
            all.push(ent);
        }
        (all, io)
    }

    fn extra_root_components(
        &self,
        ctx: &mut BuildingComponentsContext,
        MachineIo { inputs, outputs }: MachineIo,
    ) {
        ctx.commands
            .insert(MachineLogic::new(inputs, outputs, self.0));
    }

    fn spawn_art(&self, ctx: &mut BuildingContext) -> Vec<Entity> {
        if let Some((mesh, mat)) = self.0.get_appearence(&ctx.common_assets) {
            vec![spawn_bespoke_art(ctx.commands, mesh, mat)]
        } else {
            spawn_placeholder_art(
                ctx.commands,
                ctx.common_assets,
                &self.0.get_shape(),
                ctx.position,
                ctx.direction,
            )
        }
    }
}

fn spawn_bespoke_art(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
) -> Entity {
    commands
        .spawn()
        .insert_bundle(PbrBundle {
            mesh,
            material,
            ..Default::default()
        })
        .id()
}

fn spawn_placeholder_art(
    commands: &mut Commands,
    common_assets: &CommonAssets,
    shape: &Shape,
    position: IsoPos,
    direction: IsoDirection,
) -> Vec<Entity> {
    let mut all = Vec::new();
    let p = shape.positions(position, direction);
    for pos in p.blanks {
        all.push(start_tile(commands, common_assets, pos, TileVariant::Blank).id());
    }
    for pos in p.inputs {
        all.push(start_tile(commands, common_assets, pos, TileVariant::Input).id());
    }
    for pos in p.outputs {
        all.push(start_tile(commands, common_assets, pos, TileVariant::Output).id());
    }
    all.push(start_tile(commands, common_assets, position, TileVariant::Misc).id());
    all
}
