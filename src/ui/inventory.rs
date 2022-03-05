use crate::prelude::*;

#[derive(Component)]
struct InventoryUI;

#[derive(Component)]
struct InventoryText;

#[derive(Component)]
struct DescriptionText;

struct ChosenItemEvent(i32);
struct HighlightedItem(i32);

const INVENTORY_SLOTS: i32 = 10;

fn inventory_popup(
    mut commands: Commands,
    font: Res<Handle<Font>>,
) {

    // background color for the inventory window
    let bkg_color = UiColor(Color::rgb(0.15, 0.15, 0.15));

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
            color: bkg_color,
            ..Default::default()
        })

        // now the different text inside box
        .with_children(|parent| 
        {
            // inventory title
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(100. * 1.)),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                color: bkg_color,
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
                    size: Size::new(Val::Percent(100.0), Val::Px(20. * INVENTORY_SLOTS as f32)),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                color: bkg_color,
                ..Default::default()
            })
            .with_children(|parent| {
                // create vector with text sections
                let mut sections = Vec::new();
                for _ in 0..INVENTORY_SLOTS {
                    sections.push(TextSection {
                        value: "\n ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    });
                }
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(20. * (INVENTORY_SLOTS+1) as f32)),
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
                        sections: sections,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(InventoryText);
            });

            // hint section
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Auto),
                    flex_direction: FlexDirection::ColumnReverse,
                    margin: Rect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Auto,
                        bottom: Val::Auto,
                    },
                    ..Default::default()
                },
                color: bkg_color,
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(20.)),
                        margin: Rect {
                            left: Val::Auto,
                            right: Val::Auto,
                            top: Val::Auto,
                            bottom: Val::Auto,
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        " ".to_string(),
                        TextStyle {
                            font_size: 20.0,
                            font: font.clone(),
                            color: Color::WHITE,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(DescriptionText);
            });

        });
    });
}

fn inventory_input(
    mut chosen_item: EventWriter<ChosenItemEvent>,
    mut highlighted_item: ResMut<HighlightedItem>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut turn_state: ResMut<State<TurnState>>,
) {

    let key = keyboard_input.get_pressed().next().cloned();
    if let Some(key) = key 
    {
        match key {
            KeyCode::Escape => { // close inventory window
                // update state
                turn_state.pop().unwrap();
            }
            KeyCode::Return => { // activate selected item and close inventory window
                chosen_item.send(ChosenItemEvent(highlighted_item.0));
                // update state
                turn_state.pop().unwrap();
            }
            KeyCode::Up => { // move to previous item in list
                highlighted_item.0 = i32::max(0, highlighted_item.0-1);
            }
            KeyCode::Down => { // move to next item in list
                highlighted_item.0 = i32::min(INVENTORY_SLOTS as i32, highlighted_item.0+1);
            }
            _ => (),
        }
        keyboard_input.reset(key);
    }
}

fn update_inventory_text(
    mut commands: Commands,
    mut chosen_item: EventReader<ChosenItemEvent>,
    mut highlighted_item: ResMut<HighlightedItem>,
    player_query: Query<Entity, With<Player>>,
    mut text_query: Query<&mut Text, (With<InventoryText>, Without<DescriptionText>)>,
    mut description_query: Query<&mut Text, (With<DescriptionText>, Without<InventoryText>)>,
    items_query: Query<(Entity, &Naming, &Description, &Carried)>,
) {

    // get player entity, we will need it to filter out items carried by player
    let player_ent = player_query.single();
    // if user selected an item, then it will have a number over 0, otherwise -1
    let mut selected_item = -1;
    for se in chosen_item.iter() {
        selected_item = se.0 as i32;
    }

    let mut text = text_query.single_mut();
    let mut description = description_query.single_mut();

    if items_query.is_empty() {
        for i in 1..INVENTORY_SLOTS as usize {
            text.sections[i].value = format!("\n ");
        }
        text.sections[0].value = format!("You have no items.");
        description.sections[0].value = format!(" ");

    } else {
        items_query.iter()
            .filter(|(_, _, _, carried)| carried.0 == player_ent)
            .enumerate()
            .for_each(|(index, (entity, item, desc, _))| 
            {
                let mut mark;
                if index as i32 == highlighted_item.0 {
                    mark = "-";
                    description.sections[0].value = format!("{}", desc.0);
                } else {
                    mark = " ";
                }
                // update text
                if index == 0 {
                    text.sections[index].value = format!("{} {} {}", mark, item.0, mark);
                } else {
                    text.sections[index].value = format!("\n{} {} {}", mark, item.0, mark);
                }
                
                if index as i32 == selected_item {
                    highlighted_item.0 = 0;
                    commands.entity(entity).despawn_recursive();
                }
            });

        // for (index, (entity, item, desc)) 
        //     in items_query.iter().enumerate() 
        // {
        //     if index as i32 == highlighted_item.0 {
        //         mark = "-";
        //         description.sections[0].value = format!("{}", desc.0);
        //     } else {
        //         mark = " ";
        //     }
        //     // update text
        //     if index == 0 {
        //         text.sections[index].value = format!("{} {} {}", mark, item.0, mark);
        //     } else {
        //         text.sections[index].value = format!("\n{} {} {}", mark, item.0, mark);
        //     }
            
        //     if index as i32 == selected_item {
        //         highlighted_item.0 = 0;
        //         commands.entity(entity).despawn_recursive();
        //     }
        // }
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
        .insert_resource(HighlightedItem(0))

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