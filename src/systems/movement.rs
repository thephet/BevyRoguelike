use crate::prelude::*;

pub fn movement(
    mut commands: Commands,
    mb: Res<MapBuilder>,
    mut movers: Query<(Entity, &WantsToMove)>,
) {

    for (ent, move_signal) in movers.iter_mut() {

        if mb.map.can_enter_tile(move_signal.destination) {
            commands.entity(move_signal.entity)
                .insert(move_signal.destination);
        }

        commands.entity(ent).despawn();
    }
}