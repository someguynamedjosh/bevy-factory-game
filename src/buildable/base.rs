use std::fmt::Debug;

use bevy::prelude::*;
use dyn_clone::DynClone;

use super::{BuildingComponentsContext, BuildingContext, BuildingMaps, WhichMap, storage::ItemList};
use crate::prelude::*;

#[derive(Component)]
pub struct Built {
    pub buildable: Box<dyn DynBuildable>,
    pub position: IsoPos,
    pub direction: IsoDirection,
}

pub trait Buildable: Debug + DynClone + Sync + Send + 'static {
    type ExtraData;

    fn shape(&self, position: IsoPos, direction: IsoDirection) -> Vec<IsoPos>;
    fn maps(&self) -> Vec<WhichMap>;
    fn cost(&self, position: IsoPos) -> ItemList;

    fn extra_root_components(&self, ctx: &mut BuildingComponentsContext, data: Self::ExtraData);
    fn spawn_extras(
        &self,
        ctx: &mut BuildingContext,
        maps: &mut BuildingMaps,
    ) -> (Vec<Entity>, Self::ExtraData);
    fn spawn_art(&self, ctx: &mut BuildingContext) -> Vec<Entity>;
    #[allow(unused_variables)]
    fn on_destroy(&self, ctx: &mut BuildingContext, maps: &mut BuildingMaps) {}
}

pub trait DynBuildable: Debug + DynClone + Sync + Send + 'static {
    fn shape(&self, position: IsoPos, direction: IsoDirection) -> Vec<IsoPos>;
    fn maps(&self) -> Vec<WhichMap>;
    fn cost(&self, position: IsoPos) -> ItemList;

    fn spawn_self(
        &self,
        built: Built,
        ctx: &mut BuildingContext,
        maps: &mut BuildingMaps,
    ) -> Entity;
    fn dyn_spawn_art(&self, ctx: &mut BuildingContext) -> Vec<Entity>;
    fn on_destroy(&self, ctx: &mut BuildingContext, maps: &mut BuildingMaps);
}

impl<B: Buildable> DynBuildable for B {
    fn shape(&self, position: IsoPos, direction: IsoDirection) -> Vec<IsoPos> {
        Buildable::shape(self, position, direction)
    }

    fn maps(&self) -> Vec<WhichMap> {
        Buildable::maps(self)
    }

    fn cost(&self, position: IsoPos) -> ItemList {
        Buildable::cost(self, position)
    }

    fn spawn_self(
        &self,
        built: Built,
        ctx: &mut BuildingContext,
        maps: &mut BuildingMaps,
    ) -> Entity {
        let root = ctx
            .commands
            .spawn()
            .insert(ctx.position)
            .insert(ctx.direction)
            .insert(built)
            .insert(Transform::identity())
            .insert(GlobalTransform::default())
            .id();
        let (extras, data) = self.spawn_extras(ctx, maps);
        let art = self.spawn_art(ctx);
        for child in extras.into_iter().chain(art.into_iter()) {
            ctx.commands.entity(root).add_child(child);
        }
        {
            let mut commands = ctx.commands.entity(root);
            let mut ctx = BuildingComponentsContext {
                commands: &mut commands,
                position: ctx.position,
                direction: ctx.direction,
                common_assets: ctx.common_assets,
            };
            self.extra_root_components(&mut ctx, data);
        }
        root
    }

    fn dyn_spawn_art(&self, ctx: &mut BuildingContext) -> Vec<Entity> {
        Buildable::spawn_art(self, ctx)
    }

    fn on_destroy(&self, ctx: &mut BuildingContext, maps: &mut BuildingMaps) {
        Buildable::on_destroy(self, ctx, maps)
    }
}
