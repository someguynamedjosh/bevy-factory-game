use std::collections::HashMap;

use bevy::prelude::{App, Commands, Component, DespawnRecursiveExt, Entity, Plugin, Query};

use super::{
    machine::{self, Shape},
    Buildable, BuildingComponentsContext, BuildingContext, BuildingMaps, WhichMap,
};
use crate::{
    item::{Item, ItemContainer, ItemContainerAlignment},
    prelude::{fstage, IsoDirection, IsoPos},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ItemList(HashMap<Item, u32>);

impl ItemList {
    pub fn new() -> ItemList {
        Self(HashMap::default())
    }

    pub fn from_counts(counts: HashMap<Item, u32>) -> ItemList {
        Self(counts)
    }

    pub fn add(&mut self, item: Item) {
        self.add_bulk(item, 1)
    }

    pub fn add_bulk(&mut self, item: Item, count: u32) {
        if let Some(total_count) = self.0.get_mut(&item) {
            *total_count += count;
        } else {
            self.0.insert(item, count);
        }
    }

    pub fn count(&self, of: &Item) -> u32 {
        self.0.get(of).copied().unwrap_or(0)
    }

    /// Returns the actual number of items removed, which may be less than the
    /// requested count if the requested count is greater than what's available.
    pub fn remove_bulk(&mut self, item: &Item, count: u32) -> u32 {
        if count == 0 {
            return 0;
        }
        if let Some(my_count) = self.0.get_mut(item) {
            if count > *my_count {
                let removed = *my_count;
                *my_count = 0;
                removed
            } else {
                *my_count -= count;
                count
            }
        } else {
            0
        }
    }

    /// Returns the total number of items in the list.
    pub fn total_count(&self) -> u32 {
        self.0.iter().map(|x| *x.1).sum()
    }

    /// Returns the total number of liters this item list occupies.
    pub fn total_volume(&self) -> u32 {
        self.0.iter().map(|(i, c)| i.volume() * *c).sum()
    }

    pub fn summary(&self) -> String {
        let mut result = String::new();
        for (item, &count) in &self.0 {
            if count == 0 {
                continue;
            }
            result.push_str(&format!("{}x {:?}\n", count, item));
        }
        result
    }
}

#[derive(Component)]
pub struct Storage {
    inputs: Vec<Entity>,
    items: ItemList,
    item_volume: u32,
    item_volume_limit: u32,
}

impl Storage {
    pub fn add(&mut self, item: Item) -> Result<(), ()> {
        self.add_bulk(item, 1)
    }

    pub fn add_bulk(&mut self, item: Item, count: u32) -> Result<(), ()> {
        let additional_volume = item.volume() * count;
        if self.item_volume + additional_volume > self.item_volume_limit {
            return Err(());
        }
        self.item_volume += additional_volume;
        self.items.add_bulk(item, count);
        Ok(())
    }

    pub fn count(&self, of: &Item) -> u32 {
        self.items.count(of)
    }

    /// Returns the actual number of items removed, which may be less than the
    /// requested count if the requested count is greater than what's available.
    pub fn remove_bulk(&mut self, item: &Item, count: u32) -> u32 {
        let count = self.items.remove_bulk(item, count);
        self.item_volume -= item.volume() * count;
        count
    }

    /// Does not modify `self`.
    pub fn subtract_available_inventory_from(&self, list: &mut ItemList) {
        for (item, count) in &mut list.0 {
            let my_count = self.items.count(item);
            if my_count >= *count {
                *count = 0;
            } else {
                *count -= my_count;
            }
        }
    }

    /// Modifies `self` in addition to `list`.
    pub fn subtract_available_inventory_from_self_and(&mut self, list: &mut ItemList) {
        for (item, count) in &mut list.0 {
            let removed = self.remove_bulk(item, *count);
            *count -= removed;
        }
    }

    pub fn summary(&self) -> String {
        let mut result = self.items.summary();
        result.push_str(&format!("{}/{}L", self.item_volume, self.item_volume_limit));
        result
    }
}

#[derive(Clone, Debug)]
pub struct BSmallWarehouse(pub ItemList);

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
            items: self.0.clone(),
            item_volume: self.0.total_volume(),
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
