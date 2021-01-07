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
pub enum ItemHolderAlignment {
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

pub struct Item {
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
            remaining_ticks: anim_duration - 1,
        };
    }

    pub fn anim_stationary_in_holder(&mut self, pos: IsoPos, alignment: ItemHolderAlignment) {
        self.anim = ItemAnim::Stay(alignment.get_item_pos(pos));
    }

    pub fn anim_stationary_exact(&mut self, pos: Vec2) {
        self.anim = ItemAnim::Stay(pos);
    }
}

pub fn spawn_item(
    commands: &mut Commands,
    common_assets: &Res<CommonAssets>,
    origin: IsoPos,
    alignment: ItemHolderAlignment,
) -> Entity {
    commands
        .spawn(SpriteBundle {
            material: common_assets.item_mat.clone(),
            ..Default::default()
        })
        .with(Item::new(alignment.get_item_pos(origin)))
        .current_entity()
        .unwrap()
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

pub struct ItemHolder {
    pub alignment: ItemHolderAlignment,
    pub item: Option<Entity>,
    pub blocked: bool,
}

impl ItemHolder {
    pub fn new_empty(alignment: ItemHolderAlignment) -> Self {
        Self::new_maybe_preloaded(alignment, None)
    }

    pub fn new_preloaded(alignment: ItemHolderAlignment, item: Entity) -> Self {
        Self::new_maybe_preloaded(alignment, Some(item))
    }

    pub fn new_maybe_preloaded(alignment: ItemHolderAlignment, item: Option<Entity>) -> Self {
        Self {
            alignment,
            item,
            blocked: false,
        }
    }

    /// Returns Some(item) if this holder is holding an item and is not blocked.
    pub fn try_take(&mut self) -> Option<Entity> {
        if self.blocked {
            None
        } else {
            self.item.take()
        }
    }

    pub fn try_put(&mut self, item: Entity) -> bool {
        if self.blocked {
            false
        } else if self.item.is_some() {
            false
        } else {
            self.item = Some(item);
            true
        }
    }

    pub fn try_put_from(&mut self, other: &mut Option<Entity>) {
        if !self.blocked && self.item.is_none() && other.is_some() {
            self.item = other.take();
        }
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(MAIN_STAGE, animate_items.system());
    }
}
