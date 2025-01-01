use crate::prelude::*;
use super::inventory;
use super::equipment;

use bevy::color::palettes::css::GOLD;

#[derive(Component)]
pub struct InventoryUI;

#[derive(Component)]
pub struct InventoryText;

#[derive(Component)]
pub struct DescriptionText;

#[derive(Event)]
pub struct ChosenItemEvent(pub i32);

#[derive(Resource)]
pub struct HighlightedItem(pub i32);

pub const INVENTORY_SLOTS: i32 = 10;

fn popup_ui(
    mut commands: Commands,
    font_manager: Res<FontManager>,
    popup_state: Res<State<PopUpState>>,
) {
    // background color for the inventory window
    let bkg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));

    commands
    .spawn((NodeBundle {
        style: Style {
            width: Val::Percent(50.),
            height: Val::Percent(50.),
            position_type: PositionType::Absolute,
            left: Val::Percent(25.0),
            bottom: Val::Percent(30.0),
            border: UiRect::all(Val::Px(5.0)),
            ..Default::default()
        },
        background_color: BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
        ..Default::default()
    }, InventoryUI))
    
    // now inner rectangle
    .with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
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
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                background_color: bkg_color,
                ..Default::default()
            })
            .with_children(|parent| {
                
                // chose title based on State, either inventory or equipment
                let mut title = "";
                if *popup_state.get() == PopUpState::InventoryPopup {
                    title = "Inventory"
                } else if *popup_state.get() == PopUpState::EquipmentPopup {
                    title = "Equipment"
                }

                parent.spawn(TextBundle {
                    style: Style {
                        height: Val::Px(50. * 1.),
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
                            color: GOLD.into(),
                        },
                    ),
                    ..Default::default()
                });
            });

            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(20. * INVENTORY_SLOTS as f32),
                    flex_direction: FlexDirection::Column,
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
                        height: Val::Px(20. * (INVENTORY_SLOTS+1) as f32),
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
                    width: Val::Percent(100.0),
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
                        height: Val::Px(20.),
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
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    popup_currentstate: ResMut<State<PopUpState>>,
    mut popup_nextstate: ResMut<NextState<PopUpState>>,
    mut turn_nextstate: ResMut<NextState<TurnState>>,
    player_items: Query<(Entity, &Carried), Without<Weapon>>,
    player_weapons: Query<(Entity, &Carried), With<Weapon>>,
) {

    // chose carried items based on State, either inventory or equipment
    let mut carried_items: usize = 0;
    if *popup_currentstate.get() == PopUpState::InventoryPopup {
        carried_items = player_items.iter().count();
    } else if *popup_currentstate.get() == PopUpState::EquipmentPopup {
        carried_items = player_weapons.iter().count();
    }

    let list_len = i32::min(carried_items as i32, INVENTORY_SLOTS) - 1;

    let key = keyboard_input.get_pressed().next().cloned();
    if let Some(key) = key 
    {
        match key {
            KeyCode::Escape => { // close inventory window
                // update state
                popup_nextstate.set(PopUpState::None);
                turn_nextstate.set(TurnState::AwaitingInput);
            }
            KeyCode::Enter => { // activate selected item and close inventory window
                chosen_item.send(ChosenItemEvent(highlighted_item.0));
            }
            KeyCode::ArrowUp => { // move to previous item in list
                highlighted_item.0 = i32::max(0, highlighted_item.0-1);
            }
            KeyCode::ArrowDown => { // move to next item in list
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

        .add_plugins(inventory::InventoryPlugin)
        .add_plugins(equipment::EquipmentPlugin)

        // listening to user input on inventory screen
        .add_systems(Update, player_input.run_if(in_state(PopUpState::InventoryPopup)))
        .add_systems(Update, player_input.run_if(in_state(PopUpState::EquipmentPopup)))
        
        // cleanup when exiting
        .add_systems(OnExit(PopUpState::InventoryPopup), despawn_menu)
        .add_systems(OnExit(PopUpState::EquipmentPopup), despawn_menu)

        // creating when entering
        .add_systems(OnEnter(PopUpState::InventoryPopup), popup_ui)
        .add_systems(OnEnter(PopUpState::EquipmentPopup), popup_ui);

    }
}