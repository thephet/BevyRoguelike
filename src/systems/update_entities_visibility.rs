use crate::prelude::*;

pub fn update_entities_visibility(
    mut gamelog: ResMut<GameLog>,
    player_fov_q: Query<&FieldOfView, With<Player>>,
    mut entities_q: Query<(Entity, &Position, &mut Visibility, Option<&MapTile>, Option<&mut Sprite>, Option<&mut TextureAtlasSprite>)>,
    names_q: Query<&Naming, Without<Player>>,
) {

    // get the player fov
    let player_fov = player_fov_q.single();

    // for every etity with position
    for (
        ent,
        pos, 
        mut vis, 
        map_tile, 
        sprite, 
        atlas_sprite
    ) in entities_q.iter_mut() {

        // first check if it is a map tile or some other entity. If it is a map tile...
        if let Some(_) = map_tile {
            if player_fov.visible_tiles.contains(&((*pos).into())) {
                // make it visible
                vis.is_visible = true;
                // increase the color alpha to 1, to both sprites or atlas_sprite
                if let Some(mut sprite) = sprite {
                    sprite.color.set_a(1.0);
                }
                if let Some(mut atlas_sprite) = atlas_sprite {
                    atlas_sprite.color.set_a(1.0);
                }
            } else if vis.is_visible == true { // if visible true but not in fov, ting
                // decrease the color alpha, to both sprites or atlas_sprite
                if let Some(mut sprite) = sprite {
                    sprite.color.set_a(0.3);
                }
                if let Some(mut atlas_sprite) = atlas_sprite {
                    atlas_sprite.color.set_a(0.2);
                }
            }
        } else { // if it is not a map tile, but some character
            // if this thing is on the player fov, make it visible
            if player_fov.visible_tiles.contains(&((*pos).into())) {
                // if it was not visible before, make it appear and describe in gamelog
                if vis.is_visible == false {
                    vis.is_visible = true;
                    // get name of entity and update gamelog
                    if let Ok(name) = names_q.get(ent) {
                        let message = format!("\n{} appears.", name.0);
                        gamelog.add_entry(message);
                    }
                }
                
            } else { // otherwise make it invisible
                vis.is_visible = false;
            }
        }
    }
}