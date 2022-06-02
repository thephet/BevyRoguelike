use crate::prelude::*;

pub fn combat(
    mut commands: Commands,
    mut mb: ResMut<MapBuilder>,
    mut gamelog: ResMut<GameLog>,
    attacker_messages: Query<(Entity, &WantsToAttack)>,
    player: Query<Entity, With<Player>>,
    names_query: Query<&Naming>,
    mut health_query: Query<(&mut Health, &Position, &Naming)>,
    damage_query: Query<(&Damage, Option<&Carried>, Option<&Equipped>)>,
    
) {
    // get the list of victim messages
    let victims : Vec<(Entity, Entity, Entity)> = attacker_messages.iter()
        .map(|(entity, attack)| (entity, attack.attacker, attack.victim))
        .collect();

    // for every message, get the message itself, the attacker and the victim
    victims.iter().for_each(|(message, attacker, victim) | {
        // calculate damage of attack. total damage = base damage + weapon damage
        let base_damage = if let Ok((d, _, _)) = damage_query.get(*attacker) {
            d.0
        } else {
            0
        };

        let w_damage: i32 = damage_query.iter()
            .filter(|(_, c, e)| c.is_some() && e.is_some())
            .map(|(dmg, carried, _)| (dmg, carried.unwrap()))
            .filter(|(_, carried)| carried.0 == *attacker)
            .map(|(dmg, _)| dmg.0)
            .sum();
        
        let final_damage = base_damage + w_damage;

        // get the victim entity and decrease the hp
        if let Ok((mut hp, pos, name)) = health_query.get_mut(*victim) {
            hp.current -= final_damage;
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