pub use std::f32::consts::{PI, TAU};

use bevy::ecs::{Commands, DynamicBundle};
pub use bevy::{
    math::Vec2,
    prelude::{Entity, Quat, Transform},
};
pub use scones::make_constructor;

pub use crate::{
    assets::CommonAssets,
    buildable::{BuildingResult, Shape},
    common::{fstage, start_tile, SetupNeeded, TickClock, TileVariant, SPRITE_TRANSFORM},
    iso::{BuildingObstructionMap, ConveyorMap, IsoAxis, IsoDirection, IsoPos, Snapping},
    item::Item,
};

pub trait SpawnWithBundles<Input> {
    fn spawn_with_bundles(&mut self, bundles: Input) -> &mut Self;
}

impl<A: DynamicBundle + Send + Sync + 'static> SpawnWithBundles<(A,)> for Commands {
    fn spawn_with_bundles(&mut self, bundles: (A,)) -> &mut Self {
        self.spawn(bundles.0)
    }
}

impl<A: DynamicBundle + Send + Sync + 'static, B: DynamicBundle + Send + Sync + 'static>
    SpawnWithBundles<(A, B)> for Commands
{
    fn spawn_with_bundles(&mut self, bundles: (A, B)) -> &mut Self {
        self.spawn(bundles.0).with_bundle(bundles.1)
    }
}

impl<
        A: DynamicBundle + Send + Sync + 'static,
        B: DynamicBundle + Send + Sync + 'static,
        C: DynamicBundle + Send + Sync + 'static,
    > SpawnWithBundles<(A, B, C)> for Commands
{
    fn spawn_with_bundles(&mut self, bundles: (A, B, C)) -> &mut Self {
        self.spawn(bundles.0)
            .with_bundle(bundles.1)
            .with_bundle(bundles.2)
    }
}

impl<
        A: DynamicBundle + Send + Sync + 'static,
        B: DynamicBundle + Send + Sync + 'static,
        C: DynamicBundle + Send + Sync + 'static,
        D: DynamicBundle + Send + Sync + 'static,
    > SpawnWithBundles<(A, B, C, D)> for Commands
{
    fn spawn_with_bundles(&mut self, bundles: (A, B, C, D)) -> &mut Self {
        self.spawn(bundles.0)
            .with_bundle(bundles.1)
            .with_bundle(bundles.2)
            .with_bundle(bundles.3)
    }
}

impl<
        A: DynamicBundle + Send + Sync + 'static,
        B: DynamicBundle + Send + Sync + 'static,
        C: DynamicBundle + Send + Sync + 'static,
        D: DynamicBundle + Send + Sync + 'static,
        E: DynamicBundle + Send + Sync + 'static,
    > SpawnWithBundles<(A, B, C, D, E)> for Commands
{
    fn spawn_with_bundles(&mut self, bundles: (A, B, C, D, E)) -> &mut Self {
        self.spawn(bundles.0)
            .with_bundle(bundles.1)
            .with_bundle(bundles.2)
            .with_bundle(bundles.3)
            .with_bundle(bundles.4)
    }
}
