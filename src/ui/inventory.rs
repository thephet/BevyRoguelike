use crate::prelude::*;
use super::popup;


fn update_inventory_text(
    highlighted_item: Res<popup::HighlightedItem>,
    player_query: Query<Entity, With<Player>>,
    text_query: Query<Entity, (With<popup::InventoryText>, Without<popup::DescriptionText>)>,
    description_query: Query<Entity, (With<popup::DescriptionText>, Without<popup::InventoryText>)>,
    items_query: Query<(Entity, &Naming, &Description, &Carried), Without<Weapon>>,
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
        *writer.text(text, 0) = format!("You have no items.");
        *writer.text(description, 0) = format!(" ");

    } else {
        items_query.iter()
            .filter(|(_, _, _, carried)| carried.0 == player_ent)
            .enumerate()
            .filter(|(index, _)| *index < popup::INVENTORY_SLOTS as usize)
            .for_each(|(index, (_, item, desc, _))|
            {
                let mark;
                if index as i32 == highlighted_item.0 {
                    mark = "-";
                    *writer.text(description, 0) = format!("{}", desc.0);

                } else {
                    mark = " ";
                }
                // update text
                if index == 0 {
                    *writer.text(text, index) = format!("{} {} {}", mark, item.0, mark);
                } else {
                    *writer.text(text, index) = format!("\n{} {} {}", mark, item.0, mark);

                }
            });
    }
}

fn use_item(
    mut commands: Commands,
    mut highlighted_item: ResMut<popup::HighlightedItem>,
    mut chosen_item: EventReader<popup::ChosenItemEvent>,
    player_query: Query<Entity, With<Player>>,
    items_query: Query<(Entity, &Carried)>,
    mut turn_state: ResMut<NextState<TurnState>>,
    mut popup_state: ResMut<NextState<PopUpState>>,
) {
    // if user selected an item, then it will have a number over 0, otherwise -1
    let mut selected_item = -1;
    for se in chosen_item.read() {
        selected_item = se.0 as i32;
    }

    // get player entity, we will need it to filter out items carried by player
    let Ok(player_ent) = player_query.single() else {
        panic!("Can't find player entity")
    };

    // get the item entity selected by the player
    let item_entity = items_query.iter()
            .filter(|(_, carried)| carried.0 == player_ent)
            .enumerate()
            .filter(|(item_count, (_,_))| *item_count as i32 == selected_item)
            .find_map(|(_, (item_entity, _))| Some(item_entity));

    // if the item exists, send a message to activate it
    if let Some(item_entity) = item_entity 
    {
        commands.spawn(ActivateItem{used_by: player_ent, item: item_entity});
        // set also highlighted item to 0, since previous item wont exist on list
        highlighted_item.0 = 0;

        // after using an item, move turn state and disable popup
        turn_state.set(TurnState::PlayerTurn);
        popup_state.set(PopUpState::None);
    }

}

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app

        // listening to user input on inventory screen
        .add_systems(
            Update,
            (use_item, update_inventory_text)
            .run_if(in_state(PopUpState::InventoryPopup))
        );

    }
}