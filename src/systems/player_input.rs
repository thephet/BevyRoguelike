use crate::prelude::*;

pub fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_positions: Query<&mut Position, With<Player>>,
    mb: Res<MapBuilder>,
) {

    for mut pos in player_positions.iter_mut() {

        let mut new_position = pos.clone();
        if keyboard_input.pressed(KeyCode::Left) {
            new_position.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            new_position.x += 1;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            new_position.y -= 1;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            new_position.y += 1;
        }

        if mb.map.can_enter_tile(new_position) {
            pos.x = new_position.x;
            pos.y = new_position.y;
        }
    }
}