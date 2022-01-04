use crate::prelude::*;

pub fn movement(
    mut commands: Commands,
    mb: Res<MapBuilder>,
    move_messages: Query<(Entity, &WantsToMove)>,
    movers: Query<(Entity, &Alive)>
) {
    // for every message to move
    for (message_ent, move_signal) in move_messages.iter() {

        // if the movement is valid
        if mb.map.can_enter_tile(move_signal.destination) {
            
            // get the entity and its alive status
            if let Ok((mover_entity, alive)) = movers.get(move_signal.entity) {
                if alive.0 == true {
                    // and execute the movement
                    commands.entity(mover_entity).insert(move_signal.destination);
                }

            }
        }
        // delete the message
        commands.entity(message_ent).despawn();
    }
}