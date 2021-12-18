use crate::prelude::*;

pub fn random_move(
    mb: Res<MapBuilder>,
    mut movers: Query<&mut Position, With<Enemy>>,
) {
    let mut rng = rand::thread_rng();

    // for each enemy
    for mut pos in movers.iter_mut() {
        // calculate a random destination
        let destination = match rng.gen_range(0..4) {
            0 => Position{x:-1, y:0, z:0},
            1 => Position{x:1, y:0, z:0},
            2 => Position{x:0, y:-1, z:0},
            _ => Position{x:0, y:1, z:0},
        } + *pos;

        // move it if the destination is good
        if mb.map.can_enter_tile(destination) {
            *pos = destination;
        }
    }
}