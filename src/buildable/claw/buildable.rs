use bevy::prelude::*;

use super::{Buildable, BuildingComponentsContext, BuildingContext, WhichMap};
use crate::{
    buildable::{claw::logic::ClawLogic, BuildingMaps},
    iso::GRID_EDGE_LENGTH,
    prelude::*,
};

#[derive(Clone)]
pub struct BClaw {
    pub take_from: IsoPos,
}

impl Buildable for BClaw {
    type ExtraData = (Entity, Entity);

    fn shape(&self, ctx: &mut BuildingContext) -> Vec<IsoPos> {
        vec![self.take_from, ctx.position]
    }

    fn maps(&self) -> Vec<WhichMap> {
        vec![]
    }

    fn extra_root_components(
        &self,
        ctx: &mut BuildingComponentsContext,
        (take_from, move_to): Self::ExtraData,
    ) {
        let (start, end) = (ctx.position, self.take_from);
        let distance = start.centroid_pos().distance(end.centroid_pos());
        let distance = distance + 0.01;
        let distance = distance / GRID_EDGE_LENGTH * 2.0;
        assert!(distance > 0.0 && distance < 255.0);
        let length = (distance + 0.3).floor() as u8;
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
                material: ctx.common_assets.claw_mat.clone(),
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
