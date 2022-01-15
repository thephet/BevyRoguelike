use crate::prelude::*;

pub fn update_entities_visibility(
    player_fov_q: Query<&FieldOfView, With<Player>>,
    mut entities_q: Query<(&Position, &mut Visibility)>
) {

    // get the player fov
    let player_fov = player_fov_q.single();

    // for every tile
    for (pos, mut vis) in entities_q.iter_mut() {
        // if this tile is in the player fov
        if player_fov.visible_tiles.contains(&((*pos).into())) {
            // make it visible
            vis.is_visible = true;
        }
        else {
            vis.is_visible = false;
        }
    }
}