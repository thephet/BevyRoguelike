use crate::prelude::*;
use super::MapArchitect;

pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
    fn new(&mut self) -> MapBuilder {
        let mut mb = MapBuilder{
            map : Map::new(),
            rooms: Vec::new(),
            walls: Vec::new(),
            player_start : Position::new(0, 0, 0),
            enemies_start : Vec::new(),
            amulet_start : Position::new(0, 0, 0),
            theme: super::themes::DungeonTheme::new()
        };

        mb.fill(TileType::Void);
        mb.build_random_rooms();
        mb.build_corridors();
        mb.player_start = Position::from(mb.rooms[0].center());
        mb.amulet_start = mb.find_most_distant();
        for room in mb.rooms.iter().skip(1) {
            mb.enemies_start.push(room.center().into());
        }

        mb
    }
}

