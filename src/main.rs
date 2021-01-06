pub mod iso_pos;
pub mod prelude;

use bevy::{
    prelude::*,
    render::{pipeline::RenderPipeline, render_graph::base::MainPass},
    sprite::{QUAD_HANDLE, SPRITE_PIPELINE_HANDLE},
};
use prelude::*;

#[derive(Clone, Copy, Debug, Default)]
struct BuildingPos {
    pub origin: IsoPos,
    pub facing: IsoDirection,
}

#[derive(Bundle)]
struct BuildingBundle {
    // Begin sprite bundle...
    pub sprite: Sprite,
    pub mesh: Handle<Mesh>, // TODO: maybe abstract this out
    pub material: Handle<ColorMaterial>,
    pub main_pass: MainPass,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    // End sprite bundle...
    pub pos: BuildingPos,
}

impl Default for BuildingBundle {
    fn default() -> Self {
        Self {
            // Begin sprite bundle...
            mesh: QUAD_HANDLE.typed(),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                SPRITE_PIPELINE_HANDLE.typed(),
            )]),
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            main_pass: MainPass,
            draw: Default::default(),
            sprite: Default::default(),
            material: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            // End sprite bundle...
            pos: BuildingPos::default(),
        }
    }
}

impl BuildingBundle {
    fn new(material: Handle<ColorMaterial>, origin: IsoPos, facing: IsoDirection) -> Self {
        Self {
            material,
            transform: origin.building_transform(facing.axis()),
            pos: BuildingPos { origin, facing },
            ..Default::default()
        }
    }
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
    for turn in &[0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0] {
        commands.spawn(BuildingBundle::new(material.clone(), pos, facing));
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
