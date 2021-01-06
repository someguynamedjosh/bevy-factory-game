pub mod iso_pos;
pub mod prelude;

use bevy::prelude::*;
use prelude::*;

#[derive(Default)]
struct TickClock {
    tick_progress: f32,
    tick_this_frame: bool,
}

impl TickClock {
    const TICK_SPEED: f32 = 60.0 / 360.0;

    pub fn get_tick_progress(&self) -> f32 {
        self.tick_progress / Self::TICK_SPEED
    }

    pub fn is_tick_this_frame(&self) -> bool {
        self.tick_this_frame
    }

    fn advance(&mut self, dt: f32) {
        self.tick_progress += dt;
        self.tick_this_frame = self.tick_progress >= Self::TICK_SPEED;
        self.tick_progress %= Self::TICK_SPEED;
    }
}

#[derive(Default)]
struct CommonAssets {
    pub conveyor_mat: Handle<ColorMaterial>,
    pub item_mat: Handle<ColorMaterial>,
}

#[derive(Default)]
struct ConveyorData {
    pub upstream: Option<Entity>,
}
struct Conveyor;
/// Conveyors that have been placed but need to be linked to surrounding conveyors.
struct UnlinkedConveyor;
/// Conveyors that do not have any downstream.
struct TailConveyor;
/// It takes this many ticks for an item to ride one unit of a conveyor.
const CONVEYOR_DURATION: u8 = 4;

#[derive(Clone, Copy, Debug, Default)]
struct BuildingPos {
    pub origin: IsoPos,
    pub facing: IsoDirection,
}

enum ItemAnim {
    Stay(Vec2),
    Lerp {
        from: Vec2,
        to: Vec2,
        total_ticks: u8,
        remaining_ticks: u8,
    },
}

struct Item {
    anim: ItemAnim,
}

impl Item {
    pub fn new(start_pos: Vec2) -> Self {
        Self {
            anim: ItemAnim::Stay(start_pos),
        }
    }

    pub fn current_rest_position(&self) -> Vec2 {
        match &self.anim {
            ItemAnim::Stay(pos) => pos.clone(),
            ItemAnim::Lerp {
                to,
                remaining_ticks,
                ..
            } => {
                debug_assert_eq!(*remaining_ticks, 0);
                to.clone()
            }
        }
    }

    pub fn anim_conveyor(&mut self, conveyor_pos: &BuildingPos, anim_duration: u8) {
        let target_pos = conveyor_pos
            .origin
            .axis_aligned_pos(conveyor_pos.facing.axis());
        self.anim = ItemAnim::Lerp {
            from: self.current_rest_position(),
            to: target_pos,
            total_ticks: anim_duration,
            remaining_ticks: anim_duration,
        };
    }

    pub fn anim_stationary(&mut self) {
        self.anim = ItemAnim::Stay(self.current_rest_position());
    }
}

#[derive(Default)]
struct ItemHolder {
    incoming: Option<Entity>,
    ticks_until_arrived: u8,
}

fn spawn_conveyor(
    commands: &mut Commands,
    common_assets: &mut ResMut<CommonAssets>,
    origin: IsoPos,
    facing: IsoDirection,
    start_with_item: bool,
) {
    let incoming = if start_with_item {
        spawn_item(commands, common_assets, origin)
    } else {
        None
    };
    commands
        .spawn(SpriteBundle {
            material: common_assets.conveyor_mat.clone(),
            transform: origin.building_transform(facing.axis()),
            ..Default::default()
        })
        .with(BuildingPos { origin, facing })
        .with(Conveyor)
        .with(ItemHolder {
            incoming,
            ticks_until_arrived: 0,
        })
        .with(ConveyorData::default())
        .with(UnlinkedConveyor);
}

fn spawn_item(
    commands: &mut Commands,
    common_assets: &mut ResMut<CommonAssets>,
    origin: IsoPos,
) -> Option<Entity> {
    commands
        .spawn(SpriteBundle {
            material: common_assets.item_mat.clone(),
            ..Default::default()
        })
        .with(Item::new(origin.centroid_pos()))
        .current_entity()
}

fn hello_world(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut common_assets: ResMut<CommonAssets>,
) {
    let texture_handle = asset_server.load("conveyor.png");
    common_assets.conveyor_mat = materials.add(texture_handle.into());
    let texture_handle = asset_server.load("item.png");
    common_assets.item_mat = materials.add(texture_handle.into());

    let mut bundle = Camera2dBundle::default();
    bundle.transform.scale *= 2.0;
    commands.spawn(bundle);

    let mut pos = IsoPos::origin();
    let mut facing = IsoDirection::PosA;
    spawn_conveyor(commands, &mut common_assets, pos, facing, true);
    for turn in &[
        0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0,
    ] {
        if *turn == 1 {
            facing = facing.clockwise();
        }
        pos = pos.offset_perp_direction(facing, 1);

        spawn_conveyor(commands, &mut common_assets, pos, facing, false);
    }
}

