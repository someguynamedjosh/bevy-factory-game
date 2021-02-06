use crate::item::{ItemContainer, ItemContainerAlignment};
use crate::prelude::*;
use bevy::prelude::*;

pub struct Mill {
    input: Entity,
    output: Entity,
    recipe: MillRecipe,
    ingredient_buffer: Vec<(Item, bool)>,
    output_buffer: Vec<Item>,
    cooking: bool,
    cook_time: u8,
}

#[derive(Clone, Copy)]
pub enum MillRecipe {
    MakeElectronicComponents,
    MakeStructuralComponents,
}

impl MillRecipe {
    pub fn get_inputs(&self) -> Vec<Item> {
        match self {
            Self::MakeElectronicComponents => vec![Item::Metal, Item::Metal, Item::Metal],
            Self::MakeStructuralComponents => vec![Item::Metal],
        }
    }

    // How long the recipe takes to process.
    pub fn get_time(&self) -> u8 {
        match self {
            Self::MakeElectronicComponents => 96,
            Self::MakeStructuralComponents => 32,
        }
    }

    pub fn get_products(&self) -> Vec<Item> {
        match self {
            Self::MakeElectronicComponents => {
                vec![Item::ElectronicComponent, Item::ElectronicComponent]
            }
            Self::MakeStructuralComponents => vec![Item::StructuralComponent],
        }
    }
}

pub fn spawn_mill(
    commands: &mut Commands,
    common_assets: &Res<CommonAssets>,
    origin: IsoPos,
    facing: IsoDirection,
) -> (Entity, Entity) {
    let recipe = MillRecipe::MakeStructuralComponents;

    // We should have an edge facing the direction for the building to appear visually correct.
    assert!(!origin.has_vertex_pointing_in(facing));
    for (offset, perp_offset) in &[(1, 0)] {
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
        origin
            .offset_direction(facing, 0)
            .offset_perp_direction(facing, -1),
        1,
    )
    .with(ItemContainer::new_empty(ItemContainerAlignment::Centroid))
    .current_entity()
    .unwrap();
    let output = start_tile(
        commands,
        common_assets,
        origin
            .offset_direction(facing, 1)
            .offset_perp_direction(facing, -1),
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
    start_tile(commands, common_assets, origin, 3).with(Mill {
        input,
        output,
        recipe,
        ingredient_buffer,
        output_buffer: Vec::new(),
        cooking: false,
        cook_time: 0,
    });
    (input, output)
}

fn tick(
    commands: &mut Commands,
    common_assets: Res<CommonAssets>,
    mut mills: Query<(&mut Mill,)>,
    mut containers: Query<(&mut ItemContainer, &IsoPos)>,
    items: Query<&Item>,
) {
    for (mut mill,) in mills.iter_mut() {
        let (mut output, pos) = containers.get_mut(mill.output).unwrap();
        if output.item.is_none() {
            if mill.cook_time >= mill.recipe.get_time() && mill.output_buffer.is_empty() {
                mill.output_buffer = mill.recipe.get_products();
                mill.cooking = false;
                mill.cook_time = 0;
            }
            if let Some(item) = mill.output_buffer.pop() {
                output.put_new_item(commands, &common_assets, *pos, item);
            }
        }

        let mut input = containers.get_mut(mill.input).unwrap().0;
        if let Some(item_ent) = input.item {
            let item = items.get(item_ent).unwrap();
            for (required_item, stored) in &mut mill.ingredient_buffer {
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
        if !mill.cooking && mill.ingredient_buffer.iter().all(|(_, stored)| *stored) {
            mill.cooking = true;
            mill.cook_time = 0;
            for (_, stored) in &mut mill.ingredient_buffer {
                *stored = false;
            }
        }
        if mill.cooking {
            mill.cook_time += 1;
        }
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(fstage::TICK, tick.system());
    }
}
