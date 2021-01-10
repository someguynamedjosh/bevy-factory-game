use bevy::prelude::*;

#[derive(Default)]
pub struct CommonAssets {
    pub tiles: [Handle<ColorMaterial>; 4],

    pub conveyor_mat: (Handle<ColorMaterial>, Handle<ColorMaterial>),
    pub item_mat: Handle<ColorMaterial>,
    pub claw_mat: Handle<ColorMaterial>,
    pub spawner_mat: Handle<ColorMaterial>,
    pub destroyer_mat: Handle<ColorMaterial>,

    pub metal_rubble_mat: Handle<ColorMaterial>,
    pub metal_mat: Handle<ColorMaterial>,

    pub debug_container_mat: Handle<ColorMaterial>,
    pub debug_blocked_container_mat: Handle<ColorMaterial>,
    pub cursor_mat: Handle<ColorMaterial>,
    pub arrow_mat: Handle<ColorMaterial>,
}

fn startup(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut common_assets: ResMut<CommonAssets>,
) {
    for (i, path) in [
        "tile.png",
        "tile_input.png",
        "tile_output.png",
        "tile_misc.png",
    ]
    .iter()
    .enumerate()
    {
        let texture_handle = asset_server.load(*path);
        common_assets.tiles[i] = materials.add(texture_handle.into());
    }

    let texture_handle = asset_server.load("conveyor_up.png");
    common_assets.conveyor_mat.0 = materials.add(texture_handle.into());
    let texture_handle = asset_server.load("conveyor_down.png");
    common_assets.conveyor_mat.1 = materials.add(texture_handle.into());
    let texture_handle = asset_server.load("item.png");
    common_assets.item_mat = materials.add(texture_handle.into());
    let texture_handle = asset_server.load("claw.png");
    common_assets.claw_mat = materials.add(texture_handle.into());
    let texture_handle = asset_server.load("spawner.png");
    common_assets.spawner_mat = materials.add(texture_handle.into());
    let texture_handle = asset_server.load("destroyer.png");
    common_assets.destroyer_mat = materials.add(texture_handle.into());

    let texture_handle = asset_server.load("metal_rubble.png");
    common_assets.metal_rubble_mat = materials.add(texture_handle.into());
    let texture_handle = asset_server.load("metal.png");
    common_assets.metal_mat = materials.add(texture_handle.into());

    let texture_handle = asset_server.load("debug_container.png");
    common_assets.debug_container_mat = materials.add(texture_handle.into());
    let texture_handle = asset_server.load("debug_blocked_container.png");
    common_assets.debug_blocked_container_mat = materials.add(texture_handle.into());
    let texture_handle = asset_server.load("cursor.png");
    common_assets.cursor_mat = materials.add(texture_handle.into());
    let texture_handle = asset_server.load("arrow.png");
    common_assets.arrow_mat = materials.add(texture_handle.into());
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(CommonAssets::default())
            .add_startup_system(startup.system());
    }
}
