use bevy::prelude::*;

#[derive(Default)]
pub struct CommonAssets {
    pub font: Handle<Font>,

    pub tiles: [Handle<ColorMaterial>; 4],

    pub conveyor_mat: (Handle<ColorMaterial>, Handle<ColorMaterial>),
    pub item_mat: Handle<ColorMaterial>,
    pub claw_mat: Handle<ColorMaterial>,
    pub spawner_mat: Handle<ColorMaterial>,
    pub destroyer_mat: Handle<ColorMaterial>,

    pub metal_rubble_mat: Handle<ColorMaterial>,
    pub metal_mat: Handle<ColorMaterial>,
    pub structural_mat: Handle<ColorMaterial>,
    pub circuit_mat: Handle<ColorMaterial>,

    pub debug_container_mat: Handle<ColorMaterial>,
    pub debug_blocked_container_mat: Handle<ColorMaterial>,
    pub cursor_accept_mat: Handle<ColorMaterial>,
    pub cursor_deny_mat: Handle<ColorMaterial>,
    pub arrow_mat: Handle<ColorMaterial>,

    pub clay_mat: Handle<StandardMaterial>,

    pub furnace_mesh: Handle<Mesh>,
}

fn startup(
    asset_server: Res<AssetServer>,
    mut sprite_mats: ResMut<Assets<ColorMaterial>>,
    mut mesh_mats: ResMut<Assets<StandardMaterial>>,
    mut common_assets: ResMut<CommonAssets>,
) {
    let mut make_mat = |filename: &str| {
        let texture_handle = asset_server.load(filename);
        sprite_mats.add(ColorMaterial {
            texture: Some(texture_handle),
            ..Default::default()
        })
    };
    common_assets.font = asset_server.load("LiberationMono-Regular.ttf");
    for (i, path) in [
        "tile.png",
        "tile_input.png",
        "tile_output.png",
        "tile_misc.png",
    ]
    .iter()
    .enumerate()
    {
        common_assets.tiles[i] = make_mat(path);
    }

    common_assets.conveyor_mat.0 = make_mat("conveyor_up.png");

    common_assets.conveyor_mat.1 = make_mat("conveyor_down.png");
    common_assets.item_mat = make_mat("item.png");
    common_assets.claw_mat = make_mat("claw.png");
    common_assets.spawner_mat = make_mat("spawner.png");
    common_assets.destroyer_mat = make_mat("destroyer.png");

    common_assets.metal_rubble_mat = make_mat("metal_rubble.png");
    common_assets.metal_mat = make_mat("metal.png");
    common_assets.structural_mat = make_mat("structural.png");
    common_assets.circuit_mat = make_mat("circuit.png");

    common_assets.debug_container_mat = make_mat("debug_container.png");
    common_assets.debug_blocked_container_mat = make_mat("debug_blocked_container.png");
    common_assets.cursor_accept_mat = make_mat("cursor_accept.png");
    common_assets.cursor_deny_mat = make_mat("cursor_deny.png");
    common_assets.arrow_mat = make_mat("arrow.png");
    
    common_assets.clay_mat = mesh_mats.add(StandardMaterial {
        albedo: Color::rgb(1.0, 1.0, 1.0),
        ..Default::default()
    });

    common_assets.furnace_mesh = asset_server.load("furnace.obj");
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(CommonAssets::default())
            .add_startup_system(startup.system());
    }
}
