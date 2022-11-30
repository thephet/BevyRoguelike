use crate::prelude::*;
use bevy::app::AppExit;

pub fn player_input(
    mut commands: Commands,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut game_log: ResMut<GameLog>,
    player_position: Query<(Entity, &Position), With<Player>>,
    enemies: Query<(Entity, &Position), With<Enemy>>,
    items: Query<(Entity, &Position, &Naming), With<Item>>,
    mut turn_state: ResMut<State<TurnState>>,
    mut exit: EventWriter<AppExit>
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
                    .filter(|(_, item_pos, _)| **item_pos == *pos)
                    .for_each(|(item_ent, _, name)| {
                        // remove render info and add carried component
                        commands.entity(item_ent).remove::<SpriteSheetBundle>()
                            .insert(Carried(player_ent));
                        let message = format!("\n{} grabbed.", name.0);
                        game_log.add_entry(message);
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
            KeyCode::Escape => {
                exit.send(AppExit);
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

                    commands.spawn(WantsToAttack{attacker: player_ent, victim });
                });

            // if it did not hit then it is just a movement
            if !hit_something {
                commands.spawn(WantsToMove{entity: player_ent, destination: new_position});
            } 
        } 
        // else means the user clicked an action which did not move the player.
        else if wait {
            game_log.add_entry("\nPlayer waits.".to_string());
        }

        // reset keyboard, bevys bug when changing states
        keyboard_input.reset(key);

        if action {
            // update state
            turn_state.set(TurnState::PlayerTurn).unwrap();
        }

    }
}

// If this is the first weapon we grab, also equip it
pub fn equip_first_weapon(
    mut commands: Commands,
    weapons_added: Query<Entity, (With<Weapon>, Added<Carried>)>,
    total_carried_weapons: Query<Entity, (With<Weapon>, With<Carried>)>,
) {
    for entity in weapons_added.iter() {
        // if we only have 1 weapon, equip it too
        if total_carried_weapons.iter().count() == 1 {
            commands.entity(entity).insert(Equipped);
        }
    }
}

// If this is the first weapon we grab, also equip it
pub fn equip_weapon_log(
    mut gamelog: ResMut<GameLog>,
    equipped_weapon: Query<(Entity, &Naming), (With<Weapon>, With<Carried>, Added<Equipped>)>,
) {
    for (_, name) in equipped_weapon.iter() {
        let message = format!("\n{} equipped.", name.0);
        gamelog.add_entry(message);
    }
}

pub struct PlayerInputPlugin;
impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app

        // listening to user input on inventory screen
        .add_system_set(
            SystemSet::on_update(TurnState::AwaitingInput)
                .with_system(player_input)
                .with_system(equip_first_weapon.after(player_input))
                .with_system(equip_weapon_log.after(equip_first_weapon))
        );
    }
}