use crate::prelude::*;

pub fn update_entities_visibility(
    mut gamelog: ResMut<GameLog>,
    player_fov_q: Query<&FieldOfView, With<Player>>,
    mut entities_q: Query<(Entity, &Position, &mut Visibility, Option<&MapTile>, Option<&mut Sprite>)>,
    names_enemies_q: Query<&Naming, With<Enemy>>,
    names_items_q: Query<&Naming, With<Item>>,
) {

    // get the player fov
    let player_fov = player_fov_q.single();

    // for every entity with position
    for (
        ent,
        pos, 
        mut vis, 
        map_tile, 
        sprite, 
    ) in entities_q.iter_mut() {

        // first check if it is a map tile or some other entity. If it is a map tile...
        if let Some(_) = map_tile {
            if player_fov.visible_tiles.contains(&((*pos).into())) {
                // make it visible
                *vis = Visibility::Visible;
                // increase the color alpha to 1, to both sprites or atlas_sprite
                if let Some(mut sprite) = sprite {
                    sprite.color.set_alpha(1.0);
                }
            } 
            else if *vis == Visibility::Visible { // if visible true but not in fov, tint
                // decrease the color alpha, to both sprites or atlas_sprite
                if let Some(mut sprite) = sprite {
                    sprite.color.set_alpha(0.1);
                }
            }
        } else { // if it is not a map tile, but some character or entity
            // if this thing is on the player fov, make it visible
            if player_fov.visible_tiles.contains(&((*pos).into())) {
                // if it was not visible before, make it appear and describe in gamelog
                if *vis == Visibility::Hidden {
                    *vis = Visibility::Visible;
                    // if enemy, get name update gamelog
                    if let Ok(name) = names_enemies_q.get(ent) {
                        let message = format!("{} appears.\n", name.0);
                        gamelog.add_entry(message);
                    }
                    // if item, provide hint
                    if let Ok(name) = names_items_q.get(ent) {
                        let message = format!("{}. Press G to grab.\n", name.0);
                        gamelog.add_entry(message);
                    }
                }
                
            } else { // otherwise make it invisible
                *vis = Visibility::Hidden;
            }
        }
    }
}