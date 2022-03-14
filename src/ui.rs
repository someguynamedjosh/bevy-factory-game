use bevy::{math::Vec4Swizzles, prelude::*, render::camera::Camera};

use crate::{
    buildable::{
        claw::spawn_claw, conveyor::spawn_conveyor, machine::spawn_machine,
        machine_type::MachineType,
    },
    iso::GRID_EDGE_LENGTH,
    item::ItemContainer,
    prelude::*,
};

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

#[derive(Clone, Debug, PartialEq, Eq)]
enum Buildable {
    Machine(MachineType),
}

impl Buildable {
    fn build(
        &self,
        commands: &mut Commands,
        common_assets: &Res<CommonAssets>,
        obstruction_map: &mut ResMut<BuildingObstructionMap>,
        origin: IsoPos,
        facing: IsoDirection,
    ) {
        match self {
            Self::Machine(typ) => spawn_machine(
                commands,
                common_assets,
                obstruction_map,
                *typ,
                origin,
                facing,
            ),
        }
    }

    fn get_shape(&self) -> &'static Shape {
        match self {
            Self::Machine(typ) => typ.get_shape(),
        }
    }
}

enum MouseAction {
    PlaceConveyor,
    PlaceClaw,
    PlaceClawEnd {
        start_pos: IsoPos,
        start_container: Entity,
    },
    Build(Buildable),
}

impl MouseAction {
    fn get_snapping(&self, selected_direction: IsoDirection) -> Snapping {
        match self {
            Self::PlaceConveyor => Snapping::None,
            Self::PlaceClaw => Snapping::None,
            Self::PlaceClawEnd { start_pos, .. } => Snapping::AlongAnyLine {
                through: *start_pos,
            },
            Self::Build(..) => Snapping::require_edge_pointing_in(selected_direction),
        }
    }
}

