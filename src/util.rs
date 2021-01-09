use crate::item::{ItemContainer, ItemContainerAlignment};
use crate::prelude::*;
use bevy::prelude::*;

pub struct DebugSpawner {
    rate: u8,
    spawn_cycle: u8,
}
pub struct DebugDestroyer;

pub fn spawn_spawner(
    commands: &mut Commands,
    common_assets: &Res<CommonAssets>,
    origin: IsoPos,
    rate: u8,
) -> Entity {
    commands
        .spawn(SpriteBundle {
            material: common_assets.spawner_mat.clone(),
            transform: origin.building_transform(Default::default()),
            ..Default::default()
        })
        .with(origin)
        .with(DebugSpawner {
            rate,
            spawn_cycle: 0,
        })
        .with(ItemContainer::new_empty(ItemContainerAlignment::Centroid))
        .current_entity()
        .unwrap()
}

pub fn spawn_destroyer(
    commands: &mut Commands,
    common_assets: &Res<CommonAssets>,
    origin: IsoPos,
) -> Entity {
    commands
        .spawn(SpriteBundle {
            material: common_assets.destroyer_mat.clone(),
            transform: origin.building_transform(Default::default()),
            ..Default::default()
        })
        .with(origin)
        .with(DebugDestroyer)
        .with(ItemContainer::new_empty(ItemContainerAlignment::Centroid))
        .current_entity()
        .unwrap()
}

fn tick_spawners(
    commands: &mut Commands,
    common_assets: Res<CommonAssets>,
    mut spawners: Query<(&mut DebugSpawner, &mut ItemContainer, &IsoPos)>,
) {
    for (mut spawner, mut container, pos) in spawners.iter_mut() {
        if container.item.is_none() {
            spawner.spawn_cycle += 1;
            if spawner.spawn_cycle >= spawner.rate {
                spawner.spawn_cycle = 0;
                let item = spawn::item(
                    commands,
                    &common_assets,
                    Item::MetalRubble,
                    *pos,
                    ItemContainerAlignment::Centroid,
                );
                container.item = Some(item);
                container.blocked = false;
            }
        }
    }
}

fn tick_destroyers(
    commands: &mut Commands,
    mut destroyers: Query<(&mut ItemContainer,), With<DebugDestroyer>>,
) {
    for (mut container,) in destroyers.iter_mut() {
        if container.blocked {
            continue;
        }
        if let Some(item) = container.item.take() {
            commands.despawn(item);
        }
    }
}
pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(fstage::TICK, tick_spawners.system())
            .add_system_to_stage(fstage::TICK, tick_destroyers.system());
    }
}
