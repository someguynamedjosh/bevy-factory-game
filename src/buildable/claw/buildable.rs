use bevy::prelude::*;
use maplit::hashmap;

use super::{Buildable, BuildingComponentsContext, BuildingContext, WhichMap};
use crate::{
    buildable::{claw::logic::ClawLogic, storage::ItemList, BuildingDetails, BuildingMaps},
    iso::GRID_EDGE_LENGTH,
    item::ReferenceItem,
    prelude::*,
};

#[derive(Clone, Debug)]
pub struct BClaw {
    pub take_from: IsoPos,
}

fn distance(start: IsoPos, end: IsoPos) -> u8 {
    let distance = start.centroid_pos().distance(end.centroid_pos());
    let distance = distance + 0.01;
    let distance = distance / GRID_EDGE_LENGTH * 2.0;
    assert!(distance >= 0.0 && distance < 256.0);
    (distance + 0.3).floor() as u8
}

impl Buildable for BClaw {
    type ExtraData = (Entity, Entity);

    fn details(
        &self,
        position: IsoPos,
        direction: IsoDirection,
        maps: &BuildingMaps,
    ) -> Option<BuildingDetails> {
        let length = distance(self.take_from, position) as u32;
        Some(BuildingDetails {
            shape: vec![self.take_from, position],
            maps: vec![WhichMap::Claws],
            cost: ItemList::from_counts(hashmap![
                ReferenceItem::IronLump.as_item() => 1 + length,
                ReferenceItem::PureAnimus.as_item() => 3,
            ]),
        })
    }

    fn extra_root_components(
        &self,
        ctx: &mut BuildingComponentsContext,
        (take_from, move_to): Self::ExtraData,
    ) {
        let length = distance(self.take_from, ctx.position);
        assert!(length >= 1);
        ctx.commands
            .insert(ClawLogic {
                take_from,
                move_to,
                held_item: None,
                length,
                current_anim_tick: 0,
                blocked: true,
            })
            .insert_bundle(PbrBundle {
                material: ctx.common_assets.claw_mat.0.clone(),
                mesh: ctx.common_assets.quad_mesh.clone(),
                transform: sprite_transform(),
                ..Default::default()
            });
    }

    fn spawn_extras(
        &self,
        ctx: &mut BuildingContext,
        maps: &mut BuildingMaps,
    ) -> (Vec<Entity>, Self::ExtraData) {
        let take_from = *maps.item_containers.get(self.take_from).unwrap();
        let move_to = *maps.item_containers.get(ctx.position).unwrap();
        (vec![], (take_from, move_to))
    }

    fn spawn_art(&self, _ctx: &mut BuildingContext) -> Vec<Entity> {
        vec![]
    }
}
