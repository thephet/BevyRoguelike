use crate::prelude::*;

pub fn combat(
    mut commands: Commands,
    mut mb: ResMut<MapBuilder>,
    mut gamelog: ResMut<GameLog>,
    attacker_messages: Query<(Entity, &WantsToAttack)>,
    player: Query<Entity, With<Player>>,
    names_query: Query<&Naming>,
    mut health_query: Query<(&mut Health, &Position, &Naming)>,
) {

    // get the list of victim messages
    let victims : Vec<(Entity, Entity, Entity)> = attacker_messages.iter()
        .map(|(entity, attack)| (entity, attack.attacker, attack.victim))
        .collect();

    // for every message, get the message itself and the victim
    victims.iter().for_each(|(message, attacker, victim) | {
        // get the victim entity and decrease the hp
        if let Ok((mut hp, pos, name)) = health_query.get_mut(*victim) {
            hp.current -= 1;
            // add action to gamelog, first get name of attacker, then build message
            let attacker_char = names_query.get(*attacker).unwrap();
            let message = format!("\n{} attacks {}.", attacker_char.0, name.0);
            gamelog.add_entry(message);

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