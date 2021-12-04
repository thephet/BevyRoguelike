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

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y*SCREEN_WIDTH) + x) as usize
}

pub fn spawn_map_tiles(
    map: Res<Map>,
    mut commands: Commands,
    atlas: Res<CharsetAsset>,
) {
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let idx = map_idx(x, y);

            match map.tiles[idx] {
                TileType::Floor => {
                    commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: atlas.atlas.clone(),
                        sprite: TextureAtlasSprite::new(46),
                        ..Default::default()
                    })
                    .insert(Position { x: x, y: y })
                    .insert(TileSize::square(1.0));
                }
                TileType::Wall => ()
            }
        }
    }
}