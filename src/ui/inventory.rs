use crate::prelude::*;

#[derive(Component)]
struct InventoryUI;

#[derive(Component)]
struct InventoryText;

struct ChosenItemEvent(i32);
struct HighlightedItemEvent(i32);

fn inventory_popup(
    mut commands: Commands,
    font: Res<Handle<Font>>,
) {

    commands
    .spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(50.), Val::Percent(50.)),
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Percent(25.0),
                bottom: Val::Percent(30.0),
                ..Default::default()
            },
            border: Rect::all(Val::Px(5.0)),
            ..Default::default()
        },
        color: UiColor(Color::rgb(0.65, 0.65, 0.65)),
        ..Default::default()
    })
    .insert(InventoryUI)
    
    // now inner rectangle
    .with_children(|parent| {
        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: UiColor(Color::rgb(0.0, 0.0, 0.0)),
            ..Default::default()
        })

        // now the different text inside box
        .with_children(|parent| {
            // invetory title
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(100. * 1.)),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                color: Color::rgb(0.5, 0.0, 0.0).into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(50. * 1.)),
                        margin: Rect {
                            left: Val::Auto,
                            right: Val::Auto,
                            top: Val::Auto,
                            bottom: Val::Auto,
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Inventory".to_string(),
                        TextStyle {
                            font_size: 50.0,
                            font: font.clone(),
                            color: Color::GOLD,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });
            });
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(20. * 6.)),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                color: Color::rgb(0.0, 0.0, 0.5).into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(20. * 6.)),
                        margin: Rect {
                            left: Val::Auto,
                            right: Val::Auto,
                            top: Val::Auto,
                            bottom: Val::Auto,
                        },
                        ..Default::default()
                    },
                    text: Text {
                        // Construct a `Vec` of `TextSection`s
                        sections: vec![
                            TextSection {
                                value: "You have no items.".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "\n ".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "\n ".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "\n ".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "\n ".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "\n ".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            },
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(InventoryText);
            });
        });
    });
}

fn inventory_input(
    mut chosen_item: EventWriter<ChosenItemEvent>,
    mut highlighted_item: EventWriter<HighlightedItemEvent>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut turn_state: ResMut<State<TurnState>>,
    mut player_query: Query<&mut Health, With<Player>>,
    text_query: Query<With<InventoryText>>,
) {
    // current item selected by user, to send as event
    let mut current_item = 0;
    // get player max HP
    let mut health = player_query.single_mut();
    // get the number of slots for items
    let item_slots = text_query.iter().count();

    let key = keyboard_input.get_pressed().next().cloned();
    if let Some(key) = key 
    {
        match key {
            KeyCode::Escape => { // close inventory window
                // reset keyboard, bevys bug when changing states
                keyboard_input.reset(key);
                // update state
                turn_state.pop().unwrap();
            }
            KeyCode::Return => { // activate selected item and close inventory window
                chosen_item.send(ChosenItemEvent(current_item));
                health.current = i32::min(health.max, health.current+1);
                // update state
                turn_state.pop().unwrap();
            }
            KeyCode::Up => { // move to previous item in list
                current_item = i32::max(0, current_item-1);
                highlighted_item.send(HighlightedItemEvent(current_item));
            }
            KeyCode::Down => { // move to next item in list
                current_item = i32::min(item_slots as i32, current_item+1);
                highlighted_item.send(HighlightedItemEvent(current_item));
            }
            _ => (),
        }
    }
}

fn update_inventory_text(
    mut commands: Commands, 
    mut chosen_item: EventReader<ChosenItemEvent>,
    mut highlighted_item: EventReader<HighlightedItemEvent>,
    mut text_query: Query<&mut Text, With<InventoryText>>,
    items_query: Query<(Entity, &Naming), With<Carried>>,
) {

    // if user selected an item, then it will have a number over 0, otherwise -1
    let mut selected_item = -1;
    for se in chosen_item.iter() {
        selected_item = se.0 as i32;
    }

    // there will be always a highlighted item, default it will be 0
    let mut high_item = 0;
    for hi in highlighted_item.iter() {
        high_item = hi.0 as i32;
    }

    let mut text = text_query.single_mut();
    let mut mark = "";

    if items_query.is_empty() {
        text.sections[0].value = format!("You have no items.");
    } else {
        for (index, (entity, item)) in items_query.iter().enumerate() {
            if index as i32 == high_item {
                mark = "-";
            } else {
                mark = "";
            }
            // update text
            if index == 0 {
                text.sections[index].value = format!("{} {} {}", mark, item.0, mark);
            } else {
                text.sections[index].value = format!("\n{} {} {}", mark, item.0, mark);
            }
            
            if index as i32 == selected_item {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

// function to kill either start screen or game over screen
fn despawn_menu(
    mut commands: Commands, 
    query_inventory: Query<Entity, With<InventoryUI>>,
) {
    for e in query_inventory.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app

        .add_event::<ChosenItemEvent>()
        .add_event::<HighlightedItemEvent>()

        // listening to user input on inventory screen
        .add_system_set(
            SystemSet::on_update(TurnState::InventoryPopup)
                .with_system(inventory_input.label("inventory_input"))
                .with_system(update_inventory_text.after("inventory_input"))
        )

        // cleanup when exiting
        .add_system_set(
            SystemSet::on_exit(TurnState::InventoryPopup)
                .with_system(despawn_menu)
        )
        
        .add_system_set(
            SystemSet::on_enter(TurnState::InventoryPopup)
                .with_system(inventory_popup)
        );

    }
}