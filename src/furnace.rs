use crate::item::{ItemContainer, ItemContainerAlignment};
use crate::prelude::*;
use bevy::prelude::*;

pub struct Furnace {
    input: Entity,
    output: Entity,
    cooking: bool,
    cook_time: u8,
}

// How long the recipe takes to cook.
const COOK_TIME: u8 = 10;

pub fn spawn_furnace(
    commands: &mut Commands,
    common_assets: &Res<CommonAssets>,
    origin: IsoPos,
    facing: IsoDirection,
) -> (Entity, Entity) {
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
    start_tile(commands, common_assets, origin, 3).with(Furnace {
        input,
        output,
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
        if input.item.is_some() && !furnace.cooking {
            let item = input.try_take().unwrap();
            commands.despawn(item);
            furnace.cooking = true;
        } 
        if furnace.cooking && furnace.cook_time < COOK_TIME {
            furnace.cook_time += 1;
            println!("time: {}", furnace.cook_time);
        }
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(fstage::TICK, tick.system());
    }
}
