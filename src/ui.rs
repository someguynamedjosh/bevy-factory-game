use crate::prelude::*;
use bevy::prelude::*;
use bevy::render::camera::Camera;

#[derive(Default)]
struct MouseSystemState {
    event_reader: EventReader<CursorMoved>,
}

struct GuiState {
    mouse_pos: Vec2,
    mouse_pos_in_world: IsoPos,
    primary_camera: Entity,
    world_cursor: Entity,
}

fn startup(commands: &mut Commands, assets: Res<CommonAssets>) {
    let mut bundle = Camera2dBundle::default();
    bundle.transform.scale *= 2.0;
    commands.spawn(bundle);
    let primary_camera = commands.current_entity().unwrap();

    commands.spawn(SpriteBundle {
        material: assets.cursor_mat.clone(),
        ..Default::default()
    });
    let world_cursor = commands.current_entity().unwrap();

    commands.insert_resource(GuiState {
        mouse_pos: Vec2::default(),
        mouse_pos_in_world: IsoPos::default(),
        primary_camera,
        world_cursor,
    });
}

fn update_mouse_pos(
    mut state: ResMut<MouseSystemState>,
    events: Res<Events<CursorMoved>>,
    mut gui_state: ResMut<GuiState>,
    cameras: Query<&Camera>,
    mut transforms: Query<&mut Transform>,
    windows: Res<Windows>,
) {
    let camera = cameras.get(gui_state.primary_camera).unwrap();
    let inv_mat = camera.projection_matrix.inverse();
    let window = windows.get(camera.window).unwrap();
    let (width, height) = (window.width(), window.height());
    for event in state.event_reader.iter(&events) {
        gui_state.mouse_pos = event.position;
        let output_pos = event.position / Vec2::new(width, height) * 2.0 - Vec2::one();
        let world_pos = inv_mat.mul_vec4((output_pos.x, output_pos.y, 0.0, 1.0).into()) * 2.0;
        gui_state.mouse_pos_in_world = IsoPos::from_world_pos((world_pos.x, world_pos.y).into());
    }
    let mut cursor_transform = transforms.get_mut(gui_state.world_cursor).unwrap();
    *cursor_transform = gui_state.mouse_pos_in_world.building_transform(IsoAxis::A);
    cursor_transform.translation += Vec3::unit_z() * 0.05;
}

fn test(input: Res<Input<MouseButton>>) {
    if input.just_pressed(MouseButton::Left) {
        println!("Click!");
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MouseSystemState>()
            .add_startup_system(startup.system())
            .add_system_to_stage(fstage::UI, update_mouse_pos.system())
            .add_system_to_stage(fstage::UI, test.system());
    }
}
