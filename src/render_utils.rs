use crate::prelude::*;

pub fn size_scaling(primary_query: Query<&Window, With<PrimaryWindow>>, mut q: Query<(&TileSize, &mut Transform)>) {
    if let Ok(primary) = primary_query.single() {
        for (sprite_size, mut transform) in q.iter_mut() {
            let scale = Vec3::new(
                sprite_size.width / SCREEN_WIDTH as f32 * primary.width() as f32,
                sprite_size.height / SCREEN_HEIGHT as f32 * primary.height() as f32,
                1.0,
            );
            transform.scale = scale;
        }
    }
}

pub fn convert_pos(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
    let tile_size = bound_window / bound_game;
    pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
}

pub fn position_translation(primary_query: Query<&Window>, mut q: Query<(&Position, &mut Transform)>) {
    if let Ok(primary) = primary_query.single() {
        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(
                convert_pos(pos.x as f32, primary.width() as f32, SCREEN_WIDTH as f32),
                convert_pos((pos.y+UI_HEIGHT/2) as f32, primary.height() as f32, SCREEN_HEIGHT as f32),
                pos.z as f32,
            );
        }
    }
}