use bevy::prelude::*;

use super::{storage::ItemList, Buildable, BuildingDetails, BuildingMaps};
use crate::{
    buildable::WhichMap,
    item::{spawn_item, ItemContainer, ItemContainerAlignment, ReferenceItem},
    prelude::*,
};

#[derive(Clone, Debug)]
pub struct BSpawner {
    pub interval: u8,
    pub item: ReferenceItem,
}

#[derive(Component)]
pub struct SpawnerLogic {
    item: ReferenceItem,
    interval: u8,
    /// Tracks how many ticks until the next item is spawned.
    timer: u8,
}

impl Buildable for BSpawner {
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

    fn extra_root_components(
        &self,
        ctx: &mut super::BuildingComponentsContext,
        _data: Self::ExtraData,
    ) {
        ctx.commands
            .insert(SpawnerLogic {
                item: self.item,
                interval: self.interval,
                timer: 0,
            })
            .insert(ItemContainer::new_empty(ItemContainerAlignment::Centroid));
    }

    fn spawn_extras(
        &self,
        _ctx: &mut super::BuildingContext,
        _maps: &mut super::BuildingMaps,
    ) -> (Vec<bevy::prelude::Entity>, Self::ExtraData) {
        (vec![], ())
    }

    fn spawn_art(&self, ctx: &mut super::BuildingContext) -> Vec<bevy::prelude::Entity> {
        vec![ctx
            .commands
            .spawn()
            .insert_bundle(PbrBundle {
                material: ctx.common_assets.spawner_mat.clone(),
                mesh: ctx.common_assets.quad_mesh.clone(),
                transform: ctx.position.building_transform(Default::default()) * sprite_transform(),
                ..Default::default()
            })
            .id()]
    }
}

fn tick(
    mut commands: Commands,
    common_assets: Res<CommonAssets>,
    mut spawners: Query<(&mut SpawnerLogic, &mut ItemContainer, &IsoPos)>,
) {
    for (mut spawner, mut container, pos) in spawners.iter_mut() {
        if spawner.timer > 0 {
            spawner.timer -= 1;
        }
        if spawner.timer == 0 && container.item().is_none() {
            spawner.timer = spawner.interval;
            let item = spawn_item(
                &mut commands,
                &common_assets,
                spawner.item.as_item(),
                *pos,
                ItemContainerAlignment::Centroid,
            );
            container.put_item(item);
            container.set_blocked(false);
        }
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(fstage::TICK, tick.system());
    }
}
