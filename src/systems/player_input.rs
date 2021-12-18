use crate::prelude::*;

pub fn player_input(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut player_positions: Query<&mut Position, With<Player>>,
    mb: Res<MapBuilder>,
    mut turn_state: ResMut<State<TurnState>>
) {

    for mut pos in player_positions.iter_mut() {

        if let Some(key) = keyboard_input.get_pressed().next() {
            match key {
                KeyCode::Left => println!("left"),
                _ => (),
            }
        }

        let mut new_position = pos.clone();
        if keyboard_input.pressed(KeyCode::Left) {
            new_position.x -= 1;
            keyboard_input.reset(KeyCode::Left);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            new_position.x += 1;
            keyboard_input.reset(KeyCode::Right);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            new_position.y -= 1;
            keyboard_input.reset(KeyCode::Down);
        }
        if keyboard_input.pressed(KeyCode::Up) {
            new_position.y += 1;
            keyboard_input.reset(KeyCode::Up);
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