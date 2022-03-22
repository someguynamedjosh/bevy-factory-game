mod assets;
mod buildable;
mod common;
pub mod iso;
mod item;
pub mod prelude;
mod ui;

use bevy::prelude::*;
use buildable::{
    destroyer::BDestroyer, spawn_buildable, spawner::BSpawner, BuildingContext, BuildingMaps,
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
        .add_startup_system(test_scene.system())
        .run();
}
