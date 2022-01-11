use crate::prelude::*;
use bracket_lib::prelude::Rect;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    walls: Vec<Rect>,
    rooms: Vec<Rect>,
    pub player_start: Position,
    pub enemies_start: Vec<Position>
}

impl MapBuilder {

    pub fn new() -> Self {

        let mut mb = MapBuilder{
            map : Map::new(),
            walls : Vec::new(),
            rooms : Vec::new(),
            player_start : Position{x:0, y:0, z:0},
            enemies_start : Vec::new(),
        };
        mb.fill(TileType::Void);
        mb.build_random_rooms();
        mb.build_corridors();
        // rooms are rect, as per bracketlib they return a point
        mb.player_start = Position::from(mb.rooms[0].center());
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
                    room.x1 - 1, room.y1 - 1, room.x2 + 1, room.y2 + 1
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
                // push the centers to enemies start, which is where they will be placed
                // except in room 0 where we place the player
                if self.rooms.len() > 1 {
                    self.enemies_start.push(Position::from(room.center()));
                }
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

    // places an entity in a given position in the occupation map
    pub fn entity_occupy_tile(&mut self, entity: Entity, position: Position) {
        let idx = map_idx(position.x, position.y);
        self.map.occupation[idx] = Some(entity);
    }

    // frees a given position in the occupation map
    pub fn free_occupy_tile(&mut self, position: Position) {
        let idx = map_idx(position.x, position.y);
        self.map.occupation[idx] = None;
    }

    // moves entity in occupation map, between position
    pub fn move_entity_occupation(&mut self, entity: Entity, old_p: Position, new_p: Position) {
        let old_idx = map_idx(old_p.x, old_p.y);
        let new_idx = map_idx(new_p.x, new_p.y);
        self.map.occupation[old_idx] = None;
        self.map.occupation[new_idx] = Some(entity);
    } 
}