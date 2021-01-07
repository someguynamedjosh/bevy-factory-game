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
    pub claw_mat: Handle<ColorMaterial>,
    pub spawner_mat: Handle<ColorMaterial>,
    pub destroyer_mat: Handle<ColorMaterial>,
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

    pub fn anim_to_holder(
        &mut self,
        pos: IsoPos,
        alignment: ItemHolderAlignment,
        anim_duration: u8,
    ) {
        let target_pos = alignment.get_item_pos(pos);
        self.anim = ItemAnim::Lerp {
            from: self.current_rest_position(),
            to: target_pos,
            total_ticks: anim_duration,
            remaining_ticks: anim_duration,
        };
    }

    pub fn anim_stationary_in_holder(&mut self, pos: IsoPos, alignment: ItemHolderAlignment) {
        self.anim = ItemAnim::Stay(alignment.get_item_pos(pos));
    }

    pub fn anim_stationary_exact(&mut self, pos: Vec2) {
        self.anim = ItemAnim::Stay(pos);
    }
}

#[derive(Clone, Copy)]
enum ItemHolderAlignment {
    Centroid,
    AxisAligned(IsoAxis),
}

impl ItemHolderAlignment {
    pub fn get_item_pos(&self, coarse_pos: IsoPos) -> Vec2 {
        match self {
            Self::Centroid => coarse_pos.centroid_pos(),
            Self::AxisAligned(axis) => coarse_pos.axis_aligned_pos(*axis),
        }
    }
}

struct ItemHolder {
    alignment: ItemHolderAlignment,
    incoming: Option<Entity>,
    ticks_until_arrived: u8,
}

struct DebugSpawner {
    rate: u8,
    spawn_cycle: u8,
}
struct DebugDestroyer;

struct Claw {
    take_from: Entity,
    move_to: Entity,
    held_item: Option<Entity>,
    /// Length of the gantry in grid cells.
    length: u8,
    current_anim_tick: u8,
    blocked: bool,
}
// How long it takes for the claw to traverse a segment of its path.
const CLAW_SEGMENT_DURATION: u8 = 2;

impl Claw {
    pub fn anim_length(&self) -> u8 {
        // A 1 length claw has to traverse 4 segments, 2 length 6 segments, 3/8, etc.
        (self.length + 1) * 2 * CLAW_SEGMENT_DURATION
    }
}

fn spawn_conveyor(
    commands: &mut Commands,
    common_assets: &mut ResMut<CommonAssets>,
    origin: IsoPos,
    facing: IsoDirection,
    start_with_item: bool,
) -> Option<Entity> {
    let alignment = ItemHolderAlignment::AxisAligned(facing.axis());
    let incoming = if start_with_item {
        spawn_item(commands, common_assets, origin, alignment)
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
            alignment,
            incoming,
            ticks_until_arrived: 0,
        })
        .with(ConveyorData::default())
        .with(UnlinkedConveyor)
        .current_entity()
}

fn spawn_item(
    commands: &mut Commands,
    common_assets: &mut ResMut<CommonAssets>,
    origin: IsoPos,
    alignment: ItemHolderAlignment,
) -> Option<Entity> {
    commands
        .spawn(SpriteBundle {
            material: common_assets.item_mat.clone(),
            ..Default::default()
        })
        .with(Item::new(alignment.get_item_pos(origin)))
        .current_entity()
}

fn spawn_claw(
    commands: &mut Commands,
    common_assets: &mut ResMut<CommonAssets>,
    from: Entity,
    to: Entity,
    length: u8,
) {
    commands
        .spawn(SpriteBundle {
            material: common_assets.claw_mat.clone(),
            ..Default::default()
        })
        .with(Claw {
            // We can't guarantee that there is an item ready to pick up when we spawn.
            blocked: true,
            current_anim_tick: 0,
            held_item: None,
            take_from: from,
            move_to: to,
            length,
        });
}

