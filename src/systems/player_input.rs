use crate::prelude::*;

pub fn player_input(
    mut commands: Commands,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut gamelog: ResMut<GameLog>,
    player_position: Query<(Entity, &Position), With<Player>>,
    enemies: Query<(Entity, &Position), With<Enemy>>,
    items: Query<(Entity, &Position), With<Item>>,
    mut turn_state: ResMut<State<TurnState>>
) {

    let (player_ent, pos) = player_position.single();
    let mut action = true;
    let mut wait = false;

    let mut new_position = pos.clone();

    let key = keyboard_input.get_pressed().next().cloned();
    if let Some(key) = key {

        match key {
            KeyCode::Left => new_position.x -= 1,
            KeyCode::Right => new_position.x += 1,
            KeyCode::Down => new_position.y -= 1,
            KeyCode::Up => new_position.y += 1,
            KeyCode::G => {
                // Grab item at this position
                items.iter()
                    .filter(|(_, item_pos)| **item_pos == *pos)
                    .for_each(|(item_ent, _)| {
                        // remove render info and add carried component
                        commands.entity(item_ent).remove_bundle::<SpriteSheetBundle>()
                            .insert(Carried(player_ent));
                    }
                );
            }
            KeyCode::I => {
                turn_state.push(TurnState::InventoryPopup).unwrap();
                action = false;
            }
            KeyCode::E => {
                turn_state.push(TurnState::EquipmentPopup).unwrap();
                action = false;
            }
            _ => wait = true,
        }

        // move to new position   
        if new_position != *pos {
            // placeholder to know if it just a move or an attack
            let mut hit_something = false;
            // check if there is an enemy at the destination position
            enemies.iter()
                .filter(|(_, pos)| {
                    **pos == new_position
                })
                // if there's an enemy, say you hit something and send a WantsToAttack
                .for_each(|(victim, _) | {
                    hit_something = true;

                    commands.spawn()
                        .insert( WantsToAttack{attacker: player_ent, victim: victim});
                });

            // if it did not hit then it is just a movement
            if !hit_something {
                commands.spawn()
                    .insert( WantsToMove{entity: player_ent, destination: new_position});
            } 
        } 
        // else means the user clicked an action which did not move the player.
        else if wait {
            gamelog.add_entry("\nPlayer waits.".to_string());
        }

        // reset keyboard, bevys bug when changing states
        keyboard_input.reset(key);

        if action {
            // update state
            turn_state.set(TurnState::PlayerTurn).unwrap();
        }

    }
}
