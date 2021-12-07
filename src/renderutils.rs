use crate::prelude::*;

pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&TileSize, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        let scale = Vec3::new(
            sprite_size.width / SCREEN_WIDTH as f32 * window.width() as f32,
            sprite_size.height / SCREEN_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
        transform.scale =  scale;
    }
}

pub fn convert_pos(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
    let tile_size = bound_window / bound_game;
    pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
}

pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {

    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert_pos(pos.x as f32, window.width() as f32, SCREEN_WIDTH as f32),
            convert_pos(pos.y as f32, window.height() as f32, SCREEN_HEIGHT as f32),
            pos.z as f32,
        );
    }
}