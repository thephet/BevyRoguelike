use crate::prelude::*;

pub fn collisions(
    mut commands: Commands,
    enemy_positions: Query<(Entity, &Position), With<Enemy>>,
    player_query: Query<&Position, (Changed<Position>, With<Player>)>,
) {

    // get the player position based on the query
    if let Ok(player_position) = player_query.single() {
        // iterate through all the enemies
        for (ent, enemy_pos) in enemy_positions.iter() {
            // if player and enemy are at the same position
            if player_position == enemy_pos {
                // remove the enemy
                commands.entity(ent).despawn();
            }
        }
    }

}