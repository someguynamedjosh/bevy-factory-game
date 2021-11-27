use crate::{building::{Shape, spawn_building, spawn_building_with_placeholder_art}, item::ItemContainer, prelude::*};
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

    pub fn get_shape(self) -> &'static Shape {
        // (||, T) -> the origin will always have a vertex pointing +T (side pointing -T)
        // If the direction is up, || is up, and T is left.
        match self {
            Self::Furnace => &Shape {
                blanks: &[(1, 0), (-1, 0), (1, 1), (-1, 1)],
                inputs: &[(0, 1)],
                outputs: &[(0, -1)],
            },
            Self::Mill => &Shape {
                blanks: &[(0, -1)],
                inputs: &[(1, -1)],
                outputs: &[(1, 0)],
            },
        }
    }

    pub fn get_appearence(
        self,
        assets: &CommonAssets,
    ) -> Option<(Handle<Mesh>, Handle<StandardMaterial>)> {
        match self {
            // Self::Furnace => Some((assets.furnace_mesh.clone(), assets.clay_mat.clone())),
            _ => None,
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
    obstruction_map: &mut ResMut<BuildingObstructionMap>,
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
    } = if let Some((mesh, mat)) = typ.get_appearence(&*common_assets) {
        spawn_building(
            commands,
            obstruction_map,
            mesh,
            mat,
            shape,
            origin,
            facing,
        )
    } else {
        spawn_building_with_placeholder_art(
            commands,
            common_assets,
            obstruction_map,
            shape,
            origin,
            facing,
        )
    };
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
