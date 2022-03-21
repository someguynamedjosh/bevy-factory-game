use bevy::prelude::*;

#[derive(Default)]
pub struct CommonAssets {
    pub font: Handle<Font>,

    pub tiles: [Handle<StandardMaterial>; 4],

    pub conveyor_mat: (Handle<StandardMaterial>, Handle<StandardMaterial>),
    pub item_mat: Handle<StandardMaterial>,
    pub claw_mat: (Handle<StandardMaterial>, Handle<StandardMaterial>),
    pub spawner_mat: Handle<StandardMaterial>,
    pub destroyer_mat: Handle<StandardMaterial>,

    pub iron_ore_mat: Handle<StandardMaterial>,
    pub iron_mat: Handle<StandardMaterial>,

    pub debug_container_mat: Handle<StandardMaterial>,
    pub debug_blocked_container_mat: Handle<StandardMaterial>,
    pub cursor_accept_mat: Handle<StandardMaterial>,
    pub cursor_deny_mat: Handle<StandardMaterial>,

    pub clay_mat: Handle<StandardMaterial>,

    pub quad_mesh: Handle<Mesh>,
    pub furnace_mesh: Handle<Mesh>,
}

fn startup(
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mesh_mats: ResMut<Assets<StandardMaterial>>,
    mut common_assets: ResMut<CommonAssets>,
) {
    let mut make_mat = |filename: &str| {
        let tex = asset_server.load(filename);
        let mat = StandardMaterial {
            // TODO: Change this to Mask once Mask works.
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
        common_assets.tiles[i] = make_mat(path);
    }

    common_assets.conveyor_mat.0 = make_mat("conveyor_up.png");

    common_assets.conveyor_mat.1 = make_mat("conveyor_down.png");
    common_assets.item_mat = make_mat("item.png");
    common_assets.claw_mat = (make_mat("claw_open.png"), make_mat("claw_closed.png"));
    common_assets.spawner_mat = make_mat("spawner.png");
    common_assets.destroyer_mat = make_mat("destroyer.png");

    common_assets.iron_ore_mat = make_mat("iron_ore.png");
    common_assets.iron_mat = make_mat("iron.png");

    common_assets.debug_container_mat = make_mat("debug_container.png");
    common_assets.debug_blocked_container_mat = make_mat("debug_blocked_container.png");
    common_assets.cursor_accept_mat = make_mat("cursor_accept.png");
    common_assets.cursor_deny_mat = make_mat("cursor_deny.png");

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
