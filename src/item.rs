use crate::prelude::*;
use bevy::prelude::*;

enum ItemAnim {
    Stay(Vec2),
    Lerp {
        from: Vec2,
        to: Vec2,
        total_ticks: u8,
        remaining_ticks: u8,
    },
}

#[derive(Clone, Copy)]
pub enum ItemContainerAlignment {
    Centroid,
    AxisAligned(IsoAxis),
}

impl ItemContainerAlignment {
    pub fn get_item_pos(&self, coarse_pos: IsoPos) -> Vec2 {
        match self {
            Self::Centroid => coarse_pos.centroid_pos(),
            Self::AxisAligned(axis) => coarse_pos.axis_aligned_pos(*axis),
        }
    }
}

pub enum Item {
    MetalRubble,
    Metal,
}

pub struct ItemAnimator {
    anim: ItemAnim,
}

impl ItemAnimator {
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

    pub fn anim_to_container(
        &mut self,
        pos: IsoPos,
        alignment: ItemContainerAlignment,
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

    pub fn anim_stationary_in_container(&mut self, pos: IsoPos, alignment: ItemContainerAlignment) {
        self.anim = ItemAnim::Stay(alignment.get_item_pos(pos));
    }

    pub fn anim_stationary_exact(&mut self, pos: Vec2) {
        self.anim = ItemAnim::Stay(pos);
    }
}

pub fn spawn_item(
    commands: &mut Commands,
    common_assets: &Res<CommonAssets>,
    item: Item,
    origin: IsoPos,
    alignment: ItemContainerAlignment,
) -> Entity {
    let material = match &item {
        Item::MetalRubble => common_assets.metal_rubble_mat.clone(),
        Item::Metal => common_assets.metal_mat.clone(),
    };
    commands
        .spawn(SpriteBundle {
            material,
            ..Default::default()
        })
        .with(item)
        .with(ItemAnimator::new(alignment.get_item_pos(origin)))
        .current_entity()
        .unwrap()
}

fn animate_items(
    tick_clock: Res<TickClock>,
    mut items: Query<(&mut Transform, &mut ItemAnimator)>,
) {
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
                let progress = if cfg!(feature = "no-interpolation") {
                    (*total_ticks - *remaining_ticks - 1) as f32
                } else {
                    (*total_ticks - *remaining_ticks - 1) as f32 + tick_clock.get_tick_progress()
                } / *total_ticks as f32;
                from.lerp(*to, progress)
            }
        };
        transform.translation = (pos, 0.1).into();
    }
}

pub struct ItemContainer {
    pub alignment: ItemContainerAlignment,
    pub item: Option<Entity>,
    pub blocked: bool,
}

impl ItemContainer {
    pub fn new_empty(alignment: ItemContainerAlignment) -> Self {
        Self::new_maybe_preloaded(alignment, None)
    }

    pub fn new_preloaded(alignment: ItemContainerAlignment, item: Entity) -> Self {
        Self::new_maybe_preloaded(alignment, Some(item))
    }

    pub fn new_maybe_preloaded(alignment: ItemContainerAlignment, item: Option<Entity>) -> Self {
        Self {
            alignment,
            item,
            blocked: false,
        }
    }

    /// Returns Some(item) if this container is holding an item and is not blocked.
    pub fn try_take(&mut self) -> Option<Entity> {
        if self.blocked {
            None
        } else {
            self.item.take()
        }
    }

    pub fn try_put_from(
        &mut self,
        other: &mut Option<Entity>,
        this_pos: IsoPos,
        item_query: &mut Query<&mut ItemAnimator>,
    ) {
        if !self.blocked && self.item.is_none() && other.is_some() {
            self.item = other.take();
            if let Some(item) = self.item {
                item_query
                    .get_mut(item)
                    .unwrap()
                    .anim_stationary_in_container(this_pos, self.alignment)
            }
        }
    }
}

// #[cfg(feature = "draw-containers")]
mod container_debug {
    use super::*;

    pub(super) struct ContainerDebug(Entity);

    pub(super) fn attach_debug(
        commands: &mut Commands,
        common_assets: Res<CommonAssets>,
        add_to: Query<(Entity, &IsoPos, &ItemContainer), Without<ContainerDebug>>,
    ) {
        for (id, pos, container) in add_to.iter() {
            commands
                .spawn(SpriteBundle {
                    material: common_assets.debug_container_mat.clone(),
                    transform: Transform::from_translation(
                        (container.alignment.get_item_pos(*pos), 10.0).into(),
                    ),
                    ..Default::default()
                })
                .with(ContainerDebug(id));
        }
    }

    pub(super) fn animate(
        common_assets: Res<CommonAssets>,
        mut debugs: Query<(&ContainerDebug, &mut Handle<ColorMaterial>)>,
        containers: Query<(&ItemContainer,)>,
    ) {
        for (debug, mut material) in debugs.iter_mut() {
            let blocked = containers.get(debug.0).unwrap().0.blocked;
            *material = if blocked {
                common_assets.debug_blocked_container_mat.clone()
            } else {
                common_assets.debug_container_mat.clone()
            };
        }
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(fstage::ANIMATION, animate_items.system());

        if cfg!(feature = "draw-containers") {
            app.add_system_to_stage(fstage::SETUP, container_debug::attach_debug.system());
            app.add_system_to_stage(fstage::ANIMATION, container_debug::animate.system());
        }
    }
}
