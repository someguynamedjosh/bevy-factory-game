mod assets;
mod buildable;
mod common;
pub mod iso_pos;
mod item;
pub mod prelude;
mod spatial_map;
mod sprite_render;
mod ui;

use bevy::prelude::*;
use prelude::*;
use buildable::util::{spawn_destroyer, spawn_spawner};

fn test_scene(commands: &mut Commands, common_assets: Res<CommonAssets>) {
    spawn_spawner(commands, &common_assets, IsoPos::new(-5, -3), 8);
    spawn_spawner(commands, &common_assets, IsoPos::new(-5, -4), 8);
    spawn_destroyer(commands, &common_assets, IsoPos::new(-5, -6));
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            width: 600.0,
            height: 500.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_obj::ObjPlugin)
        .add_plugin(sprite_render::Plug)
        .add_plugin(spatial_map::Plug)
        .add_plugin(common::Plug)
        .add_plugin(assets::Plug)
        .add_plugin(ui::Plug)
        .add_plugin(buildable::Plug)
        .add_plugin(item::Plug)
        .add_startup_system(test_scene.system())
        .run();
}
