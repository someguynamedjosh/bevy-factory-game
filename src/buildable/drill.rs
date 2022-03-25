use bevy::prelude::*;
use maplit::hashmap;

use super::{
    machine::{self, Shape},
    storage::ItemList,
    Buildable, BuildingComponentsContext, BuildingContext, BuildingDetails, BuildingMaps, WhichMap,
};
use crate::{
    item::{ItemContainer, ItemContainerAlignment, ReferenceItem},
    prelude::*,
    resource_nodes::ResourceNode,
};

#[derive(Component)]
pub struct Drill {
    node: ResourceNode,
    outputs: Vec<Entity>,
    timer: u8,
}

#[derive(Clone, Debug)]
pub struct BDrill;

const SHAPE: Shape = Shape {
    blanks: &[
        (1, 1),
        (1, -1),
        (1, 3),
        (1, -3),
        (0, 3),
        (0, 2),
        (0, 1),
        (0, -1),
        (0, -2),
        (0, -3),
    ],
    inputs: &[],
    outputs: &[(1, 0), (1, 2), (1, -2)],
};

impl Buildable for BDrill {
    type ExtraData = (Vec<Entity>, ResourceNode);

    fn details(
        &self,
        position: IsoPos,
        direction: IsoDirection,
        maps: &BuildingMaps,
    ) -> Option<BuildingDetails> {
        if maps.resource_nodes.get(position).is_some() {
            Some(BuildingDetails {
                shape: SHAPE.all_positions(position, direction).collect(),
                maps: vec![WhichMap::Buildings],
                cost: ItemList::from_counts(hashmap![
                    ReferenceItem::IronLump.as_item() => 10,
                    ReferenceItem::PureAnimus.as_item() => 1,
                ]),
            })
        } else {
            None
        }
    }

    fn extra_root_components(&self, ctx: &mut BuildingComponentsContext, data: Self::ExtraData) {
        ctx.commands.insert(Drill {
            node: data.1,
            outputs: data.0,
            timer: 0,
        });
    }

    fn spawn_extras(
        &self,
        ctx: &mut BuildingContext,
        maps: &mut BuildingMaps,
    ) -> (Vec<bevy::prelude::Entity>, Self::ExtraData) {
        let mut inputs = Vec::new();
        for pos in SHAPE.positions(ctx.position, ctx.direction).inputs {
            let container = ctx
                .commands
                .spawn()
                .insert(ItemContainer::new_empty(ItemContainerAlignment::Centroid))
                .insert(pos)
                .id();
            maps.item_containers.set(pos, container);
            inputs.push(container);
        }
        let node = maps.resource_nodes.get(ctx.position).unwrap().clone();
        (inputs.clone(), (inputs, node))
    }

    fn spawn_art(&self, ctx: &mut BuildingContext) -> Vec<bevy::prelude::Entity> {
        machine::spawn_placeholder_art(ctx, &SHAPE)
    }
}

fn tick(
    mut commands: Commands,
    common_assets: Res<CommonAssets>,
    mut drills: Query<(&mut Drill,)>,
    mut containers: Query<(&IsoPos, &mut ItemContainer)>,
) {
    for (mut drill,) in drills.iter_mut() {
        let drill = &mut *drill;
        for &output in &drill.outputs {
            let (pos, mut container) = containers.get_mut(output).unwrap();
            if container.item().is_none() {
                container.create_and_put_item(
                    &mut commands,
                    &common_assets,
                    *pos,
                    drill.node.of.as_item(),
                );
            }
        }
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(fstage::TICK, tick);
    }
}
