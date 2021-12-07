use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub walls: Vec<Rect>,
    pub rooms: Vec<Rect>,
    pub player_start: Position
}

impl MapBuilder {

    pub fn new() -> Self {

        let mut mb = MapBuilder{
            map : Map::new(),
            walls : Vec::new(),
            rooms : Vec::new(),
            player_start : Position{x:0, y:0, z:0},
        };
        mb.fill(TileType::Void);
        mb.build_random_rooms();
        mb.build_corridors();
        mb.player_start = mb.rooms[0].center();
        mb
    }

    fn fill(&mut self, tile:TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self) {
        let mut rng = rand::thread_rng();

        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.gen_range(2..SCREEN_WIDTH - 12),
                rng.gen_range(2..SCREEN_HEIGHT - 12),
                rng.gen_range(2..12),
                rng.gen_range(2..12),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {
                let wall = Rect::with_exact(
                    room.x1 - 1, room.y1 - 1, room.x2 +1, room.y2 + 1
                );
                // First make the floor space that will be the room
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 
                        && p.y < SCREEN_HEIGHT 
                    {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                // now place the walls around it 
                wall.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 
                        && p.y < SCREEN_HEIGHT 
                    {
                        let idx = map_idx(p.x, p.y);
                        if self.map.tiles[idx] == TileType::Void {
                            self.map.tiles[idx] = TileType::Wall;
                        }
                    }
                });
                self.rooms.push(room);
                self.walls.push(wall);
            }
        }
    }

    fn apply_horizontal_tunnel_walls(&mut self, x1:i32, x2:i32, y:i32) {
        use std::cmp::{min, max};
        for x in min(x1,x2) ..= max(x1,x2) {
            if let Some(idx) = self.map.try_idx(Position{x, y, z:0}) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
            if let Some(idx) = self.map.try_idx(Position{x, y:y-1, z:0}) {
                if self.map.tiles[idx as usize] == TileType::Void{ 
                    self.map.tiles[idx as usize] = TileType::Wall;
                }
            }
            if let Some(idx) = self.map.try_idx(Position{x, y:y+1, z:0}) {
                if self.map.tiles[idx as usize] == TileType::Void{ 
                    self.map.tiles[idx as usize] = TileType::Wall;
                }
            }
        }
    }

    fn apply_vertical_tunnel_walls(&mut self, y1:i32, y2:i32, x:i32) {
        use std::cmp::{min, max};
        for y in min(y1,y2) ..= max(y1,y2) {
            if let Some(idx) = self.map.try_idx(Position{x, y, z:0}) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
            if let Some(idx) = self.map.try_idx(Position{x:x-1, y, z:0}) {
                if self.map.tiles[idx as usize] == TileType::Void{ 
                    self.map.tiles[idx as usize] = TileType::Wall;
                }
            }
            if let Some(idx) = self.map.try_idx(Position{x:x+1, y, z:0}) {
                if self.map.tiles[idx as usize] == TileType::Void{ 
                    self.map.tiles[idx as usize] = TileType::Wall;
                }
            }
        }
    }

    fn build_corridors(&mut self) {
        let mut rng = rand::thread_rng();
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a,b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i-1].center();
            let new = room.center();

            if rng.gen_range(0..2) == 1 {
                self.apply_horizontal_tunnel_walls(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel_walls(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel_walls(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel_walls(prev.x, new.x, new.y);
            }
        }
    }
}

// from bracket-lib
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Rect {
    // Create a new rectangle, specifying X/Y Width/Height
    pub fn with_size<T>(x: T, y: T, w: T, h: T) -> Rect
    where
        T: TryInto<i32>,
    {
        let x_i32: i32 = x.try_into().ok().unwrap();
        let y_i32: i32 = y.try_into().ok().unwrap();
        Rect {
            x1: x_i32,
            y1: y_i32,
            x2: x_i32 + w.try_into().ok().unwrap(),
            y2: y_i32 + h.try_into().ok().unwrap(),
        }
    }
    // Create a new rectangle, specifying exact dimensions
    pub fn with_exact<T>(x1: T, y1: T, x2: T, y2: T) -> Rect
    where
        T: TryInto<i32>,
    {
        Rect {
            x1: x1.try_into().ok().unwrap(),
            y1: y1.try_into().ok().unwrap(),
            x2: x2.try_into().ok().unwrap(),
            y2: y2.try_into().ok().unwrap(),
        }
    }
    // Returns true if this overlaps with other,
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    // Returns the center of the rectangle
    pub fn center(&self) -> Position {
        Position{x: (self.x1 + self.x2) / 2, y: (self.y1 + self.y2) / 2, z: 0}
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(Position),
    {
        for y in self.y1..self.y2 {
            for x in self.x1..self.x2 {
                f(Position{x, y, z:0});
            }
        }
    }
}