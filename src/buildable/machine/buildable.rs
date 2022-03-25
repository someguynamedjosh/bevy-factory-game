use bevy::prelude::*;

use super::{logic::MachineLogic, shape::Shape, typee::MachineType};
use crate::{
    buildable::{
        storage::ItemList, Buildable, BuildingComponentsContext, BuildingContext, BuildingDetails,
        BuildingMaps, WhichMap,
    },
    item::{ItemContainer, ItemContainerAlignment},
    prelude::*,
};

#[derive(Clone, Debug)]
pub struct BMachine(pub MachineType);

pub struct MachineIo {
    inputs: Vec<Entity>,
    outputs: Vec<Entity>,
}

impl Buildable for BMachine {
    type ExtraData = MachineIo;

    fn details(
        &self,
        position: IsoPos,
        direction: IsoDirection,
        maps: &BuildingMaps,
    ) -> Option<BuildingDetails> {
        let shape = self
            .0
            .get_shape()
            .all_positions(position, direction)
            .collect();
        Some(BuildingDetails {
            shape,
            maps: vec![WhichMap::Buildings],
            cost: self.0.get_cost(),
        })
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
            spawn_placeholder_art(ctx, &self.0.get_shape())
        }
    }

    fn on_destroy(&self, ctx: &mut BuildingContext, maps: &mut BuildingMaps) {
        let p = self.0.get_shape().positions(ctx.position, ctx.direction);
        for container in p.inputs.chain(p.outputs) {
            maps.item_containers.clear(container);
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

pub fn spawn_placeholder_art(ctx: &mut BuildingContext, shape: &Shape) -> Vec<Entity> {
    let BuildingContext {
        commands,
        position,
        direction,
        common_assets,
    } = ctx;
    let (position, direction) = (*position, *direction);
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
