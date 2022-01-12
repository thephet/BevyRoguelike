use crate::prelude::*;

pub fn combat(
    mut commands: Commands,
    mut mb: ResMut<MapBuilder>,
    attackers: Query<(Entity, &WantsToAttack)>,
    player: Query<Entity, With<Player>>,
    mut health_query: Query<(&mut Health, &Position)>,
) {

    // get the list of victim messages
    let victims : Vec<(Entity, Entity)> = attackers.iter()
        .map(|(entity, attack)| (entity, attack.victim))
        .collect();

    // for every message, get the message itself and the victim
    victims.iter().for_each(|(message, victim) | {
        // get the victim entity and decrease the hp
        if let Ok((mut hp, pos)) = health_query.get_mut(*victim) {
            hp.current -= 1;

            // less than 1 HP remove it
            if hp.current < 1 {
                if let Ok(_) = player.get(*victim) {
                    
                } else {
                    mb.free_occupy_tile(*pos);
                    commands.entity(*victim).despawn();
                }
            }
        }
        // remove the message
        commands.entity(*message).despawn();
    });

}