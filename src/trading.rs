use crate::{
    building::{spawn_building_with_placeholder_art, Shape},
    item::ItemContainer,
    prelude::*,
};
use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct CreditBalance(pub f64);

pub const SELLER_SHAPE: Shape = Shape {
    blanks: &[(0, 1), (0, -1), (-1, 0), (-1, 1), (-1, -1)],
    inputs: &[(1, 0), (0, 2), (0, -2), (-1, 2), (-1, -2), (-2, 0)],
    outputs: &[],
};

struct Seller {
    inputs: Vec<Entity>,
}

pub fn spawn_seller(
    commands: &mut Commands,
    common_assets: &Res<CommonAssets>,
    obstruction_map: &mut ResMut<BuildingObstructionMap>,
    origin: IsoPos,
    facing: IsoDirection,
) {
    let res = spawn_building_with_placeholder_art(
        commands,
        common_assets,
        obstruction_map,
        &SELLER_SHAPE,
        origin,
        facing,
    );
    let seller = Seller { inputs: res.inputs };
    commands.set_current_entity(res.origin);
    commands.with(seller);
}

fn tick(
    commands: &mut Commands,
    mut credits: ResMut<CreditBalance>,
    sellers: Query<&Seller>,
    mut containers: Query<&mut ItemContainer>,
    items: Query<&Item>,
) {
    for seller in sellers.iter() {
        for &input in &seller.inputs {
            let mut container = containers.get_mut(input).unwrap();
            if let Some(entity) = container.try_take() {
                let item = items.get(entity).unwrap();
                credits.0 += item.get_properties().base_market_value as f64;
                commands.despawn(entity);
            }
        }
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(fstage::TICK, tick.system());
        app.add_resource(CreditBalance(0.0));
    }
}
