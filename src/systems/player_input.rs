use crate::prelude::*;

pub fn player_input(
    mut commands: Commands,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    player_position: Query<(Entity, &Position), With<Player>>,
    mut turn_state: ResMut<State<TurnState>>
) {

    if let Ok((ent, pos)) = player_position.single() {

        let mut new_position = pos.clone();

        let key = keyboard_input.get_pressed().next().cloned();
        if let Some(key) = key {

            match key {
                KeyCode::Left => new_position.x -= 1,
                KeyCode::Right => new_position.x += 1,
                KeyCode::Down => new_position.y -= 1,
                KeyCode::Up => new_position.y += 1,
                _ => (),
            }

            // move to new position   
            if new_position != *pos {      
                commands.spawn()
                    .insert( WantsToMove{entity: ent, destination: new_position});
            }

            // reset keyboard, bevys bug when changing states
            keyboard_input.reset(key);
            // update state
            turn_state.set(TurnState::PlayerTurn).unwrap();
        }
    }
}