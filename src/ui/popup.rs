use crate::prelude::*;
use super::inventory;
use super::equipment;

#[derive(Component)]
pub struct InventoryUI;

#[derive(Component)]
pub struct InventoryText;

#[derive(Component)]
pub struct DescriptionText;

pub struct ChosenItemEvent(pub i32);
#[derive(Resource)]
pub struct HighlightedItem(pub i32);

pub const INVENTORY_SLOTS: i32 = 10;

fn popup_ui(
    mut commands: Commands,
    font_manager: Res<FontManager>,
    turn_state: Res<State<TurnState>>,
) {
    // background color for the inventory window
    let bkg_color = BackgroundColor(Color::rgb(0.15, 0.15, 0.15));

    commands
    .spawn((NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(50.), Val::Percent(50.)),
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Percent(25.0),
                bottom: Val::Percent(30.0),
                ..Default::default()
            },
            border: UiRect::all(Val::Px(5.0)),
            ..Default::default()
        },
        background_color: BackgroundColor(Color::rgb(0.65, 0.65, 0.65)),
        ..Default::default()
    }, InventoryUI))
    
    // now inner rectangle
    .with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            background_color: bkg_color,
            ..Default::default()
        })

        // now the different text inside box
        .with_children(|parent| 
        {
            // inventory title
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(100. * 1.)),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                background_color: bkg_color,
                ..Default::default()
            })
            .with_children(|parent| {
                
                // chose title based on State, either inventory or equipment
                let mut title = "";
                if *(turn_state.current()) == TurnState::InventoryPopup {
                    title = "Inventory"
                } else if *(turn_state.current()) == TurnState::EquipmentPopup {
                    title = "Equipment"
                }

                parent.spawn(TextBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(50. * 1.)),
                        margin: UiRect {
                            left: Val::Auto,
                            right: Val::Auto,
                            top: Val::Auto,
                            bottom: Val::Auto,
                        },
                        ..Default::default()
                    },
                    text: Text::from_section(
                        title.to_string(),
                        TextStyle {
                            font_size: 50.0,
                            font: font_manager.font.clone(),
                            color: Color::GOLD,
                        },
                    ),
                    ..Default::default()
                });
            });

            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(20. * INVENTORY_SLOTS as f32)),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                background_color: bkg_color,
                ..Default::default()
            })
            .with_children(|parent| {
                // create vector with text sections
                let mut sections = Vec::new();
                for _ in 0..INVENTORY_SLOTS {
                    sections.push(TextSection {
                        value: "\n ".to_string(),
                        style: TextStyle {
                            font: font_manager.font.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    });
                }
                parent.spawn((TextBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(20. * (INVENTORY_SLOTS+1) as f32)),
                        margin: UiRect {
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
                }, InventoryText));
            });

            // hint section
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Auto),
                    flex_direction: FlexDirection::ColumnReverse,
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Auto,
                        bottom: Val::Auto,
                    },
                    ..Default::default()
                },
                background_color: bkg_color,
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn((TextBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(20.)),
                        margin: UiRect {
                            left: Val::Auto,
                            right: Val::Auto,
                            top: Val::Auto,
                            bottom: Val::Auto,
                        },
                        ..Default::default()
                    },
                    text: Text::from_section(
                        " ".to_string(),
                        TextStyle {
                            font_size: 20.0,
                            font: font_manager.font.clone(),
                            color: Color::WHITE,
                        },
                    ),
                    ..Default::default()
                }, DescriptionText));
            });

        });
    });
}

fn player_input(
    mut chosen_item: EventWriter<ChosenItemEvent>,
    mut highlighted_item: ResMut<HighlightedItem>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut turn_state: ResMut<State<TurnState>>,
    player_items: Query<(Entity, &Carried), Without<Weapon>>,
    player_weapons: Query<(Entity, &Carried), With<Weapon>>,
) {

    // chose carried items based on State, either inventory or equipment
    let mut carried_items: usize = 0;
    if *(turn_state.current()) == TurnState::InventoryPopup {
        carried_items = player_items.iter().count();
    } else if *(turn_state.current()) == TurnState::EquipmentPopup {
        carried_items = player_weapons.iter().count();
    }

    let list_len = i32::min(carried_items as i32, INVENTORY_SLOTS) - 1;

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
            }
            KeyCode::Up => { // move to previous item in list
                highlighted_item.0 = i32::max(0, highlighted_item.0-1);
            }
            KeyCode::Down => { // move to next item in list
                highlighted_item.0 = i32::min(list_len as i32, highlighted_item.0+1);
            }
            _ => (),
        }
        keyboard_input.reset(key);
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

pub struct PopUpPlugin;
impl Plugin for PopUpPlugin {
    fn build(&self, app: &mut App) {
        app

        .add_event::<ChosenItemEvent>()
        .insert_resource(HighlightedItem(0))

        .add_plugin(inventory::InventoryPlugin)
        .add_plugin(equipment::EquipmentPlugin)

        // listening to user input on inventory screen
        .add_system_set(
            SystemSet::on_update(TurnState::InventoryPopup)
                .with_system(player_input)
        )
        .add_system_set(
            SystemSet::on_update(TurnState::EquipmentPopup)
                .with_system(player_input)
        )

        // cleanup when exiting
        .add_system_set(
            SystemSet::on_exit(TurnState::InventoryPopup)
                .with_system(despawn_menu)
        )
        .add_system_set(
            SystemSet::on_exit(TurnState::EquipmentPopup)
                .with_system(despawn_menu)
        )
        
        .add_system_set(
            SystemSet::on_enter(TurnState::InventoryPopup)
                .with_system(popup_ui)
        )
        .add_system_set(
            SystemSet::on_enter(TurnState::EquipmentPopup)
                .with_system(popup_ui)
        );

    }
}