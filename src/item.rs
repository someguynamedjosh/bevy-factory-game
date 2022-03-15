mod animation;
mod animator;
mod base;
mod container;
mod container_debug;
mod element;
mod reference_item;

use bevy::prelude::*;

pub use self::{animator::*, base::*, container::*, element::*, reference_item::*};
use crate::prelude::*;

pub fn spawn_item(
    commands: &mut Commands,
    common_assets: &Res<CommonAssets>,
    item: Item,
    origin: IsoPos,
    alignment: ItemContainerAlignment,
) -> Entity {
    let material = match item.as_known_item() {
        Some(ReferenceItem::IronOre) => common_assets.metal_rubble_mat.clone(),
        Some(ReferenceItem::IronNugget) => common_assets.metal_mat.clone(),
        None => common_assets.claw_mat.clone(),
    };
    commands
        .spawn()
        .with_bundle(SpriteBundle {
            material,
            transform: SPRITE_TRANSFORM,
            ..Default::default()
        })
        .with(item)
        .with(ItemAnimator::new(alignment.get_item_pos(origin)))
        .current_entity()
        .unwrap()
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
