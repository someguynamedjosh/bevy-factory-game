use bevy::{
    prelude::{App, Component, Entity, Plugin, Query, Commands, DespawnRecursiveExt},
    utils::HashMap,
};

use super::{
    machine::{self, Shape},
    Buildable, BuildingComponentsContext, BuildingContext, BuildingMaps, WhichMap,
};
use crate::{
    item::{Item, ItemContainer, ItemContainerAlignment},
    prelude::{fstage, IsoDirection, IsoPos},
};

#[derive(Component)]
pub struct Storage {
    inputs: Vec<Entity>,
    items: HashMap<Item, usize>,
    item_volume: u32,
    item_volume_limit: u32,
}

impl Storage {
    pub fn add(&mut self, item: Item) -> Result<(), ()> {
        if self.item_volume + item.volume() > self.item_volume_limit {
            return Err(());
        }
        self.item_volume += item.volume();
        if let Some(count) = self.items.get_mut(&item) {
            *count += 1;
        } else {
            self.items.insert(item, 1);
        }
        Ok(())
    }

    pub fn count(&self, of: &Item) -> usize {
        self.items.get(of).copied().unwrap_or(0)
    }

    pub fn remove_bulk(&mut self, item: &Item, count: usize) {
        if count == 0 {
            return;
        }
        debug_assert!(self.count(item) >= count);
        *self.items.get_mut(item).unwrap() -= count;
    }

    pub fn summary(&self) -> String {
        let mut result = String::new();
        for (item, &count) in &self.items {
            if count == 0 {
                continue;
            }
            result.push_str(&format!("{}x {:?}\n", count, item));
        }
        result.push_str(&format!("{}/{}L", self.item_volume, self.item_volume_limit));
        result
    }
}

#[derive(Clone, Debug)]
pub struct BSmallWarehouse;

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
    inputs: &[(1, 0), (1, 2), (1, -2)],
    outputs: &[],
};

impl Buildable for BSmallWarehouse {
    type ExtraData = Vec<Entity>;

    fn shape(&self, position: IsoPos, direction: IsoDirection) -> Vec<IsoPos> {
        SHAPE.all_positions(position, direction).collect()
    }

    fn maps(&self) -> Vec<WhichMap> {
        vec![WhichMap::Buildings]
    }

    fn extra_root_components(&self, ctx: &mut BuildingComponentsContext, data: Self::ExtraData) {
        ctx.commands.insert(Storage {
            inputs: data,
            items: HashMap::default(),
            item_volume: 0,
            item_volume_limit: 20_000,
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
        (inputs.clone(), inputs)
    }

    fn spawn_art(&self, ctx: &mut BuildingContext) -> Vec<bevy::prelude::Entity> {
        machine::spawn_placeholder_art(ctx, &SHAPE)
    }
}

fn tick(
    mut commands: Commands,
    mut warehouses: Query<(&mut Storage,)>,
    mut containers: Query<(&mut ItemContainer,)>,
    items: Query<(&Item,)>,
) {
    for (mut warehouse,) in warehouses.iter_mut() {
        for input in warehouse.inputs.clone() {
            let mut container = containers.get_mut(input).unwrap().0;
            if let Some(item) = container.try_take() {
                let success = warehouse.add(items.get(item).unwrap().0.clone());
                if success.is_ok() {
                    commands.entity(item).despawn_recursive();
                } else {
                    container.put_item(item);
                }
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