fn update_tick_clock(time: Res<Time>, mut tick_clock: ResMut<TickClock>) {
    tick_clock.advance(time.delta_seconds());
}

fn link_unlinked_conveyors(
    commands: &mut Commands,
    all_conveyors: Query<(Entity, &BuildingPos), With<ConveyorData>>,
    mut unlinked_conveyors: Query<
        (Entity, &mut ConveyorData, &BuildingPos),
        With<UnlinkedConveyor>,
    >,
) {
    for (id, mut data, pos) in unlinked_conveyors.iter_mut() {
        let upstream_pos = pos.origin.offset_perp_direction(pos.facing, -1);
        let downstream_pos = pos.origin.offset_perp_direction(pos.facing, 1);
        let mut has_downstream = false;
        for (cid, cpos) in all_conveyors.iter() {
            // If they are in our upstream position and we are in their downstream position...
            if cpos.origin == upstream_pos {
                let candidate_downstream_pos = cpos.origin.offset_perp_direction(cpos.facing, 1);
                if candidate_downstream_pos == pos.origin {
                    data.upstream = Some(cid);
                    // They have a downstream now, they cannot be
                    commands.remove_one::<TailConveyor>(cid);
                }
            }
            // If they are in our downstream position and we are in their upstream position...
            if cpos.origin == downstream_pos {
                let candidate_upstream_pos = cpos.origin.offset_perp_direction(cpos.facing, -1);
                if candidate_upstream_pos == pos.origin {
                    has_downstream = true;
                }
            }
        }
        commands.remove_one::<UnlinkedConveyor>(id);
        if !has_downstream {
            commands.insert_one(id, TailConveyor);
        }
    }
}

fn tick_conveyors(
    tick_clock: Res<TickClock>,
    tail_conveyors: Query<(Entity,), With<TailConveyor>>,
    mut all_conveyors: Query<(&BuildingPos, &ConveyorData, &mut ItemHolder)>,
    mut all_items: Query<&mut Item>,
) {
    if !tick_clock.is_tick_this_frame() {
        return;
    }
    for (mut current,) in tail_conveyors.iter() {
        loop {
            let (pos, data, item_holder) = all_conveyors.get_mut(current).unwrap();
            let empty = item_holder.incoming.is_none();
            let next = if let Some(next) = data.upstream {
                next
            } else {
                break;
            };
            drop(data);
            if empty {
                let pos = pos.clone();
                let mut upstream_holder = all_conveyors.get_mut(next).unwrap().2;
                if upstream_holder.ticks_until_arrived == 0 {
                    let item = upstream_holder.incoming.take();
                    item.map(|e| {
                        let mut item = all_items.get_mut(e).unwrap();
                        item.anim_conveyor(&pos, CONVEYOR_DURATION);
                    });
                    let mut this_holder = all_conveyors.get_mut(current).unwrap().2;
                    this_holder.incoming = item;
                    this_holder.ticks_until_arrived = CONVEYOR_DURATION;
                }
            } else if item_holder.ticks_until_arrived == 0 {
                item_holder.incoming.map(|e| {
                    let mut item = all_items.get_mut(e).unwrap();
                    item.anim_stationary();
                });
            }
            current = next;
        }
    }
}

fn animate_items(tick_clock: Res<TickClock>, mut items: Query<(&mut Transform, &mut Item)>) {
    for (mut transform, mut item) in items.iter_mut() {
        let pos = match &mut item.anim {
            ItemAnim::Stay(pos) => pos.clone(),
            ItemAnim::Lerp {
                from,
                to,
                total_ticks,
                remaining_ticks,
            } => {
                if tick_clock.is_tick_this_frame() && *remaining_ticks > 0 {
                    *remaining_ticks -= 1;
                }
                let progress = ((*total_ticks - *remaining_ticks - 1) as f32
                    + tick_clock.get_tick_progress())
                    / *total_ticks as f32;
                from.lerp(*to, progress)
            }
        };
        transform.translation = (pos, 0.1).into();
    }
}

fn tick_item_holders(tick_clock: Res<TickClock>, mut item_holders: Query<(&mut ItemHolder,)>) {
    if !tick_clock.is_tick_this_frame() {
        return;
    }
    for (mut item_holder,) in item_holders.iter_mut() {
        if item_holder.ticks_until_arrived > 0 {
            item_holder.ticks_until_arrived -= 1;
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(CommonAssets::default())
        .add_resource(TickClock::default())
        .add_startup_system(hello_world.system())
        .add_system(update_tick_clock.system())
        .add_system(tick_item_holders.system())
        .add_system(link_unlinked_conveyors.system())
        .add_system(tick_conveyors.system())
        .add_system(animate_items.system())
        .run();
}
