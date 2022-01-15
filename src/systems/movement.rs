use crate::prelude::*;

pub fn movement(
    mut commands: Commands,
    mut mb: ResMut<MapBuilder>,
    move_messages: Query<(Entity, &WantsToMove)>,
    mut movers: Query<(Entity, &mut Position, &FieldOfView)>
) {
    // for every message to move
    for (message_ent, move_signal) in move_messages.iter() {

        // if the movement is physically valid
        if mb.map.can_enter_tile(move_signal.destination) {

            // if no other character is in that cell
            if mb.map.is_tile_occupied(move_signal.destination) {
            
                // get the entity and its alive status
                if let Ok((mov_ent, mut position, fov)) = movers.get_mut(move_signal.entity) {
                    // update occupation map
                    mb.move_entity_occupation(mov_ent, *position, move_signal.destination);
                    // and execute the movement
                    position.x = move_signal.destination.x;
                    position.y = move_signal.destination.y;
                    // and update the fov
                    commands.entity(mov_ent).insert(fov.clone_dirty());

                }
            }
        }
        // delete the message
        commands.entity(message_ent).despawn();
    }
}