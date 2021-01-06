pub mod iso_coord;
pub mod prelude;

use bevy::prelude::*;
use prelude::Axis;
use prelude::*;

fn hello_world(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("conveyor.png");
    let material = materials.add(texture_handle.into());
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            material: material.clone(),
            transform: IsoCoord::origin().building_transform(Axis::C),
            ..Default::default()
        })
        .spawn(SpriteBundle {
            material: material.clone(),
            transform: IsoCoord::origin().offset_b(-1).building_transform(Axis::C),
            ..Default::default()
        })
        .spawn(SpriteBundle {
            material: material.clone(),
            transform: IsoCoord::origin().offset_b(1).building_transform(Axis::C),
            ..Default::default()
        });
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(hello_world.system())
        .run();
}
