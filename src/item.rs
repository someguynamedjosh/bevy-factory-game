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
        Some(ReferenceItem::IronOre) => common_assets.iron_ore_mat.clone(),
        Some(ReferenceItem::IronNugget) => common_assets.iron_mat.clone(),
        None => common_assets.item_mat.clone(),
    };
    commands
        .spawn()
        .insert_bundle(PbrBundle {
            material,
            mesh: common_assets.quad_mesh.clone(),
            transform: sprite_transform(),
            ..Default::default()
        })
        .insert(item)
        .insert(ItemAnimator::new(alignment.get_item_pos(origin)))
        .id()
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(fstage::ANIMATION, animate_items.system());

        if cfg!(feature = "draw-containers") {
            app.add_system_to_stage(fstage::SETUP, container_debug::attach_debug.system());
            app.add_system_to_stage(fstage::ANIMATION, container_debug::animate.system());
        }
    }
}
