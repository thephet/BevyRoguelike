use crate::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    atlas: Res<CharsetAsset>,
    mb: Res<MapBuilder>,
) {

    commands
    .spawn_bundle(SpriteSheetBundle {
        texture_atlas: atlas.atlas.clone(),
        sprite: TextureAtlasSprite::new(64),
        ..Default::default()
    })
    .insert(Position { x: mb.player_start.x, y: mb.player_start.y, z: 1 })
    .insert(TileSize::square(1.0))
    .insert(Player);
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_positions: Query<&mut Position, With<Player>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
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

            let mut camera_transform = camera_query.single_mut().unwrap();
            let cam_x = convert_pos(pos.x as f32, 800.0, SCREEN_WIDTH as f32);
            let cam_y = convert_pos(pos.y as f32, 500.0, SCREEN_HEIGHT as f32);
            camera_transform.translation = Vec3::new(cam_x, cam_y, 999.0);

        }
    }
}