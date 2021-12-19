use crate::prelude::*;

pub fn player_input(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut player_positions: Query<&mut Position, With<Player>>,
    mb: Res<MapBuilder>,
    mut turn_state: ResMut<State<TurnState>>
) {

    for mut pos in player_positions.iter_mut() {

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
            keyboard_input.reset(key);
        }

        if new_position != *pos {
            if mb.map.can_enter_tile(new_position) {
                // move to new position
                pos.x = new_position.x;
                pos.y = new_position.y;
                // update state
                turn_state.set(TurnState::PlayerTurn).unwrap();
    
            }
        }
    }
}