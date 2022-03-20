use crate::prelude::*;

/// Defines the visual/physical structure of a machine.
#[derive(Debug)]
pub struct Shape {
    pub blanks: &'static [(i32, i32)],
    pub inputs: &'static [(i32, i32)],
    pub outputs: &'static [(i32, i32)],
}

pub struct ShapeIters<T> {
    pub blanks: T,
    pub inputs: T,
    pub outputs: T,
}

impl Shape {
    fn positions_impl(
        def: &'static [(i32, i32)],
        origin: IsoPos,
        facing: IsoDirection,
    ) -> impl Iterator<Item = IsoPos> {
        def.iter()
            .map(move |&(perp, par)| origin.offset_both_direction(facing, par, perp))
    }

    pub fn positions(
        &self,
        origin: IsoPos,
        facing: IsoDirection,
    ) -> ShapeIters<impl Iterator<Item = IsoPos>> {
        ShapeIters {
            blanks: Self::positions_impl(self.blanks, origin, facing),
            inputs: Self::positions_impl(self.inputs, origin, facing),
            outputs: Self::positions_impl(self.outputs, origin, facing),
        }
    }
}
