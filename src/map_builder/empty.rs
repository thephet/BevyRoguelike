use crate::prelude::*;
use super::MapArchitect;

pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
    fn new(&mut self) -> MapBuilder {
        let mut mb = MapBuilder{
            map : Map::new(),
            rooms: Vec::new(),
            walls: Vec::new(),
            player_start : Position::new(0, 0, 0),
            enemies_start : Vec::new(),
            amulet_start : Position::new(0, 0, 0)
        };
        mb.fill(TileType::Floor);
        mb.player_start = Position::new_from2d(SCREEN_WIDTH/2, SCREEN_HEIGHT/2);
        mb.amulet_start = mb.find_most_distant();
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            mb.enemies_start.push(
                Position::new_from2d(
                    rng.gen_range(1.. SCREEN_WIDTH),
                    rng.gen_range(1.. SCREEN_WIDTH)
                )
            )
        }
        mb
    }
}