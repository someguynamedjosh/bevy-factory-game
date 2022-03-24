mod assets;
mod buildable;
mod common;
pub mod iso;
mod item;
pub mod mini_rand;
pub mod prelude;
pub mod resource_nodes;
mod ui;

use bevy::prelude::*;
use buildable::{
    destroyer::BDestroyer,
    spawn_buildable,
    spawner::BSpawner,
    storage::{BSmallWarehouse, ItemList},
    BuildingContext, BuildingMaps,
};
use item::ReferenceItem;
use prelude::*;

fn test_scene(mut commands: Commands, common_assets: Res<CommonAssets>, mut maps: BuildingMaps) {
    let mut ctx = BuildingContext {
        commands: &mut commands,
        position: IsoPos::default(),
        direction: IsoDirection::default(),
        common_assets: &common_assets,
    };

    ctx.position = IsoPos::new(-5, -3);
    spawn_buildable(
        Box::new(BSpawner {
            item: ReferenceItem::Magnetite,
            interval: 8,
        }),
        &mut ctx,
        &mut maps,
    );

    ctx.position = IsoPos::new(-5, -5);
    spawn_buildable(
        Box::new(BSpawner {
            item: ReferenceItem::Animite,
            interval: 8,
        }),
        &mut ctx,
        &mut maps,
    );

    ctx.position = IsoPos::new(-5, -7);
    spawn_buildable(Box::new(BDestroyer), &mut ctx, &mut maps);

    ctx.position = IsoPos::new(-4, 8);
    ctx.direction = IsoDirection::PosB;
    let mut items = ItemList::new();
    items.add_bulk(ReferenceItem::IronLump.as_item(), 300);
    items.add_bulk(ReferenceItem::PureAnimus.as_item(), 300);
    spawn_buildable(Box::new(BSmallWarehouse(items)), &mut ctx, &mut maps);

    for x in -10..10 {
        for y in -10..10 {
            resource_nodes::spawn_resource_node_for_chunk(
                &mut commands,
                &common_assets,
                &mut maps,
                x,
                y,
            );
        }
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 600.0,
            height: 500.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_obj::ObjPlugin)
        .add_plugin(iso::Plug)
        .add_plugin(common::Plug)
        .add_plugin(assets::Plug)
        .add_plugin(ui::Plug)
        .add_plugin(buildable::Plug)
        .add_plugin(item::Plug)
        .add_plugin(resource_nodes::Plug)
        .add_startup_system(test_scene.system())
        .run();
}
