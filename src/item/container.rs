use bevy::prelude::*;

use super::ItemAnimator;
use crate::{item::spawn_item, prelude::*};

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

pub struct ItemContainerPos {
    pub pos: IsoPos,
    pub alignment: ItemContainerAlignment,
}

pub struct ItemContainer {
    alignment: ItemContainerAlignment,
    item: Option<Entity>,
    blocked: bool,
}

impl ItemContainer {
    pub fn new_empty(alignment: ItemContainerAlignment) -> Self {
        Self::new_maybe_preloaded(alignment, None)
    }

    pub fn new_maybe_preloaded(
        alignment: ItemContainerAlignment,
        item: Option<Entity>,
    ) -> Self {
        Self {
            alignment,
            item,
            blocked: false,
        }
    }

    pub fn alignment(&self) -> ItemContainerAlignment {
        self.alignment
    }

    pub fn item(&self) -> Option<Entity> {
        self.item
    }

    pub fn blocked(&self) -> bool {
        self.blocked
    }

    pub fn set_blocked(&self, blocked: bool) {
        self.blocked = blocked;
    }

    /// Returns Some(item) if this container is holding an item and is not
    /// blocked.
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

    // Will panic if blocked or already has an item.
    pub fn put_new_item(
        &mut self,
        commands: &mut Commands,
        common_assets: &Res<CommonAssets>,
        this_pos: IsoPos,
        item: Item,
    ) {
        assert!(self.item.is_none());
        assert!(!self.blocked);
        let item = spawn_item(commands, common_assets, item, this_pos, self.alignment);
        self.item = Some(item);
    }
}
