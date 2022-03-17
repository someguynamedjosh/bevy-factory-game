mod assets;
mod buildable;
mod buildable2;
mod common;
pub mod iso;
mod item;
pub mod prelude;
mod sprite_render;
mod ui;

use bevy::prelude::*;
use buildable::util::{spawn_destroyer, spawn_spawner};
use prelude::*;

fn test_scene(mut commands: Commands, common_assets: Res<CommonAssets>) {
    spawn_spawner(&mut commands, &common_assets, IsoPos::new(-5, -3), 8);
    spawn_spawner(&mut commands, &common_assets, IsoPos::new(-5, -4), 8);
    spawn_destroyer(&mut commands, &common_assets, IsoPos::new(-5, -6));
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
        .add_plugin(sprite_render::Plug)
        .add_plugin(iso::Plug)
        .add_plugin(common::Plug)
        .add_plugin(assets::Plug)
        .add_plugin(ui::Plug)
        .add_plugin(buildable::Plug)
        .add_plugin(item::Plug)
        .add_startup_system(test_scene.system())
        .run();
}
