use bevy::{math::Vec4Swizzles, prelude::*, render::camera::Camera};

use super::{action::ActionState, camera::CameraState};
use crate::{buildable::BuildingMaps, prelude::*};

pub struct CursorState {
    pub pos: Vec2,
    pub world_pos: IsoPos,
    pub direction: IsoDirection,

    pub hovered_container: Option<Entity>,

    world_cursor: Entity,
}

pub fn startup(mut commands: Commands, assets: Res<CommonAssets>) {
    let world_cursor = commands
        .spawn()
        .insert_bundle(PbrBundle {
            material: assets.cursor_accept_mat.clone(),
            mesh: assets.quad_mesh.clone(),
            transform: sprite_transform(),
            ..Default::default()
        })
        .id();

    commands.insert_resource(CursorState {
        pos: Vec2::default(),
        world_pos: IsoPos::default(),
        direction: Default::default(),
        hovered_container: None,
        world_cursor,
    });
}

pub fn update_pre(
    key_input: Res<Input<KeyCode>>,
    mut reader: EventReader<CursorMoved>,
    action_state: Res<ActionState>,
    camera_state: Res<CameraState>,
    mut cursor_state: ResMut<CursorState>,
    windows: Res<Windows>,
    cameras: Query<&Camera>,
    mut transforms: Query<&mut Transform>,
    maps: BuildingMaps,
) {
    for event in reader.iter() {
        cursor_state.pos = event.position;
    }

    if key_input.just_pressed(KeyCode::E) {
        cursor_state.direction = cursor_state.direction.clockwise();
    }
    if key_input.just_pressed(KeyCode::Q) {
        cursor_state.direction = cursor_state.direction.counter_clockwise();
    }

    // https://antongerdelan.net/opengl/raycasting.html
    let camera = cameras.get(camera_state.primary_camera).unwrap();
    let camera_transform = transforms.get_mut(camera_state.primary_camera).unwrap();
    let window = windows.get(camera.window).unwrap();
    let (width, height) = (window.width(), window.height());
    let output_pos = cursor_state.pos / Vec2::new(width, height) * 2.0 - Vec2::ONE;
    let clip_pos = camera
        .projection_matrix
        .inverse()
        .mul_vec4((output_pos.x, output_pos.y, -1.0, 1.0).into());
    let world_pos = camera_transform
        .compute_matrix()
        //     .inverse()
        // Mat4::identity()
        .mul_vec4((clip_pos.x, clip_pos.y, -1.0, 0.0).into())
        .xyz()
        .normalize();
    let desired_delta_z = -camera_transform.translation.z;
    let world_pos = world_pos * (desired_delta_z / world_pos.z);
    let world_pos_2 = Vec2::new(
        world_pos.x + camera_transform.translation.x,
        world_pos.y + camera_transform.translation.y,
    );
    let snapping = action_state.action.get_snapping(cursor_state.direction);
    cursor_state.world_pos = IsoPos::from_world_pos(world_pos_2, snapping);

    let mut cursor_transform = transforms.get_mut(cursor_state.world_cursor).unwrap();
    *cursor_transform = cursor_state.world_pos.building_transform(IsoAxis::A)
    // Why the hell do we need to do this twice?!?!
    * sprite_transform()
    * sprite_transform();
    cursor_transform.translation.z += 0.02;

    cursor_state.hovered_container = maps.item_containers.get(cursor_state.world_pos).copied();
}

pub fn update_post(
    common_assets: Res<CommonAssets>,
    mut materials: Query<&mut Handle<StandardMaterial>>,
    action_state: Res<ActionState>,
    cursor_state: Res<CursorState>,
) {
    let cursor_mat = if action_state.ok {
        common_assets.cursor_accept_mat.clone()
    } else {
        common_assets.cursor_deny_mat.clone()
    };
    *materials.get_mut(cursor_state.world_cursor).unwrap() = cursor_mat;
}