fn spawn_spawner(
    commands: &mut Commands,
    common_assets: &mut ResMut<CommonAssets>,
    origin: IsoPos,
    rate: u8,
) -> Option<Entity> {
    commands
        .spawn(SpriteBundle {
            material: common_assets.spawner_mat.clone(),
            transform: origin.building_transform(Default::default()),
            ..Default::default()
        })
        .with(BuildingPos {
            origin,
            facing: Default::default(),
        })
        .with(DebugSpawner {
            rate,
            spawn_cycle: 0,
        })
        .with(ItemHolder {
            alignment: ItemHolderAlignment::Centroid,
            incoming: None,
            ticks_until_arrived: 0,
        })
        .current_entity()
}

fn spawn_destroyer(
    commands: &mut Commands,
    common_assets: &mut ResMut<CommonAssets>,
    origin: IsoPos,
) -> Option<Entity> {
    commands
        .spawn(SpriteBundle {
            material: common_assets.destroyer_mat.clone(),
            transform: origin.building_transform(Default::default()),
            ..Default::default()
        })
        .with(BuildingPos {
            origin,
            facing: Default::default(),
        })
        .with(DebugDestroyer)
        .with(ItemHolder {
            alignment: ItemHolderAlignment::Centroid,
            incoming: None,
            ticks_until_arrived: 0,
        })
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
    let texture_handle = asset_server.load("claw.png");
    common_assets.claw_mat = materials.add(texture_handle.into());
    let texture_handle = asset_server.load("spawner.png");
    common_assets.spawner_mat = materials.add(texture_handle.into());
    let texture_handle = asset_server.load("destroyer.png");
    common_assets.destroyer_mat = materials.add(texture_handle.into());

    let mut bundle = Camera2dBundle::default();
    bundle.transform.scale *= 2.0;
    commands.spawn(bundle);

    let mut pos = IsoPos::origin();
    let mut facing = IsoDirection::PosA;
    let first = spawn_conveyor(commands, &mut common_assets, pos, facing, true).unwrap();
    let spawner = spawn_spawner(commands, &mut common_assets, pos.offset_a(-1), 5).unwrap();
    spawn_claw(commands, &mut common_assets, spawner, first, 1);

    let mut claw_from = None;
    let mut claw_to = None;
    let mut last = None;
    for turn in &[
        0, 2, 0, 1, 0, 0, 1, 0, 0, 1, 3, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1,
        0, 0, 4,
    ] {
        if *turn == 1 {
            facing = facing.clockwise();
        }
        pos = pos.offset_perp_direction(facing, 1);

        let conveyor = spawn_conveyor(commands, &mut common_assets, pos, facing, false);
        if *turn == 2 {
            claw_to = conveyor;
        } else if *turn == 3 {
            claw_from = conveyor;
        } else if *turn == 4 {
            last = conveyor;
        }
    }

    spawn_claw(
        commands,
        &mut common_assets,
        claw_from.unwrap(),
        claw_to.unwrap(),
        3,
    );
    let destroyer = spawn_destroyer(
        commands,
        &mut common_assets,
        pos.offset_direction(facing, 1),
    );
    spawn_claw(
        commands,
        &mut common_assets,
        last.unwrap(),
        destroyer.unwrap(),
        1,
    );
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

fn tick_spawners(
    commands: &mut Commands,
    mut common_assets: ResMut<CommonAssets>,
    tick_clock: Res<TickClock>,
    mut spawners: Query<(&mut DebugSpawner, &mut ItemHolder, &BuildingPos)>,
) {
    if !tick_clock.is_tick_this_frame() {
        return;
    }

    for (mut spawner, mut holder, pos) in spawners.iter_mut() {
        if holder.incoming.is_none() {
            spawner.spawn_cycle += 1;
            if spawner.spawn_cycle >= spawner.rate {
                spawner.spawn_cycle = 0;
                let item = spawn_item(
                    commands,
                    &mut common_assets,
                    pos.origin,
                    ItemHolderAlignment::Centroid,
                )
                .unwrap();
                holder.incoming = Some(item);
                holder.ticks_until_arrived = 0;
            }
        }
    }
}

fn tick_destroyers(
    commands: &mut Commands,
    tick_clock: Res<TickClock>,
    mut destroyers: Query<(&mut ItemHolder,), With<DebugDestroyer>>,
) {
    if !tick_clock.is_tick_this_frame() {
        return;
    }

    for (mut holder,) in destroyers.iter_mut() {
        if holder.ticks_until_arrived != 0 {
            continue;
        }
        if let Some(item) = holder.incoming.take() {
            commands.despawn(item);
        }
    }
}

fn tick_claws(
    tick_clock: Res<TickClock>,
    mut claws: Query<(&mut Claw,)>,
    mut holders: Query<(&mut ItemHolder,)>,
) {
    if !tick_clock.is_tick_this_frame() {
        return;
    }

    for (mut claw,) in claws.iter_mut() {
        let anim_length = claw.anim_length();
        if !claw.blocked {
            claw.current_anim_tick = (claw.current_anim_tick + 1) % anim_length;
        }
        claw.blocked = false;
        if claw.current_anim_tick == 0 {
            // Trying to pick up an item.
            let mut from = holders
                .get_component_mut::<ItemHolder>(claw.take_from)
                .unwrap();
            if from.incoming.is_some() && from.ticks_until_arrived == 0 {
                claw.held_item = from.incoming.take();
            } else {
                claw.blocked = true;
            }
        } else if claw.current_anim_tick == anim_length / 2 {
            let mut to = holders
                .get_component_mut::<ItemHolder>(claw.move_to)
                .unwrap();
            if to.incoming.is_none() {
                to.incoming = claw.held_item.take();
                to.ticks_until_arrived = 0;
            } else {
                claw.blocked = true;
            }
        }
    }
}

fn animate_claws(
    tick_clock: Res<TickClock>,
    mut claws: Query<(&Claw, &mut Transform)>,
    item_holders: Query<(&ItemHolder, &BuildingPos)>,
    mut items: Query<(&mut Item,)>,
) {
    for (claw, mut transform) in claws.iter_mut() {
        let (from_holder, from_pos) = item_holders.get(claw.take_from).unwrap();
        let (to_holder, to_pos) = item_holders.get(claw.move_to).unwrap();
        let from_pos = from_holder.alignment.get_item_pos(from_pos.origin);
        let to_pos = to_holder.alignment.get_item_pos(to_pos.origin);
        let anim_length = claw.anim_length();
        let current_tick = claw.current_anim_tick;
        let mut progress = current_tick as f32;
        if !claw.blocked {
            progress += tick_clock.get_tick_progress();
        }
        progress /= anim_length as f32 / 2.0;
        if progress > 1.0 {
            progress = 2.0 - progress;
        }
        let position_now = from_pos.lerp(to_pos, progress);
        transform.translation = (position_now, 0.2).into();
        if let Some(item) = claw.held_item {
            items
                .get_component_mut::<Item>(item)
                .unwrap()
                .anim_stationary_exact(position_now);
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
            let alignment = item_holder.alignment;
            let next = if let Some(next) = data.upstream {
                next
            } else {
                break;
            };
            if empty {
                let pos = pos.clone();
                let mut upstream_holder = all_conveyors.get_mut(next).unwrap().2;
                if upstream_holder.ticks_until_arrived == 0 {
                    let item = upstream_holder.incoming.take();
                    item.map(|e| {
                        let mut item = all_items.get_mut(e).unwrap();
                        item.anim_to_holder(pos.origin, alignment, CONVEYOR_DURATION);
                    });
                    let mut this_holder = all_conveyors.get_mut(current).unwrap().2;
                    this_holder.incoming = item;
                    this_holder.ticks_until_arrived = CONVEYOR_DURATION;
                }
            } else if item_holder.ticks_until_arrived == 0 {
                item_holder.incoming.map(|e| {
                    let mut item = all_items.get_mut(e).unwrap();
                    item.anim_stationary_in_holder(pos.origin, alignment);
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
        .add_system(tick_claws.system())
        .add_system(link_unlinked_conveyors.system())
        .add_system(tick_conveyors.system())
        .add_system(animate_claws.system())
        .add_system(animate_items.system())
        .add_system(tick_spawners.system())
        .add_system(tick_destroyers.system())
        .run();
}
