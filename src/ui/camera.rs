use bevy::{prelude::*, input::mouse::MouseWheel};

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
    mut scroll_reader: EventReader<MouseWheel>,
) {
    let mut camera_offset = Vec2::ZERO;
    if key_input.pressed(KeyCode::W) {
        camera_offset.y += 1.0;
    }
    if key_input.pressed(KeyCode::S) {
        camera_offset.y -= 1.0;
    }
    if key_input.pressed(KeyCode::D) {
        // This weird number is so that holding W and D travels along the
        // negative C axis and so on instead of going in a 45 degree angle which
        // does not align with the grid.
        camera_offset.x += 1.0 / (PI / 6.0).tan();
    }
    if key_input.pressed(KeyCode::A) {
        camera_offset.x -= 1.0 / (PI / 6.0).tan();
    }
    let mut cam_t = transforms.get_mut(camera_state.primary_camera).unwrap();
    // Multiplying by Z is a hacky way of making the camera move faster when it's zoomed out.
    camera_offset *= time.delta_seconds() * cam_t.translation.z;
    cam_t.translation.x += camera_offset.x;
    cam_t.translation.y += camera_offset.y;
    for event in scroll_reader.iter() {
        let old = cam_t.translation.z;
        cam_t.translation.z *= 2.0f32.powf(-0.2 * event.y);
        let delta = cam_t.translation.z - old;
        cam_t.translation.y -= 0.5 * delta;
    }
}
