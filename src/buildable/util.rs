use bevy::prelude::*;

use crate::{
    item::{spawn_item, ItemContainer, ItemContainerAlignment, ReferenceItem},
    prelude::*,
};

#[derive(Component)]
pub struct DebugSpawner {
    rate: u8,
    spawn_cycle: u8,
}

#[derive(Component)]
pub struct DebugDestroyer;

pub fn spawn_spawner(
    commands: &mut Commands,
    common_assets: &Res<CommonAssets>,
    origin: IsoPos,
    rate: u8,
) -> Entity {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: common_assets.spawner_mat.clone(),
            transform: origin.building_transform(Default::default()) * sprite_transform(),
            ..Default::default()
        })
        .insert(origin)
        .insert(DebugSpawner {
            rate,
            spawn_cycle: 0,
        })
        .insert(ItemContainer::new_empty(ItemContainerAlignment::Centroid))
        .id()
}

pub fn spawn_destroyer(
    commands: &mut Commands,
    common_assets: &Res<CommonAssets>,
    origin: IsoPos,
) -> Entity {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: common_assets.destroyer_mat.clone(),
            transform: origin.building_transform(Default::default()) * sprite_transform(),
            ..Default::default()
        })
        .insert(origin)
        .insert(DebugDestroyer)
        .insert(ItemContainer::new_empty(ItemContainerAlignment::Centroid))
        .id()
}

fn tick_spawners(
    mut commands: Commands,
    common_assets: Res<CommonAssets>,
    mut spawners: Query<(&mut DebugSpawner, &mut ItemContainer, &IsoPos)>,
) {
    for (mut spawner, mut container, pos) in spawners.iter_mut() {
        if container.item().is_none() {
            spawner.spawn_cycle += 1;
            if spawner.spawn_cycle >= spawner.rate {
                spawner.spawn_cycle = 0;
                let item = spawn_item(
                    &mut commands,
                    &common_assets,
                    ReferenceItem::IronOre.as_item(),
                    *pos,
                    ItemContainerAlignment::Centroid,
                );
                container.put_item(item);
                container.set_blocked(false);
            }
        }
    }
}

fn tick_destroyers(
    mut commands: Commands,
    mut destroyers: Query<(&mut ItemContainer,), With<DebugDestroyer>>,
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
        app.add_system_to_stage(fstage::TICK, tick_spawners.system())
            .add_system_to_stage(fstage::TICK, tick_destroyers.system());
    }
}
