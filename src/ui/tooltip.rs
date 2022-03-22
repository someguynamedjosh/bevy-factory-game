use bevy::{prelude::*};

use super::{
    action::{Action, ActionState},
    cursor::CursorState,
};
use crate::{
    buildable::{
        BuildingMaps,
    },
    item::ItemContainer,
    prelude::*,
};

pub struct TooltipState {
    tool_text: Entity,
}

pub fn startup(mut commands: Commands, assets: Res<CommonAssets>) {
    let style = Style {
        align_self: AlignSelf::FlexStart,
        position_type: PositionType::Absolute,
        position: Rect {
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..Default::default()
        },
        ..Default::default()
    };
    let text = Text {
        sections: vec![TextSection {
            value: "Hello world!".to_owned(),
            style: TextStyle {
                font_size: 16.0,
                font: assets.font.clone(),
                ..Default::default()
            },
        }],
        ..Default::default()
    };
    let bundle = TextBundle {
        style,
        text,
        ..Default::default()
    };
    let tool_text = commands.spawn().insert_bundle(bundle).id();

    commands.insert_resource(TooltipState { tool_text });
}

pub fn update_post(
    maps: BuildingMaps,
    containers: Query<&ItemContainer>,
    mut texts: Query<&mut Text>,
    items: Query<&Item>,
    action_state: Res<ActionState>,
    cursor_state: Res<CursorState>,
    tooltip_state: Res<TooltipState>,
) {
    let hovered_container = maps.item_containers.get(cursor_state.world_pos).copied();

    let tooltip = match &action_state.action {
        Action::PlaceClawStart => format!("Claw Start"),
        Action::PlaceClawEnd { .. } => format!("Claw End"),
        Action::PlaceConveyor => format!("Conveyor"),
        Action::PlaceBuildable(bld) => format!("{:?}", bld),
        Action::Destroy => format!("Destroy"),
    };
    let mut text = texts.get_mut(tooltip_state.tool_text).unwrap();
    let hovered_item = if let Some(container) = hovered_container {
        if let Some(item) = containers.get(container).unwrap().item() {
            let item = items.get(item).unwrap();
            format!("{:?}", item.as_elements())
        } else {
            format!("")
        }
    } else {
        format!("")
    };
    text.sections[0].value = format!(
        "{}\n{}\n{}",
        tooltip, /* credits.0.floor() */ 0, hovered_item
    );
}
