mod assets;
mod building;
mod claw;
mod common;
mod conveyor;
pub mod iso_pos;
mod item;
mod machine;
pub mod prelude;
mod spatial_map;
mod trading;
mod ui;
mod util;

use bevy::prelude::*;
use prelude::*;
use util::{spawn_destroyer, spawn_spawner};

fn test_scene(
    commands: &mut Commands,
    common_assets: Res<CommonAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_spawner(commands, &common_assets, IsoPos::new(-5, -3), 8);
    spawn_spawner(commands, &common_assets, IsoPos::new(-5, -4), 8);
    spawn_destroyer(commands, &common_assets, IsoPos::new(-5, -6));

    let mesh_handle = meshes.add(Mesh::from(shape::Box::new(2.0, 2.0, 2.0)));
    let material_handle = materials.add(StandardMaterial {
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        mesh: mesh_handle,
        material: material_handle,
        ..Default::default()
    });
    commands.spawn(LightBundle {
        light: Light {
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..Default::default()
    });
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            width: 600.0,
            height: 500.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(spatial_map::Plug)
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
