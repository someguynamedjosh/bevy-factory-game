use crate::{
    building::Shape,
    item::{ItemContainer, ItemContainerAlignment},
    prelude::*,
};
use bevy::prelude::*;

#[derive(Debug)]
pub struct Recipe {
    inputs: &'static [Item],
    time: u8,
    outputs: &'static [Item],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MachineType {
    Furnace,
    Mill,
}

impl MachineType {
    fn get_recipes(self) -> &'static [Recipe] {
        use Item::*;
        match self {
            Self::Furnace => &[Recipe {
                inputs: &[MetalRubble, MetalRubble],
                time: 40,
                outputs: &[Metal],
            }],
            Self::Mill => &[
                Recipe {
                    inputs: &[Metal, Metal, Metal],
                    time: 96,
                    outputs: &[ElectronicComponent, ElectronicComponent],
                },
                Recipe {
                    inputs: &[Metal],
                    time: 32,
                    outputs: &[StructuralComponent],
                },
            ],
        }
    }

    fn get_shape(self) -> &'static Shape {
        match self {
            Self::Furnace => &Shape {
                blanks: &[(0, 1), (-1, 1), (-1, -1), (0, -1)],
                inputs: &[(-1, 0)],
                outputs: &[(1, 0)],
            },
            Self::Mill => &Shape {
                blanks: &[(1, 0)],
                inputs: &[(0, -1)],
                outputs: &[(1, -1)],
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Machine {
    inputs: Vec<Entity>,
    output: Entity,

    recipes: &'static [Recipe],
    recipe: &'static Recipe,
    input_buffer: Vec<bool>,
    output_buffer: Vec<Item>,

    processing: bool,
    processing_time: u8,
}

pub fn spawn_machine(
    commands: &mut Commands,
    common_assets: &Res<CommonAssets>,
    typ: MachineType,
    origin: IsoPos,
    facing: IsoDirection,
) {
    let recipes = typ.get_recipes();
    let shape = typ.get_shape();
    // Machine shapes expect to have an edge in the direction the machine points in.
    assert!(!origin.has_vertex_pointing_in(facing));
    assert!(recipes.len() > 0);
    let BuildingResult {
        inputs,
        outputs,
        origin,
    } = spawn::building(commands, common_assets, shape, origin, facing);
    assert_eq!(outputs.len(), 1);
    let output = outputs[0];
    let recipe = &recipes[0];
    let input_buffer = vec![false; recipe.inputs.len()];

    let machine = Machine {
        inputs,
        output,
        recipes,
        recipe,
        input_buffer,
        output_buffer: Vec::new(),
        processing: false,
        processing_time: 0,
    };
    commands.set_current_entity(origin);
    commands.with(machine);
}

fn tick(
    commands: &mut Commands,
    common_assets: Res<CommonAssets>,
    mut machines: Query<(&mut Machine,)>,
    mut containers: Query<(&mut ItemContainer, &IsoPos)>,
    items: Query<&Item>,
) {
    for (mut machine,) in machines.iter_mut() {
        let recipe = machine.recipe;
        let (mut output, pos) = containers.get_mut(machine.output).unwrap();
        if output.item.is_none() {
            if machine.processing_time == recipe.time && machine.output_buffer.is_empty() {
                for item in recipe.outputs {
                    machine.output_buffer.push(item.clone());
                }
                machine.processing = false;
                machine.processing_time = 0;
            }
            if let Some(item) = machine.output_buffer.pop() {
                output.put_new_item(commands, &common_assets, *pos, item);
            }
        }

        for input in 0..machine.inputs.len() {
            let input = machine.inputs[input];
            let mut input = containers.get_mut(input).unwrap().0;
            if let Some(item_ent) = input.item {
                let item = items.get(item_ent).unwrap();
                for idx in 0..recipe.inputs.len() {
                    if machine.input_buffer[idx] {
                        continue;
                    }
                    if *item == recipe.inputs[idx] {
                        machine.input_buffer[idx] = true;
                        input.item = None;
                        commands.despawn(item_ent);
                        break;
                    }
                }
            }
        }
        if !machine.processing && machine.input_buffer.iter().all(|&stored| stored) {
            machine.processing = true;
            machine.processing_time = 0;
            for stored in &mut machine.input_buffer {
                *stored = false;
            }
        }
        if machine.processing && machine.processing_time < recipe.time {
            machine.processing_time += 1;
        }
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(fstage::TICK, tick.system());
    }
}
