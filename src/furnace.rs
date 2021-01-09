use crate::item::{ItemContainer, ItemContainerAlignment};
use crate::prelude::*;
use bevy::prelude::*;

pub struct Furnace {
    input: Entity,
    output: Entity,
    ingredient_buffer: Vec<(Item, bool)>,
    cooking: bool,
    cook_time: u8,
}

pub enum FurnaceRecipe {
    RefineMetal,
}

impl FurnaceRecipe {
    pub fn get_inputs(&self) -> Vec<Item> {
        match self {
            Self::RefineMetal => vec![Item::MetalRubble, Item::MetalRubble],
        }
    }
}

// How long the recipe takes to cook.
const COOK_TIME: u8 = 16;

pub fn spawn_furnace(
    commands: &mut Commands,
    common_assets: &Res<CommonAssets>,
    origin: IsoPos,
    facing: IsoDirection,
) -> (Entity, Entity) {
    let recipe = FurnaceRecipe::RefineMetal;

    // We should have an edge facing the direction for the building to appear visually correct.
    assert!(!origin.has_vertex_pointing_in(facing));
    for (offset, perp_offset) in &[(0, 1), (-1, 1), (-1, -1), (0, -1)] {
        start_tile(
            commands,
            common_assets,
            origin
                .offset_direction(facing, *offset)
                .offset_perp_direction(facing, *perp_offset),
            0,
        );
    }
    let input = start_tile(
        commands,
        common_assets,
        origin.offset_direction(facing, -1),
        1,
    )
    .with(ItemContainer::new_empty(ItemContainerAlignment::Centroid))
    .current_entity()
    .unwrap();
    let output = start_tile(
        commands,
        common_assets,
        origin.offset_direction(facing, 1),
        2,
    )
    .with(ItemContainer::new_empty(ItemContainerAlignment::Centroid))
    .current_entity()
    .unwrap();

    let ingredient_buffer = recipe
        .get_inputs()
        .into_iter()
        .map(|i| (i, false))
        .collect();
    start_tile(commands, common_assets, origin, 3).with(Furnace {
        input,
        output,
        ingredient_buffer,
        cooking: false,
        cook_time: 0,
    });
    (input, output)
}

fn tick(
    commands: &mut Commands,
    common_assets: Res<CommonAssets>,
    mut furnaces: Query<(&mut Furnace,)>,
    mut containers: Query<(&mut ItemContainer, &IsoPos)>,
    items: Query<&Item>,
) {
    for (mut furnace,) in furnaces.iter_mut() {
        if furnace.cook_time == COOK_TIME {
            let (mut output, pos) = containers.get_mut(furnace.output).unwrap();
            if output.item.is_none() {
                let item = Item::Metal;
                output.put_new_item(commands, &common_assets, *pos, item);
                furnace.cooking = false;
                furnace.cook_time = 0;
            }
        }

        let mut input = containers.get_mut(furnace.input).unwrap().0;
        if let Some(item_ent) = input.item {
            let item = items.get(item_ent).unwrap();
            for (required_item, stored) in &mut furnace.ingredient_buffer {
                if *stored {
                    continue;
                }
                if *item == *required_item {
                    *stored = true;
                    input.item = None;
                    commands.despawn(item_ent);
                    break;
                }
            }
        }
        if !furnace.cooking && furnace.ingredient_buffer.iter().all(|(_, stored)| *stored) {
            furnace.cooking = true;
            furnace.cook_time = 0;
            for (_, stored) in &mut furnace.ingredient_buffer {
                *stored = false;
            }
        }
        if furnace.cooking && furnace.cook_time < COOK_TIME {
            furnace.cook_time += 1;
        }
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(fstage::TICK, tick.system());
    }
}
