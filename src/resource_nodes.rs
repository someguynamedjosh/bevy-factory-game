use bevy::prelude::*;

use crate::{buildable::BuildingMaps, item::ReferenceItem, map_newtype, prelude::*};

map_newtype!(ResourceNodeMap, ResourceNode);

#[derive(Clone, Component, Debug, PartialEq, Eq, Hash)]
pub struct ResourceNode {
    pub of: ReferenceItem,
    pub rate: u8,
}

pub fn spawn_resource_node(
    commands: &mut Commands,
    common_assets: &CommonAssets,
    maps: &mut BuildingMaps,
    of: ReferenceItem,
    rate: u8,
    pos: IsoPos,
) {
    let material = match of {
        ReferenceItem::Magnetite => common_assets.magnetite_node_mat.clone(),
        ReferenceItem::Animite => common_assets.animite_node_mat.clone(),
        _ => common_assets.item_mat.clone(),
    };
    let node = ResourceNode { of, rate };
    maps.resource_nodes.set(pos, node.clone());
    commands
        .spawn()
        .insert_bundle(PbrBundle {
            mesh: common_assets.quad_mesh.clone(),
            material,
            transform: pos.building_transform(IsoAxis::default())
                * sprite_transform()
                * Transform::from_scale(Vec3::ONE * 2.0),
            ..Default::default()
        })
        .insert(node)
        .insert(pos);
}

pub fn spawn_resource_node_for_chunk(
    commands: &mut Commands,
    common_assets: &CommonAssets,
    maps: &mut BuildingMaps,
    x: i32,
    y: i32,
) {
    let (dx, dy, of, rate): (i32, i32, u8, u8) = rand((x, y), (0..32, 0..32, 0..255, 1..255));
    let x = x * 38 + dx;
    let y = y * 38 + dy;
    let pos = IsoPos::new(x, y);
    let of = match of {
        0..=127 => ReferenceItem::Magnetite,
        128..=255 => ReferenceItem::Animite,
    };
    let rate = match rate {
        0..=255 => 1,
    };
    spawn_resource_node(commands, common_assets, maps, of, rate, pos)
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.insert_resource(ResourceNodeMap::default());
    }
}
