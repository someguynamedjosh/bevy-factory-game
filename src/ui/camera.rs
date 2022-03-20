use bevy::prelude::*;

use crate::prelude::*;

pub struct CameraState {
    pub primary_camera: Entity,
}

pub fn startup(mut commands: Commands) {
    let mut bundle = PerspectiveCameraBundle::default();
    bundle.transform = Transform {
        translation: Vec3::new(0.0, -7.0, 20.0),
        rotation: Quat::from_rotation_x(0.05 * TAU),
        scale: Vec3::ONE,
    };
    let primary_camera = commands.spawn().insert_bundle(bundle).id();
    commands.spawn().insert_bundle(UiCameraBundle::default());

    commands.insert_resource(CameraState { primary_camera });
}

pub fn update(
    camera_state: Res<CameraState>,
    key_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut transforms: Query<&mut Transform>,
) {
    let mut camera_offset = Vec2::ZERO;
    if key_input.pressed(KeyCode::W) {
        camera_offset.y += 1.0;
    }
    if key_input.pressed(KeyCode::S) {
        camera_offset.y -= 1.0;
    }
    if key_input.pressed(KeyCode::D) {
        camera_offset.x += 1.0;
    }
    if key_input.pressed(KeyCode::A) {
        camera_offset.x -= 1.0;
    }
    camera_offset *= time.delta_seconds() * 10.0;
    let mut cam_t = transforms.get_mut(camera_state.primary_camera).unwrap();
    cam_t.translation.x += camera_offset.x;
    cam_t.translation.y += camera_offset.y;
}
