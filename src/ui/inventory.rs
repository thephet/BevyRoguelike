use crate::prelude::*;

#[derive(Component)]
struct InventoryUI;

#[derive(Component)]
struct InventoryText;

struct ChosenItemEvent(i32);

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
                    size: Size::new(Val::Percent(100.0), Val::Px(100. * 1.)),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                color: Color::rgb(0.0, 0.0, 0.5).into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(20. * 1.)),
                        margin: Rect {
                            left: Val::Auto,
                            right: Val::Auto,
                            top: Val::Auto,
                            bottom: Val::Auto,
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "You have no items".to_string(),
                        TextStyle {
                            font_size: 20.0,
                            font: font.clone(),
                            color: Color::WHITE,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(InventoryText);
            });
        });
    });
}

fn inventory_input(
    mut chosen_item: EventWriter<ChosenItemEvent>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut turn_state: ResMut<State<TurnState>>,
    mut player_query: Query<&mut Health, With<Player>>,
) {
    // current item selected by user, to send as event
    let current_item = 0;
    // get player max HP
    let mut health = player_query.single_mut();

    let key = keyboard_input.get_pressed().next().cloned();
    if let Some(key) = key 
    {
        match key {
            KeyCode::Escape => {
                // reset keyboard, bevys bug when changing states
                keyboard_input.reset(key);
                // update state
                turn_state.pop().unwrap();
            }
            KeyCode::Return => {
                chosen_item.send(ChosenItemEvent(current_item));
                health.current = i32::min(health.max, health.current+1);
                // update state
                turn_state.pop().unwrap();
            }
            _ => (),
        }
    }
}

fn update_inventory_text(
    mut commands: Commands, 
    mut chosen_item: EventReader<ChosenItemEvent>,
    mut text_query: Query<&mut Text, With<InventoryText>>,
    items_query: Query<(Entity, &Naming), With<Carried>>,
) {

    // if user selected an item, then it will have a number over 0, otherwise -1
    let mut selected_item = -1;
    for se in chosen_item.iter() {
        selected_item = se.0 as i32;
    }

    let mut text = text_query.single_mut();

    if items_query.is_empty() {
        text.sections[0].value = format!("You have no items.");
    }
    for (index, (entity, item)) in items_query.iter().enumerate() {
        // update text
        text.sections[index].value = format!("{}", item.0);
        if index as i32 == selected_item {
            commands.entity(entity).despawn_recursive();
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