fn startup(commands: &mut Commands, assets: Res<CommonAssets>) {
    let mut bundle = Camera3dBundle::default();
    bundle.transform = Transform {
        translation: Vec3::new(0.0, -7.0, 20.0),
        rotation: Quat::from_rotation_x(0.05 * TAU),
        scale: Vec3::one(),
    };
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
        material: assets.cursor_accept_mat.clone(),
        transform: SPRITE_TRANSFORM,
        ..Default::default()
    });
    let world_cursor = commands.current_entity().unwrap();

    commands.spawn(SpriteBundle {
        material: assets.arrow_mat.clone(),
        transform: SPRITE_TRANSFORM,
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

    // https://antongerdelan.net/opengl/raycasting.html
    let camera = cameras.get(gui_state.primary_camera).unwrap();
    let camera_transform = transforms.get_mut(gui_state.primary_camera).unwrap();
    let window = windows.get(camera.window).unwrap();
    let (width, height) = (window.width(), window.height());
    let output_pos = gui_state.mouse_pos / Vec2::new(width, height) * 2.0 - Vec2::one();
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
    let snapping = gui_state.action.get_snapping(gui_state.direction);
    gui_state.mouse_pos_in_world = IsoPos::from_world_pos(world_pos_2, snapping);

    let mut cursor_transform = transforms.get_mut(gui_state.world_cursor).unwrap();
    *cursor_transform =
        gui_state.mouse_pos_in_world.building_transform(IsoAxis::A) * SPRITE_TRANSFORM;
    cursor_transform.translation.z += 0.02;

    let mut arrow_transform = transforms.get_mut(gui_state.arrow).unwrap();
    arrow_transform.translation = (gui_state.mouse_pos_in_world.centroid_pos(), 0.06).into();
    let angle = -gui_state.direction.unit_vec().angle_between(Vec2::unit_x());
    arrow_transform.rotation = Quat::from_rotation_z(angle);
    arrow_transform.translation.z += 0.02;
}

fn ui_update(
    commands: &mut Commands,
    time: Res<Time>,
    common_assets: Res<CommonAssets>,
    mut state: ResMut<GuiState>,
    input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    mut obstruction_map: ResMut<BuildingObstructionMap>,
    mut conveyor_map: ResMut<ConveyorMap>,
    containers: Query<(Entity, &ItemContainer, &IsoPos)>,
    mut transforms: Query<&mut Transform>,
    mut texts: Query<&mut Text>,
    mut materials: Query<&mut Handle<ColorMaterial>>,
    items: Query<&Item>,
) {
    let mut hovered_container = None;
    for (entity, container, pos) in containers.iter() {
        if *pos == state.mouse_pos_in_world {
            hovered_container = Some((entity, container, *pos));
            break;
        }
    }
    let ok = match &state.action {
        MouseAction::PlaceConveyor => !obstruction_map.is_occupied(state.mouse_pos_in_world),
        MouseAction::PlaceClaw | MouseAction::PlaceClawEnd { .. } => hovered_container.is_some(),
        MouseAction::Build(typ) => {
            let shape = typ.get_shape();
            (|| {
                let iters = shape.positions(state.mouse_pos_in_world, state.direction);
                for p in iters.blanks.chain(iters.inputs.chain(iters.outputs)) {
                    if obstruction_map.is_occupied(p) {
                        return false;
                    }
                }
                true
            })()
        }
    };
    let cursor_mat = if ok {
        common_assets.cursor_accept_mat.clone()
    } else {
        common_assets.cursor_deny_mat.clone()
    };
    materials.set(state.world_cursor, cursor_mat).unwrap();
    if input.just_pressed(MouseButton::Left) && ok {
        match &state.action {
            MouseAction::PlaceConveyor => {
                spawn_conveyor(
                    commands,
                    &common_assets,
                    &mut conveyor_map,
                    &mut obstruction_map,
                    state.mouse_pos_in_world,
                    state.direction,
                    false,
                );
            }
            MouseAction::PlaceClaw => {
                if let Some((e, _, p)) = hovered_container {
                    state.action = MouseAction::PlaceClawEnd {
                        start_pos: p,
                        start_container: e,
                    };
                }
            }
            MouseAction::PlaceClawEnd {
                start_container,
                start_pos,
            } => {
                if let Some((e, _, p)) = hovered_container {
                    let distance = p.centroid_pos().distance(start_pos.centroid_pos()) + 0.01;
                    let distance = distance / GRID_EDGE_LENGTH * 2.0;
                    assert!(distance > 0.0 && distance < 255.0);
                    let length = (distance + 0.3).floor() as u8;
                    spawn_claw(commands, &common_assets, *start_container, e, length);
                    state.action = MouseAction::PlaceClaw;
                }
            }
            MouseAction::Build(typ) => {
                typ.build(
                    commands,
                    &common_assets,
                    &mut obstruction_map,
                    state.mouse_pos_in_world,
                    state.direction,
                );
            }
        }
    }
    if key_input.just_pressed(KeyCode::Key1) {
        state.action = MouseAction::PlaceConveyor
    }
    if key_input.just_pressed(KeyCode::Key2) {
        state.action = MouseAction::PlaceClaw;
    }
    if key_input.just_pressed(KeyCode::Key3) {
        state.action = MouseAction::Build(Buildable::Machine(MachineType::Purifier));
    }
    if key_input.just_pressed(KeyCode::Key4) {
        state.action = MouseAction::Build(Buildable::Machine(MachineType::Joiner));
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
    camera_offset *= time.delta_seconds() * 10.0;
    let mut cam_t = transforms.get_mut(state.primary_camera).unwrap();
    cam_t.translation += (camera_offset, 0.0).into();

    let tooltip = match &state.action {
        MouseAction::PlaceClaw => format!("Claw Start"),
        MouseAction::PlaceClawEnd { .. } => format!("Claw End"),
        MouseAction::PlaceConveyor => format!("Conveyor"),
        MouseAction::Build(typ) => format!("{:?}", typ),
    };
    let mut text = texts.get_mut(state.tool_text).unwrap();
    let hovered_item = if let Some((_, container, _)) = hovered_container {
        if let Some(item) = container.item() {
            let item = items.get(item).unwrap();
            format!("{:?}", item.as_elements())
        } else {
            format!("")
        }
    } else {
        format!("")
    };
    text.value = format!(
        "{}\n{}\n{}",
        tooltip, /* credits.0.floor() */ 0, hovered_item
    );
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
