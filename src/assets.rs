use bevy::prelude::*;

#[derive(Default)]
pub struct CommonAssets {
    pub font: Handle<Font>,

    pub tiles: [Handle<Image>; 4],

    pub conveyor_mat: (Handle<Image>, Handle<Image>),
    pub item_mat: Handle<Image>,
    pub claw_mat: Handle<Image>,
    pub spawner_mat: Handle<Image>,
    pub destroyer_mat: Handle<Image>,

    pub metal_rubble_mat: Handle<Image>,
    pub metal_mat: Handle<Image>,
    pub structural_mat: Handle<Image>,
    pub circuit_mat: Handle<Image>,

    pub debug_container_mat: Handle<Image>,
    pub debug_blocked_container_mat: Handle<Image>,
    pub cursor_accept_mat: Handle<StandardMaterial>,
    pub cursor_deny_mat: Handle<StandardMaterial>,
    pub arrow_mat: Handle<Image>,

    pub clay_mat: Handle<StandardMaterial>,

    pub quad_mesh: Handle<Mesh>,
    pub furnace_mesh: Handle<Mesh>,
}

fn startup(
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sprite_mats: ResMut<Assets<ColorMaterial>>,
    mut mesh_mats: ResMut<Assets<StandardMaterial>>,
    mut common_assets: ResMut<CommonAssets>,
) {
    let mut make_tex = |filename: &str| asset_server.load(filename);
    let mut make_mat = |filename: &str| {
        let tex = asset_server.load(filename);
        let mat = StandardMaterial {
            alpha_mode: AlphaMode::Blend,
            base_color_texture: Some(tex),
            unlit: true,
            ..Default::default()
        };
        mesh_mats.add(mat)
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
        common_assets.tiles[i] = make_tex(path);
    }

    common_assets.conveyor_mat.0 = make_tex("conveyor_up.png");

    common_assets.conveyor_mat.1 = make_tex("conveyor_down.png");
    common_assets.item_mat = make_tex("item.png");
    common_assets.claw_mat = make_tex("claw.png");
    common_assets.spawner_mat = make_tex("spawner.png");
    common_assets.destroyer_mat = make_tex("destroyer.png");

    common_assets.metal_rubble_mat = make_tex("metal_rubble.png");
    common_assets.metal_mat = make_tex("metal.png");
    common_assets.structural_mat = make_tex("structural.png");
    common_assets.circuit_mat = make_tex("circuit.png");

    common_assets.debug_container_mat = make_tex("debug_container.png");
    common_assets.debug_blocked_container_mat = make_tex("debug_blocked_container.png");
    common_assets.cursor_accept_mat = make_mat("cursor_accept.png");
    common_assets.cursor_deny_mat = make_mat("cursor_deny.png");
    common_assets.arrow_mat = make_tex("arrow.png");

    common_assets.clay_mat = mesh_mats.add(StandardMaterial {
        base_color: Color::rgb(1.0, 1.0, 1.0),
        ..Default::default()
    });

    common_assets.quad_mesh = meshes.add(shape::Quad::new(Vec2::ONE).into());
    common_assets.furnace_mesh = asset_server.load("furnace.obj");
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.insert_resource(CommonAssets::default())
            .add_startup_system_to_stage(StartupStage::PreStartup, startup.system());
    }
}
