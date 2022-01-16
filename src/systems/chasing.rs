use crate::prelude::*;

pub fn chasing(
    mut commands: Commands,
    mb: Res<MapBuilder>,
    movers: Query<(Entity, &Position, &FieldOfView), With<ChasingPlayer>>,
    positions: Query<(Entity, &Position), With<Health>>,
    player: Query<(Entity, &Position), With<Player>>,
) {
    let (_, player_pos) = player.single();
    // transform x,y position to index in array
    let player_idx = map_idx(player_pos.x, player_pos.y);
    // just get the map
    let map = &mb.map;

    // create dijkstra map around player
    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &search_targets,
        map,
        1024.0
    );

    movers.iter()
    .for_each(| (entity, pos, fov) | {
        // if monster cannot see player, then just return and do nothing
        if !fov.visible_tiles.contains( &((*player_pos).into()) ) {
            return;
        }
        //println!("yes");
        let idx = map_idx(pos.x, pos.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(
            &dijkstra_map, idx, &mb.map
        )
        {
            let distance = DistanceAlg::Pythagoras.distance2d(
                (*pos).into(), (*player_pos).into());
            let destination: Position = if distance > 1.2 {
                Position::from((map.index_to_point2d(destination), pos.z))
            } else {
                *player_pos
            };

            let mut attacked = false;
            positions
                .iter()
                .filter(|(_, target_pos)| **target_pos == destination.into())
                .for_each(|(victim, _)| {
                    // if the victim is the player
                    if let Ok( (player_victim, _) ) = player.get(victim) {
                        // send an attack message
                        commands.spawn().insert( WantsToAttack{attacker: entity, victim: player_victim});
                    }
                    attacked = true;
                });

                if !attacked {
                    // move to new position         
                    commands.spawn()
                        .insert( WantsToMove{entity: entity, destination: destination.into()});
                }
        }
    });

}