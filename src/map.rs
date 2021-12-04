use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y*SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }
}

pub fn render_map(
    mut map: Res<Map>,
    mut commands: Commands,
    atlas: Res<CharsetAsset>,
) {
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let idx = map_idx(x, y);
            
            let screen_x = x - SCREEN_WIDTH / 2;
            let screen_y = y - SCREEN_HEIGHT / 2;

            match map.tiles[idx] {
                TileType::Floor => {
                    commands
                    .spawn()
                    .insert_bundle(SpriteSheetBundle {
                        texture_atlas: atlas.atlas.clone(),
                        transform: Transform::from_translation(Vec3::new(screen_x as f32, screen_y as f32, 0.0)),
                        sprite: TextureAtlasSprite::new(46),
                        ..Default::default()
                    });
                }
                TileType::Wall => {
                    commands
                    .spawn()
                    .insert_bundle(SpriteSheetBundle {
                        texture_atlas: atlas.atlas.clone(),
                        transform: Transform::from_translation(Vec3::new(screen_x as f32, screen_y as f32, 0.0)),
                        sprite: TextureAtlasSprite::new(46),
                        ..Default::default()
                    });
                }
            }
        }
    }
}