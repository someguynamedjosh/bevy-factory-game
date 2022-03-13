use bevy::prelude::*;
use itertools::Itertools;

use crate::{
    building::{BuildingSpawner, Shape},
    item::{spawn_item, Element, ItemContainer},
    prelude::*,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MachineType {
    Purifier,
    Joiner,
}

impl MachineType {
    pub fn processing_time(self) -> u8 {
        match self {
            _ => 12,
        }
    }

    pub fn process(self, items: Vec<Item>) -> Vec<Item> {
        match self {
            Self::Purifier => {
                let (item,) = items.into_iter().collect_tuple().unwrap();
                vec![item.with_modified_elements(|xs| xs.filter(|x| x != &Element::Impurity))]
            }
            Self::Joiner => {
                let (a, b) = items.into_iter().collect_tuple().unwrap();
                let item = [a.into_elements(), b.into_elements()].concat().into();
                vec![item]
            }
        }
    }

    pub fn get_shape(self) -> &'static Shape {
        // (T, ||) -> the origin will always have a vertex pointing +T (side pointing
        // -T) If the direction is up, || is up, and T is left.
        match self {
            Self::Purifier => &Shape {
                blanks: &[(0, 1), (0, -1), (1, 1), (1, -1)],
                inputs: &[(1, 0)],
                outputs: &[(-1, 0)],
            },
            Self::Joiner => &Shape {
                blanks: &[],
                inputs: &[(0, 1), (0, -1)],
                outputs: &[(-1, 0)],
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
    outputs: Vec<Entity>,

    typ: MachineType,
    input_buffer: Vec<Option<Item>>,
    output_buffer: Vec<Item>,

    processing_time: u8,
}

impl Machine {
    pub fn processing(&self) -> bool {
        self.input_buffer.iter().all(Option::is_some)
    }
}

pub fn spawn_machine(
    commands: &mut Commands,
    common_assets: &Res<CommonAssets>,
    obstruction_map: &mut ResMut<BuildingObstructionMap>,
    typ: MachineType,
    origin: IsoPos,
    facing: IsoDirection,
) {
    let shape = typ.get_shape();
    // Machine shapes expect to have an edge in the direction the machine points in.
    assert!(!origin.has_vertex_pointing_in(facing));
    let BuildingResult {
        inputs,
        outputs,
        origin,
        art,
    } = if let Some((mesh, mat)) = typ.get_appearence(&*common_assets) {
        BuildingSpawner::start(
            commands,
            Some(mesh),
            Some(mat),
            obstruction_map,
            shape,
            origin,
            facing,
        )
        .finish()
    } else {
        BuildingSpawner::start_with_placeholder_art(
            commands,
            Some(common_assets),
            obstruction_map,
            shape,
            origin,
            facing,
        )
        .finish()
    };

    let machine = Machine {
        input_buffer: vec![None; inputs.len()],
        inputs,
        outputs,
        typ,
        output_buffer: Vec::new(),
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
        let done = machine.processing_time == machine.typ.processing_time();
        let mut can_output = done;
        for &output in &machine.outputs {
            let (output, _) = containers.get_mut(output).unwrap();
            if output.item().is_some() {
                can_output = false;
                break;
            }
        }

        if machine.processing() {
            if done && can_output {
                let mut inputs = Vec::new();
                for input in &mut machine.input_buffer {
                    inputs.push(input.take().unwrap());
                }
                let results = machine.typ.process(inputs);
                assert_eq!(results.len(), machine.outputs.len());
                for (result, &output) in results.into_iter().zip(machine.outputs.iter()) {
                    let (mut output, pos) = containers.get_mut(output).unwrap();
                    output.create_and_put_item(commands, &common_assets, *pos, result);
                }
                machine.processing_time = 0;
            }
        }

        let Machine {
            inputs,
            input_buffer,
            ..
        } = &mut *machine;

        for (&container, buffer) in inputs.iter().zip(input_buffer.iter_mut()) {
            let (mut container, _) = containers.get_mut(container).unwrap();
            if buffer.is_none() {
                if let Some(item) = container.try_take() {
                    commands.despawn(item);
                    let item = items.get(item).unwrap().clone();
                    *buffer = Some(item);
                }
            }
        }

        let done = machine.processing_time == machine.typ.processing_time();

        if machine.processing() && !done {
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
