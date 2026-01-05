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
        .spawn((Node {
            width: Val::Percent(50.),
            height: Val::Percent(50.),
            position_type: PositionType::Absolute,
            left: Val::Percent(25.0),
            bottom: Val::Percent(30.0),
            border: UiRect::all(Val::Px(5.0)),
            ..Default::default()
        },
        BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
        InventoryUI))
    
    // now inner rectangle
    .with_children(|parent| {
        parent.spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            border: UiRect::all(Val::Px(5.0)),
            ..Default::default()
        },
        BorderColor(Color::srgb(0.85, 0.85, 0.85)),
        bkg_color,
        ))
        // now the different text inside box
        .with_children(|parent| 
        {
            // inventory title
            parent.spawn((Node {
                width: Val::Percent(100.0),
                height: Val::Px(90.0),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            bkg_color,
            ))
            .with_children(|parent| {
                
                // chose title based on State, either inventory or equipment
                let mut title = "";
                if *popup_state.get() == PopUpState::InventoryPopup {
                    title = "Inventory"
                } else if *popup_state.get() == PopUpState::EquipmentPopup {
                    title = "Equipment"
                }

                parent.spawn((
                    Text::new(title.to_string()),
                    TextFont {
                        font: font_manager.font.clone(),
                        font_size: 50.0,
                        ..default()
                    },
                    TextColor(GOLD.into()),
                    TextLayout::new_with_justify(JustifyText::Center),
                    Node {
                        margin: UiRect {
                            left: Val::Auto,
                            right: Val::Auto,
                            bottom: Val::Auto,
                            top: Val::Px(15.0),
                        },
                        ..default()
                    },
                ));
            });

            parent.spawn((Node {
                width: Val::Percent(100.0),
                height: Val::Px(20. * INVENTORY_SLOTS as f32),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            bkg_color,
            ))


            .with_children(|parent| {
                parent
                    .spawn((
                        // Parent text entity (no direct content, we’ll use child spans)
                        Text::default(),
                        TextFont {
                            font: font_manager.font.clone(),
                            font_size: 20.0,
                            ..Default::default()
                        },
                        // Layout of the whole text block
                        TextLayout::new_with_justify(JustifyText::Left),
                        // UI layout (replaces Style in TextBundle)
                        Node {
                            height: Val::Px(20.0 * (INVENTORY_SLOTS + 1) as f32),
                            margin: UiRect {
                                left: Val::Auto,
                                right: Val::Auto,
                                top: Val::Auto,
                                bottom: Val::Auto,
                            },
                            ..Default::default()
                        },
                        InventoryText,
                    ))
                    .with_children(|parent| {
                        // Create one span per inventory slot
                        for _ in 0..INVENTORY_SLOTS {
                            parent.spawn((
                                TextSpan::new("\n ".to_string()),
                                TextFont {
                                    font: font_manager.font.clone(),
                                    font_size: 20.0,
                                    ..Default::default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        }
                    });
            });

            // hint section
            parent.spawn((Node {
                width: Val::Percent(100.0),
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Auto,
                    bottom: Val::Auto,
                },
                ..Default::default()
            },
            bkg_color,
            ))
            .with_children(|parent| {

                parent.spawn((
                    Text::new(" ".to_string()),
                    TextFont {
                        font: font_manager.font.clone(),
                        font_size: 20.0,
                        ..default()
                    },
                    Node {
                        margin: UiRect {
                            left: Val::Auto,
                            right: Val::Auto,
                            bottom: Val::Auto,
                            top: Val::Px(15.0),
                        },
                        ..default()
                    },
                    TextColor(Color::WHITE.into()),
                    TextLayout::new_with_justify(JustifyText::Center),
                    DescriptionText));
                
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