use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Void,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        let tiles = vec![TileType::Floor; NUM_TILES];
        Self {
            tiles: tiles,
        }
    }

    pub fn in_bounds(&self, point: Position) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH
            && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Position) -> bool {
        self.in_bounds(point)
            && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    pub fn try_idx(&self, point: Position) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y*SCREEN_WIDTH) + x) as usize
}

pub fn spawn_map_tiles(
    mut commands: Commands,
    mb: Res<MapBuilder>,
    atlas: Res<CharsetAsset>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let idx = map_idx(x, y);

            match mb.map.tiles[idx] {
                TileType::Floor => {
                    commands
                    .spawn_bundle(SpriteBundle {
                        material: color_materials.add(Color::rgba(0.3, 0.3, 0.3, 0.7).into()),
                        sprite: Sprite::new(Vec2::new(1.0, 1.0)),
                        ..Default::default()
                    })
                    .insert(Position { x: x, y: y, z: 1 })
                    .insert(TileSize::square(1.0));

                    // commands           
                    // .spawn_bundle(SpriteSheetBundle {
                    //     texture_atlas: atlas.atlas.clone(),
                    //     sprite: TextureAtlasSprite::new('.' as u32),
                    //     ..Default::default()
                    // })
                    // .insert(Position { x: x, y: y, z: 0 })
                    // .insert(TileSize::square(1.0));
                }
                TileType::Wall => {
                    commands
                    .spawn_bundle(SpriteBundle {
                        material: color_materials.add(Color::rgba(0.3, 0.3, 0.3, 0.2).into()),
                        sprite: Sprite::new(Vec2::new(1.0, 1.0)),
                        ..Default::default()
                    })
                    .insert(Position { x: x, y: y, z: 1 })
                    .insert(TileSize::square(1.0));
                    
                    commands
                        .spawn_bundle(SpriteSheetBundle {
                            texture_atlas: atlas.atlas.clone(),
                            sprite: TextureAtlasSprite::new('#' as u32),
                            ..Default::default()
                        })
                        .insert(Position { x: x, y: y, z: 0 })
                        .insert(TileSize::square(1.0));
                }
                TileType::Void => ()
            }
        }
    }
}