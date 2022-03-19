use bevy::prelude::*;
use dyn_clone::DynClone;

use super::{BuildingComponentsContext, BuildingContext, BuildingMaps, WhichMap};
use crate::prelude::*;

#[derive(Component)]
pub struct Built {
    pub buildable: Box<dyn DynBuildable>,
}

pub trait Buildable: DynClone + Sync + Send + 'static {
    type ExtraData;

    fn shape(&self, ctx: &mut BuildingContext) -> Vec<IsoPos>;
    fn maps(&self) -> Vec<WhichMap>;
    fn extra_root_components(&self, ctx: &mut BuildingComponentsContext, data: Self::ExtraData);
    fn spawn_extras(
        &self,
        ctx: &mut BuildingContext,
        maps: &mut BuildingMaps,
    ) -> (Vec<Entity>, Self::ExtraData);
    fn spawn_art(&self, ctx: &mut BuildingContext) -> Vec<Entity>;
}

pub trait DynBuildable: DynClone + Sync + Send + 'static {
    fn shape(&self, ctx: &mut BuildingContext) -> Vec<IsoPos>;
    fn maps(&self) -> Vec<WhichMap>;
    fn spawn_self(
        &self,
        built: Built,
        ctx: &mut BuildingContext,
        maps: &mut BuildingMaps,
    ) -> Entity;
}

impl<B: Buildable> DynBuildable for B {
    fn shape(&self, ctx: &mut BuildingContext) -> Vec<IsoPos> {
        Buildable::shape(self, ctx)
    }

    fn maps(&self) -> Vec<WhichMap> {
        Buildable::maps(self)
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
}
