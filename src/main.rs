mod assets;
mod claw;
mod common;
mod conveyor;
pub mod iso_pos;
mod item;
pub mod prelude;
mod util;

use bevy::prelude::*;
use prelude::*;

fn test_scene(commands: &mut Commands, common_assets: Res<CommonAssets>) {
    let mut bundle = Camera2dBundle::default();
    bundle.transform.scale *= 2.0;
    commands.spawn(bundle);

    let mut pos = IsoPos::origin();
    let mut facing = IsoDirection::PosA;
    let first = spawn::conveyor(commands, &common_assets, pos, facing, true);
    let spawner = spawn::spawner(commands, &common_assets, pos.offset_a(-1), 5);
    spawn::claw(commands, &common_assets, spawner, first, 1);

    let mut claw_from = None;
    let mut claw_to = None;
    let mut last = None;
    for turn in &[
        0, 2, 0, 1, 0, 0, 1, 0, 0, 1, 3, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1,
        0, 0, 4,
    ] {
        if *turn == 1 {
            facing = facing.clockwise();
        }
        pos = pos.offset_perp_direction(facing, 1);

        let conveyor = spawn::conveyor(commands, &common_assets, pos, facing, false);
        if *turn == 2 {
            claw_to = Some(conveyor);
        } else if *turn == 3 {
            claw_from = Some(conveyor);
        } else if *turn == 4 {
            last = Some(conveyor);
        }
    }

    spawn::claw(
        commands,
        &common_assets,
        claw_from.unwrap(),
        claw_to.unwrap(),
        3,
    );
    let destroyer = spawn::destroyer(commands, &common_assets, pos.offset_direction(facing, 1));
    spawn::claw(commands, &common_assets, last.unwrap(), destroyer, 1);
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(common::Plug)
        .add_plugin(assets::Plug)
        .add_plugin(item::Plug)
        // .add_plugin(util::Plug)
        // .add_plugin(claw::Plug)
        .add_plugin(conveyor::Plug)
        .add_startup_system(test_scene.system())
        .run();
}
