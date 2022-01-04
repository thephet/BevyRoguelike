use crate::prelude::*;

pub fn combat(
    mut commands: Commands,
    attackers: Query<(Entity, &WantsToAttack)>,
    mut health_query: Query<(&mut Health, &mut Alive)>,
) {

    // get the list of victim messages
    let victims : Vec<(Entity, Entity)> = attackers.iter()
        .map(|(entity, attack)| (entity, attack.victim))
        .collect();

    // for every message, get the message itself and the victim
    victims.iter().for_each(|(message, victim) | {
        // get the victim entity and decrease the hp
        if let Ok((mut hp, mut alive)) = health_query.get_mut(*victim) {
            hp.current -= 1;

            // less than 1 HP remove it
            if hp.current < 1 {
                alive.0 = false;
                commands.entity(*victim).despawn();
            }
        }
        // remove the message
        commands.entity(*message).despawn();
    });

}