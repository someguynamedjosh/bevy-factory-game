pub mod iso_pos;
pub mod prelude;

use bevy::prelude::*;
use prelude::*;

#[derive(Clone, Copy, Debug, Default)]
struct BuildingPos {
    pub origin: IsoPos,
    pub facing: IsoDirection,
}

#[derive(Bundle)]
struct BuildingBundle {
    pub pos: BuildingPos,
}

fn building_bundles(
    material: Handle<ColorMaterial>,
    origin: IsoPos,
    facing: IsoDirection,
) -> (SpriteBundle, BuildingBundle) {
    (
        SpriteBundle {
            material,
            transform: origin.building_transform(facing.axis()),
            ..Default::default()
        },
        BuildingBundle {
            pos: BuildingPos { origin, facing },
        },
    )
}

fn hello_world(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("conveyor.png");
    let material = materials.add(texture_handle.into());
    commands.spawn(Camera2dBundle::default());

    let mut pos = IsoPos::origin();
    let mut facing = IsoDirection::PosA;
    for turn in &[
        0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0,
    ] {
        let bundles = building_bundles(material.clone(), pos, facing);
        commands.spawn_with_bundles(bundles);

        if *turn == 1 {
            facing = facing.counter_clockwise();
        }
        pos = pos.offset_perp_direction(facing, 1);
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(hello_world.system())
        .run();
}
