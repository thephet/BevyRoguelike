use crate::prelude::*;

pub fn random_move(
    mut commands: Commands,
    movers: Query<(Entity, &Position), With<Enemy>>,
) {
    let mut rng = rand::thread_rng();

    // for each enemy
    for (ent, pos) in movers.iter() {
        // calculate a random destination
        let destination = match rng.gen_range(0..4) {
            0 => Position{x:-1, y:0, z:0},
            1 => Position{x:1, y:0, z:0},
            2 => Position{x:0, y:-1, z:0},
            _ => Position{x:0, y:1, z:0},
        } + *pos;

        if destination != *pos {
            // move to new position         
            commands.spawn()
                .insert( WantsToMove{entity: ent, destination: destination});
        }
    }
}