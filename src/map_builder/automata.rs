use crate::prelude::*;
use super::MapArchitect;

pub struct CellularAutomataArchitect {}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self) -> MapBuilder {
        let mut mb = MapBuilder{
            map : Map::new(),
            rooms: Vec::new(),
            walls: Vec::new(),
            player_start : Position::new(0, 0, 0),
            enemies_start : Vec::new(),
            amulet_start : Position::new(0, 0, 0),
            theme: super::themes::ForestTheme::new()
        };

        self.random_noise_map(&mut mb.map);
        for _ in 0..10 {
            self.iteration(&mut mb);
        }

        mb.wall_around_boundary();
        mb.clean_walls_replace_with_void();
        let start = self.find_start(&mb.map);
        mb.enemies_start = mb.spawn_monsters(&start);
        mb.player_start = start;
        mb.amulet_start = mb.find_most_distant();
        mb

    }
}

impl CellularAutomataArchitect {

    fn random_noise_map(&mut self, map: &mut Map)
    {
        let mut rng = rand::thread_rng();

        map.tiles.iter_mut().for_each(|t| {
            let roll = rng.gen_range(0..100);
            if roll > 55 {
                *t = TileType::Floor;
            } else {
                *t = TileType::Wall;
            }
        });
    }

    fn iteration(&mut self, mb: &mut MapBuilder) 
    {
        let mut new_tiles = mb.map.tiles.clone();
        for y in 1 .. SCREEN_HEIGHT -1 {
            for x in 1 .. SCREEN_WIDTH -1 {
                let neighbors = mb.count_neighbors(x, y, &(mb.map));
                let idx = map_idx(x, y);
                if neighbors > 4 || neighbors == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }
        mb.map.tiles = new_tiles;
    }

    fn find_start(&self, map: &Map) -> Position 
    {
        let center = Point::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2);
        let closest_point = map.tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| (idx, DistanceAlg::Pythagoras.distance2d(
                center,
                map.index_to_point2d(idx)
            )))
            .min_by(|(_, distance), (_, distance2)| 
                distance.partial_cmp(&distance2).unwrap()
            )
            .map(|(idx, _)| idx)
            .unwrap();
        map.index_to_point2d(closest_point).into()
    }

}