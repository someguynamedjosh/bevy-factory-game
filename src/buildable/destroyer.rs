use bevy::prelude::*;

use super::{
    storage::ItemList, Buildable, BuildingComponentsContext, BuildingContext, BuildingDetails,
    BuildingMaps, WhichMap,
};
use crate::{
    item::{ItemContainer, ItemContainerAlignment},
    prelude::*,
};

#[derive(Clone, Debug)]
pub struct BDestroyer;

#[derive(Component)]
pub struct DestroyerLogic;

impl Buildable for BDestroyer {
    type ExtraData = ();

    fn details(
        &self,
        position: IsoPos,
        direction: IsoDirection,
        maps: &BuildingMaps,
    ) -> Option<BuildingDetails> {
        Some(BuildingDetails {
            shape: vec![position],
            maps: vec![WhichMap::Buildings, WhichMap::ItemContainers],
            cost: ItemList::new(),
        })
    }

    fn extra_root_components(&self, ctx: &mut BuildingComponentsContext, _data: Self::ExtraData) {
        ctx.commands
            .insert(DestroyerLogic)
            .insert(ItemContainer::new_empty(ItemContainerAlignment::Centroid));
    }

    fn spawn_extras(
        &self,
        _ctx: &mut BuildingContext,
        _maps: &mut BuildingMaps,
    ) -> (Vec<bevy::prelude::Entity>, Self::ExtraData) {
        (vec![], ())
    }

    fn spawn_art(&self, ctx: &mut BuildingContext) -> Vec<bevy::prelude::Entity> {
        vec![ctx
            .commands
            .spawn()
            .insert_bundle(PbrBundle {
                material: ctx.common_assets.destroyer_mat.clone(),
                mesh: ctx.common_assets.quad_mesh.clone(),
                transform: ctx.position.building_transform(Default::default()) * sprite_transform(),
                ..Default::default()
            })
            .id()]
    }
}

fn tick(
    mut commands: Commands,
    mut destroyers: Query<(&mut ItemContainer,), With<DestroyerLogic>>,
) {
    for (mut container,) in destroyers.iter_mut() {
        if container.blocked() {
            continue;
        }
        if let Some(item) = container.try_take() {
            commands.entity(item).despawn();
        }
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(fstage::TICK, tick.system());
    }
}
