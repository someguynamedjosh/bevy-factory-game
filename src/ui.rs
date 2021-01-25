use crate::iso_pos::GRID_MEDIAN_LENGTH;
use crate::item::ItemContainer;
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
    tool_text: Entity,

    world_cursor: Entity,
    arrow: Entity,

    direction: IsoDirection,
    action: MouseAction,
}

enum MouseAction {
    PlaceConveyor,
    PlaceClaw,
    PlaceClawEnd {
        start_pos: IsoPos,
        start_container: Entity,
    },
    PlaceFurnace,
}

impl MouseAction {
    fn use_perpendicular_arrow(&self) -> bool {
        match self {
            Self::PlaceConveyor => true,
            _ => false,
        }
    }

    fn get_snapping(&self, selected_direction: IsoDirection) -> Snapping {
        match self {
            Self::PlaceConveyor => Snapping::None,
            Self::PlaceClaw => Snapping::None,
            Self::PlaceClawEnd { start_pos, .. } => Snapping::AlongAnyLine {
                through: *start_pos,
            },
            Self::PlaceFurnace => Snapping::require_edge_pointing_in(selected_direction),
        }
    }
}

fn startup(commands: &mut Commands, assets: Res<CommonAssets>) {
    let mut bundle = Camera2dBundle::default();
    bundle.transform.scale *= 2.0;
    commands.spawn(bundle);
    let primary_camera = commands.current_entity().unwrap();
    commands.spawn(CameraUiBundle::default());

    commands.spawn(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexStart,
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text {
            value: "Hello world!".to_owned(),
            style: TextStyle {
                font_size: 16.0,
                ..Default::default()
            },
            font: assets.font.clone(),
        },
        ..Default::default()
    });
    let tool_text = commands.current_entity().unwrap();

    commands.spawn(SpriteBundle {
        material: assets.cursor_mat.clone(),
        ..Default::default()
    });
    let world_cursor = commands.current_entity().unwrap();

    commands.spawn(SpriteBundle {
        material: assets.arrow_mat.clone(),
        ..Default::default()
    });
    let arrow = commands.current_entity().unwrap();

    commands.insert_resource(GuiState {
        mouse_pos: Vec2::default(),
        mouse_pos_in_world: IsoPos::default(),
        primary_camera,
        tool_text,
        world_cursor,
        arrow,
        direction: Default::default(),
        action: MouseAction::PlaceConveyor,
    });
}

fn update_mouse_pos(
    mut state: ResMut<MouseSystemState>,
    events: Res<Events<CursorMoved>>,
    mut gui_state: ResMut<GuiState>,
    windows: Res<Windows>,
    cameras: Query<&Camera>,
    mut transforms: Query<&mut Transform>,
) {
    for event in state.event_reader.iter(&events) {
        gui_state.mouse_pos = event.position;
    }

    let camera = cameras.get(gui_state.primary_camera).unwrap();
    let camera_transform = transforms.get_mut(gui_state.primary_camera).unwrap();
    let inv_mat = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
    let window = windows.get(camera.window).unwrap();
    let (width, height) = (window.width(), window.height());
    let output_pos = gui_state.mouse_pos / Vec2::new(width, height) * 2.0 - Vec2::one();
    let world_pos = inv_mat.mul_vec4((output_pos.x, output_pos.y, 0.0, 1.0).into());
    let world_pos_2 = (world_pos.x, world_pos.y).into();
    let snapping = gui_state.action.get_snapping(gui_state.direction);
    gui_state.mouse_pos_in_world = IsoPos::from_world_pos(world_pos_2, snapping);

    let mut cursor_transform = transforms.get_mut(gui_state.world_cursor).unwrap();
    *cursor_transform = gui_state.mouse_pos_in_world.building_transform(IsoAxis::A);
    cursor_transform.translation += Vec3::unit_z() * 0.05;
    let mut arrow_transform = transforms.get_mut(gui_state.arrow).unwrap();
    arrow_transform.translation = (gui_state.mouse_pos_in_world.centroid_pos(), 0.06).into();
    let mut angle = -gui_state.direction.unit_vec().angle_between(Vec2::unit_x());
    if gui_state.action.use_perpendicular_arrow() {
        angle += TAU * 0.25;
    }
    arrow_transform.rotation = Quat::from_rotation_z(angle);
}

fn ui_update(
    commands: &mut Commands,
    time: Res<Time>,
    common_assets: Res<CommonAssets>,
    mut state: ResMut<GuiState>,
    input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    containers: Query<(Entity, &IsoPos), With<ItemContainer>>,
    mut transforms: Query<&mut Transform>,
    mut texts: Query<&mut Text>,
) {
    if input.just_pressed(MouseButton::Left) {
        let mut clicked_container = None;
        for (container, pos) in containers.iter() {
            if *pos == state.mouse_pos_in_world {
                clicked_container = Some((*pos, container));
                break;
            }
        }
        match state.action {
            MouseAction::PlaceConveyor => {
                spawn::conveyor(
                    commands,
                    &common_assets,
                    state.mouse_pos_in_world,
                    state.direction,
                    false,
                );
            }
            MouseAction::PlaceClaw => {
                if let Some((p, c)) = clicked_container {
                    state.action = MouseAction::PlaceClawEnd {
                        start_pos: p,
                        start_container: c,
                    };
                }
            }
            MouseAction::PlaceClawEnd {
                start_container,
                start_pos,
            } => {
                if let Some((p, c)) = clicked_container {
                    let distance = p.centroid_pos().distance(start_pos.centroid_pos()) + 0.01;
                    let distance = distance / GRID_MEDIAN_LENGTH;
                    assert!(distance > 0.0 && distance < 255.0);
                    let mut length = (distance.floor() as u8) & !0b1;
                    if distance % 2.0 > 0.2 && distance % 2.0 < 1.8 {
                        length += 1;
                    }
                    spawn::claw(commands, &common_assets, start_container, c, length);
                    state.action = MouseAction::PlaceClaw;
                }
            }
            MouseAction::PlaceFurnace => {
                spawn::furnace(
                    commands,
                    &common_assets,
                    state.mouse_pos_in_world,
                    state.direction,
                );
            }
        }
    }
    if key_input.just_pressed(KeyCode::Key1) {
        state.action = MouseAction::PlaceConveyor;
    }
    if key_input.just_pressed(KeyCode::Key2) {
        state.action = MouseAction::PlaceClaw;
    }
    if key_input.just_pressed(KeyCode::Key3) {
        state.action = MouseAction::PlaceFurnace;
    }
    if key_input.just_pressed(KeyCode::E) {
        state.direction = state.direction.clockwise();
    }
    if key_input.just_pressed(KeyCode::Q) {
        state.direction = state.direction.counter_clockwise();
    }
    let mut camera_offset = Vec2::zero();
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
    camera_offset *= time.delta_seconds() * 1_000.0;
    let mut cam_t = transforms.get_mut(state.primary_camera).unwrap();
    cam_t.translation += (camera_offset, 0.0).into();

    let tooltip = match &state.action {
        MouseAction::PlaceClaw => format!("Claw Start"),
        MouseAction::PlaceClawEnd{..} => format!("Claw End"),
        MouseAction::PlaceConveyor => format!("Conveyor"),
        MouseAction::PlaceFurnace => format!("Furnace"),
    };
    let mut text = texts.get_mut(state.tool_text).unwrap();
    text.value = tooltip;
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MouseSystemState>()
            .add_startup_system(startup.system())
            .add_system_to_stage(fstage::UI, update_mouse_pos.system())
            .add_system_to_stage(fstage::UI, ui_update.system());
    }
}
