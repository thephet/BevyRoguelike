use crate::prelude::*;

// gets the name of the entity (with Naming component) nearer to the user click
pub fn cursor_click(
    // need to get window dimensions
    wnds: Res<Windows>,
    // to get the mouse clicks
    buttons: Res<Input<MouseButton>>,
    // query to get camera transform
    q_camera: Query<&Transform, With<MainCamera>>,
    // query to get all the entities with Name component
    q_names: Query<(&Naming, &Position)>
) {
    // if the user left clicks
    if buttons.just_pressed(MouseButton::Left) {
        // get the primary window
        let wnd = wnds.get_primary().unwrap();

        // check if the cursor is in the primary window
        if let Some(pos) = wnd.cursor_position() {
            // get the size of the window
            let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            // the default orthographic projection is in pixels from the center;
            // just undo the translation
            let p = pos - size / 2.0;

            // assuming there is exactly one main camera entity, so this is OK
            let camera_transform = q_camera.single().unwrap();

            let tile_size_x = wnd.width() / SCREEN_WIDTH as f32;
            let tile_size_y = wnd.height() / SCREEN_HEIGHT as f32;

            // apply the camera transform
            let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

            // transform world coordinates to our grid
            let grid_x = (pos_wld.x / tile_size_x) + (SCREEN_WIDTH / 2) as f32;
            let grid_y = (pos_wld.y / tile_size_y) + (SCREEN_HEIGHT / 2) as f32;

            // now we go through all the entities with name to see which one is the nearest
            let mut distance = 9999.9;
            let mut s = String::new();
            for (name, pos) in q_names.iter() {
                // calculate eucledian distance from click to all entities with Naming
                let mut dist = (grid_x - pos.x as f32).powi(2) + (grid_y - pos.y as f32).powi(2);
                dist = dist.sqrt();
                
                if dist < distance {
                    distance = dist;
                    s = name.0.clone();
                }
            }
            println!("{}", s);
        }
    }
}