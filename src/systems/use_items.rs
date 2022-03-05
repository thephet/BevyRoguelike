use crate::prelude::*;

pub fn use_items(
    mut commands: Commands,
    mut gamelog: ResMut<GameLog>,
    mut health_target_query: Query<(Entity, &mut Health)>,
    item_messages: Query<(Entity, &ActivateItem)>,
    healing_query: Query<(Entity, &ProvidesHealing)>,
    mapping_query: Query<(Entity, &ProvidesDungeonMap)>,
    mut maptiles_query: Query<(Entity, &mut Visibility), With<MapTile>>,
    names_query: Query<&Naming>,
) {

    // for every message
    for (message_entity, activated_item) in item_messages.iter() 
    {
        // if it is a healing item
        if let Ok((_, healing)) = healing_query.get(activated_item.item) 
        {
            if let Ok((_, mut health)) = health_target_query.get_mut(activated_item.used_by) 
            {
                // increase health
                health.current = i32::min( health.max, health.current+healing.amount);
                // update gamelog
                let target_char = names_query.get(activated_item.used_by).unwrap();
                let message = format!("\n{} heals {} HP.", target_char.0, healing.amount);
                gamelog.add_entry(message);
            }
        }

        // if it is a map item
        if let Ok(_) = mapping_query.get(activated_item.item) {
            // reveal all tiles
            maptiles_query.iter_mut().for_each(|(_, mut vis)| vis.is_visible = true);
            // update gamelog
            let message = format!("\nMap revealed.");
            gamelog.add_entry(message);
        }

        // delete the message
        commands.entity(message_entity).despawn();
        // remove the item
        commands.entity(activated_item.item).despawn_recursive();
    }

}