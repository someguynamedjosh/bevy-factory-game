mod assets;
mod building;
mod claw;
mod common;
mod conveyor;
pub mod iso_pos;
mod item;
mod machine;
pub mod prelude;
mod trading;
mod ui;
mod util;

use bevy::prelude::*;
use prelude::*;
use util::{spawn_destroyer, spawn_spawner};

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
        .add_plugin(common::Plug)
        .add_plugin(assets::Plug)
        .add_plugin(ui::Plug)
        .add_plugin(util::Plug)
        .add_plugin(machine::Plug)
        .add_plugin(trading::Plug)
        .add_plugin(claw::Plug)
        .add_plugin(conveyor::Plug)
        .add_plugin(item::Plug)
        .add_startup_system(test_scene.system())
        .run();
}
