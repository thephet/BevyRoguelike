use crate::prelude::*;
use super::MapArchitect;

const STAGGER_DISTANCE: usize = 400;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

pub struct DrunkardsWalkArchitect {}

pub trait DrunkardWalk {
    fn drunkard(&mut self, start: &Position, map: &mut Map)
    {
        let mut drunkard_pos = start.clone();
        let mut distance_staggered = 0;
        let mut rng = rand::thread_rng();

        loop {
            let drunk_idx = map.point2d_to_index(drunkard_pos.into());
            map.tiles[drunk_idx] = TileType::Floor;

            match rng.gen_range(0..4) {
                0 => drunkard_pos.x -= 1,
                1 => drunkard_pos.x += 1,
                2 => drunkard_pos.y -= 1,
                _ => drunkard_pos.y += 1,
            }
            if !map.in_bounds(drunkard_pos) {
                break;
            }
 
            distance_staggered += 1;
            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }

    fn drunkard_loop(&mut self, center: &Position, map: &mut Map, floor_vs_wall_ratio: f32)
    {
        let mut rng = rand::thread_rng();
        let desired_floor = NUM_TILES as f32 / floor_vs_wall_ratio;

        while map.tiles.iter()
        .filter(|t| **t == TileType::Floor).count() < desired_floor as usize
        {
            self.drunkard(
                &Position::new_from2d(
                    rng.gen_range(0..SCREEN_WIDTH),
                    rng.gen_range(0..SCREEN_HEIGHT)
                ),
                map
            );
            let dijkstra_map = DijkstraMap::new(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &vec![map.point2d_to_index((*center).into())],
                map,
                1024.0
            );
            dijkstra_map.map
                .iter()
                .enumerate()
                .filter(|(_, distance)| *distance > &2000.0)
                .for_each(|(idx, _)| map.tiles[idx] = TileType::Wall);
        }
    }
}

impl DrunkardWalk for DrunkardsWalkArchitect {}

impl MapArchitect for DrunkardsWalkArchitect {
    fn new(&mut self) -> MapBuilder {
        let mut mb = MapBuilder{
            map : Map::new(),
            rooms: Vec::new(),
            walls: Vec::new(),
            player_start : Position::new(0, 0, 0),
            enemies_start : Vec::new(),
            amulet_start : Position::new(0, 0, 0),
            theme: super::themes::CaveTheme::new()
        };

        mb.fill(TileType::Wall);
        let center = Position::new_from2d(SCREEN_WIDTH /2, SCREEN_HEIGHT/2);
        self.drunkard(&center, &mut  mb.map);
        self.drunkard_loop(&center, &mut mb.map, 4.0);

        mb.clean_walls_replace_with_void();
        mb.enemies_start = mb.spawn_monsters(&center);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();
        mb
    }
}
