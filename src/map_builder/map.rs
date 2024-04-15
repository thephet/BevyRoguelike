use crate::prelude::*;

use bracket_lib::pathfinding::BaseMap;
use bracket_lib::pathfinding::SmallVec;


const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Exit,
    Void,
}

pub struct Map {
    // for tiles, like wall, floor,...
    pub tiles: Vec<TileType>,
    // entities occupying the tiles, like player, enemies, objects, ...
    pub occupation: Vec<Option<Entity>>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Void; NUM_TILES],
            occupation: vec![None; NUM_TILES],
        }
    }

    pub fn in_bounds<T: Into<Position>> (&self, position: T) -> bool {
        let position = position.into();
        position.x >= 0 && position.x < SCREEN_WIDTH
            && position.y >= 0 && position.y < SCREEN_HEIGHT
    }

    // checks if it is physically possible (ie no wall or physical object)
    pub fn can_enter_tile<T: Into<Position>> (&self, position: T) -> bool {
        let position = position.into();
        self.in_bounds(position) && (
            self.tiles[map_idx(position.x, position.y)] == TileType::Floor ||
            self.tiles[map_idx(position.x, position.y)] == TileType::Exit
        )
    }

    // checks if another entity like an enemy or player, are already in that cell
    pub fn is_tile_occupied<T: Into<Position>> (&self, position: T) -> bool {
        let position = position.into();
        self.in_bounds(position)
            && self.occupation[map_idx(position.x, position.y)] == None
    }

    pub fn try_idx(&self, position: Position) -> Option<usize> {
        if !self.in_bounds(position) {
            None
        } else {
            Some(map_idx(position.x, position.y))
        }
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }
    fn in_bounds(&self, item: Point) -> bool {
        self.in_bounds(item)
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> 
    {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }
        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(
            self.index_to_point2d(idx1), self.index_to_point2d(idx2)
        )
    }

    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] != TileType::Floor
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y*SCREEN_WIDTH) + x) as usize
}

pub fn spawn_map_tiles(
    mut commands: Commands,
    mb: Res<MapBuilder>,
    atlas: Res<CharsetAsset>,
) {

    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let idx = map_idx(x, y);
            let glyph = mb.theme.tile_to_render(mb.map.tiles[idx]);                

            if let Some(glyph) = glyph {
                match mb.map.tiles[idx] 
                {
                    TileType::Floor => 
                    {
                        commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                color: glyph.color,
                                custom_size: Some(Vec2::new(1.0, 1.0)),
                                ..Default::default()
                            },
                            visibility: Visibility::Hidden,
                            ..Default::default()
                        })
                        .insert(MapTile)
                        .insert(Position { x: x, y: y, z: 1 })
                        .insert(TileSize::square(1.0));
                    }
                    
                    tile_type @ (TileType::Wall | TileType::Exit) =>
                    {
                        // if exit, add entity with exit component, so it's easy to find later
                        if tile_type == TileType::Exit {
                            commands.spawn((Position { x, y, z: 1 }, ExitTile));
                        }
                        if let Some(bkg_color) = glyph.bkg_color {
                            commands
                            .spawn((
                                MapTile,
                                TileSize::square(1.0),
                                Position { x, y, z: 0 },
                                SpriteBundle {
                                sprite: Sprite {
                                    color: bkg_color,
                                    custom_size: Some(Vec2::new(1.0, 1.0)),
                                    ..Default::default()
                                },
                                visibility: Visibility::Hidden,
                                ..Default::default()
                            }));
                        }

                        commands
                            .spawn((
                                MapTile,
                                TileSize::square(1.0),
                                Position { x, y, z: 1 },
                                SpriteSheetBundle {
                                    sprite: Sprite {
                                        color: glyph.color,
                                        custom_size: Some(Vec2::new(1.0, 1.0)),
                                        
                                        ..Default::default()
                                    },
                                    atlas: TextureAtlas {
                                        layout: atlas.atlas.clone(),
                                        index: glyph.index,
                                    },
                                    texture: atlas.texture.clone(),

                                    visibility: Visibility::Hidden,
                                    ..Default::default()
                                }
                            ));
                    }
                    TileType::Void => ()
                }
            }
        }
    }
}