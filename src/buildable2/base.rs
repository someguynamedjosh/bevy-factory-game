use bevy::prelude::*;
use dyn_clone::DynClone;

use super::{BuildingContext, MutBuildingMaps, WhichMap, BuildingComponentsContext};
use crate::prelude::*;

#[derive(Component)]
pub struct Built {
    pub buildable: Box<dyn Buildable>,
}

pub trait Buildable: DynClone + Sync + Send + 'static {
    fn shape(&self, ctx: &mut BuildingContext) -> Vec<IsoPos>;
    fn maps(&self) -> Vec<WhichMap>;
    fn extra_root_components(&self, ctx: &mut BuildingComponentsContext);
    #[allow(unused_variables)]
    fn spawn_extras(&self, ctx: &mut BuildingContext, maps: &mut MutBuildingMaps) -> Vec<Entity> {
        vec![]
    }
    fn spawn_art(&self, ctx: &mut BuildingContext) -> Vec<Entity>;
}
