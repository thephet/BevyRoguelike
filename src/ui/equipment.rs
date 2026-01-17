use crate::prelude::*;
use super::popup;


fn update_equipment_text(
    highlighted_item: Res<popup::HighlightedItem>,
    player_query: Query<Entity, With<Player>>,
    text_query: Query<Entity, (With<popup::InventoryText>, Without<popup::DescriptionText>)>,
    description_query: Query<Entity, (With<popup::DescriptionText>, Without<popup::InventoryText>)>,
    items_query: Query<(Entity, &Naming, &Description, &Carried), With<Weapon>>,
    equipped_query: Query<(Entity, &Equipped)>,
    mut writer: TextUiWriter,
) {

    // get player entity, we will need it to filter out items carried by player
    let Ok(player_ent) = player_query.single() else {
        panic!("Can't get player entity");
    };

    // text list of items, and the description at the bottom
    let Ok(text) = text_query.single() else {
        panic!("Can't get text entity")
    };
    let Ok(description) = description_query.single() else {
        panic!("Can't get description entity")
    };

    if items_query.is_empty() {
        for i in 1..popup::INVENTORY_SLOTS as usize {
            *writer.text(text, i) = format!("\n ");

        }
        *writer.text(text, 0) = format!("No equipment.");
        *writer.text(description, 0) = format!(" ");


    } else {
        items_query.iter()
            .filter(|(_, _, _, carried)| carried.0 == player_ent)
            .enumerate()
            .filter(|(index, _)| *index < popup::INVENTORY_SLOTS as usize)
            .for_each(|(index, (item_entity, item, desc, _))|
            {
                // mark to signal where the user cursor is
                let mark;
                if index as i32 == highlighted_item.0 {
                    mark = "-";
                    *writer.text(description, 0) = format!("{}", desc.0);
                } else {
                    mark = " ";
                }

                // legend to say if the item is currently equipped
                let equipped = match equipped_query.get(item_entity) {
                    Ok(_) => "(e)",
                    Err(_) => "",
                };

                // update text
                if index == 0 {
                    *writer.text(text, index) = format!("{} {} {} {}", mark, item.0, equipped, mark);
                } else {
                    *writer.text(text, index) = format!("\n{} {} {} {}", mark, item.0, equipped, mark);

                }
            });
    }
}

fn equip_weapon(
    mut commands: Commands,
    mut highlighted_item: ResMut<popup::HighlightedItem>,
    mut chosen_item: MessageReader<popup::ChosenItemEvent>,
    player_query: Query<Entity, With<Player>>,
    items_query: Query<(Entity, &Carried), With<Weapon>>,
    equipped_query: Query<(Entity, &Equipped)>,
    mut turn_state: ResMut<NextState<TurnState>>,
    mut popup_state: ResMut<NextState<PopUpState>>,
) {
    // if user selected an item, then it will have a number over 0, otherwise -1
    let mut selected_item = -1;
    for se in chosen_item.read() {
        selected_item = se.0 as i32;
    }

    // get player entity, we will need it to filter out items carried by player
    let Ok(player_ent) = player_query.single()  else {
        panic!("Can't get player entity")
    };

    // get the item entity selected by the player
    let item_entity = items_query.iter()
    .filter(|(_, carried)| carried.0 == player_ent)
    .enumerate()
    .filter(|(item_count, (_,_))| *item_count as i32 == selected_item)
    .find_map(|(_, (item_entity, _))| Some(item_entity));


    // if the item exists, remove equipped from other weapons, and equip this one
    if let Some(item_entity) = item_entity 
    {
        // remove equipped component from every weapon equipped (should be only 1)
        equipped_query.iter()
            .for_each(|(equipped_weapon, _)| {
                commands.entity(equipped_weapon).remove::<Equipped>();
            });
        commands.entity(item_entity).insert(Equipped);
        
        // set also highlighted item to 0
        highlighted_item.0 = 0;

        // after using an item, move turn state and disable popup
        turn_state.set(TurnState::PlayerTurn);
        popup_state.set(PopUpState::None);
    }

}

pub struct EquipmentPlugin;
impl Plugin for EquipmentPlugin {
    fn build(&self, app: &mut App) {
        app

        // listening to user input on inventory screen
        .add_systems(
            Update,
            (equip_weapon, update_equipment_text)
            .run_if(in_state(PopUpState::EquipmentPopup))
        );

    }
}