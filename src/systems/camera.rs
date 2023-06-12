use crate::prelude::*;

// Move the camera when the player moves. See the Changed in the query
pub fn camera_move(
    primary_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Position, (Changed<Position>, With<Player>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {

    // if player position got updated
    for player_position in player_query.iter() {
        // get camera transform and window
        if let Ok(window) = primary_query.get_single() {
            let mut camera_transform = camera_query.single_mut();
            // calculate new coordinates and update
            let cam_x = convert_pos(player_position.x as f32, window.width() as f32, SCREEN_WIDTH as f32);
            let cam_y = convert_pos(player_position.y as f32, window.height() as f32, SCREEN_HEIGHT as f32);
            camera_transform.translation = Vec3::new(cam_x, cam_y, 999.0);
        }
    }

}