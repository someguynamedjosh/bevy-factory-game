use super::*;

pub(super) struct ContainerDebug(Entity);

pub(super) fn attach_debug(
    commands: &mut Commands,
    common_assets: Res<CommonAssets>,
    add_to: Query<(Entity, &IsoPos, &ItemContainer), Without<ContainerDebug>>,
) {
    for (id, pos, container) in add_to.iter() {
        commands
            .spawn(SpriteBundle {
                material: common_assets.debug_container_mat.clone(),
                transform: Transform::from_translation(
                    (container.alignment.get_item_pos(*pos), 0.2).into(),
                ) * SPRITE_TRANSFORM,
                ..Default::default()
            })
            .with(ContainerDebug(id));
    }
}

pub(super) fn animate(
    common_assets: Res<CommonAssets>,
    mut debugs: Query<(&ContainerDebug, &mut Handle<ColorMaterial>)>,
    containers: Query<&ItemContainer>,
) {
    for (debug, mut material) in debugs.iter_mut() {
        let blocked = containers.get(debug.0).unwrap().blocked;
        *material = if blocked {
            common_assets.debug_blocked_container_mat.clone()
        } else {
            common_assets.debug_container_mat.clone()
        };
    }
}
