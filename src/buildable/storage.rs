use super::{
    machine::{self, Shape},
    Buildable, BuildingContext,
};
use crate::prelude::{IsoDirection, IsoPos};

#[derive(Clone, Debug)]
pub struct BSmallWarehouse;

const SHAPE: Shape = Shape {
    blanks: &[],
    inputs: &[(-1, 0)],
    outputs: &[],
};

impl Buildable for BSmallWarehouse {
    type ExtraData = ();

    fn shape(&self, position: IsoPos, direction: IsoDirection) -> Vec<IsoPos> {
        SHAPE.all_positions(position, direction).collect()
    }

    fn maps(&self) -> Vec<super::WhichMap> {
        todo!()
    }

    fn extra_root_components(
        &self,
        _ctx: &mut super::BuildingComponentsContext,
        _data: Self::ExtraData,
    ) {
    }

    fn spawn_extras(
        &self,
        _ctx: &mut BuildingContext,
        _maps: &mut super::BuildingMaps,
    ) -> (Vec<bevy::prelude::Entity>, Self::ExtraData) {
        (vec![], ())
    }

    fn spawn_art(&self, ctx: &mut BuildingContext) -> Vec<bevy::prelude::Entity> {
        machine::spawn_placeholder_art(ctx, &SHAPE)
    }
}
