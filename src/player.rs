use crate::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    atlas: Res<CharsetAsset>,
) {

    commands
    .spawn_bundle(SpriteSheetBundle {
        texture_atlas: atlas.atlas.clone(),
        sprite: TextureAtlasSprite::new(64),
        ..Default::default()
    })
    .insert(Position { x: 40, y: 25, z: 1 })
    .insert(TileSize::square(1.0))
    .insert(Player);
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_positions: Query<&mut Position, With<Player>>,
    map: Res<Map>,
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

        if map.can_enter_tile(new_position) {
            pos.x = new_position.x;
            pos.y = new_position.y;
        }
    }
}