use bevy::prelude::*;
use itertools::Itertools;

use super::machine_type::MachineType;
use crate::{
    buildable::{BuildingSpawner, Shape},
    item::{Element, ItemContainer},
    prelude::*,
};

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
    let builder = BuildingSpawner::start(commands, obstruction_map, shape, origin, facing);
    let BuildingResult {
        inputs,
        outputs,
        origin,
        art,
    } = if let Some((mesh, mat)) = typ.get_appearence(&*common_assets) {
        builder.with_bespoke_art(mesh, mat).finish()
    } else {
        builder.with_placeholder_art(common_assets).finish()
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
