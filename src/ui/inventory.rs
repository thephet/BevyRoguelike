use crate::prelude::*;

#[derive(Component)]
struct InventoryUI;

fn inventory_popup(
    mut commands: Commands,
    font: Res<Handle<Font>>,
) {

    commands
    .spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(50.), Val::Percent(50.)),
            flex_direction: FlexDirection::Column,
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
                //align_items: AlignItems::Stretch,
                ..Default::default()
            },
            color: UiColor(Color::rgb(0.0, 0.0, 0.0)),
            ..Default::default()
        })

        .with_children(|parent| {
            // Spawn menu text
            parent.spawn_bundle(TextBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Px(80. * 1.)),
                    margin: Rect {
                        left: Val::Auto,
                        right: Val::Auto,
                        bottom: Val::Auto,
                        top: Val::Auto,
                    },
                    ..Default::default()
                },
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Inventory".to_string(),
                            style: TextStyle {
                                font: font.clone(),
                                font_size: 60.0,
                                color: Color::GOLD,
                            },
                        },
                        TextSection {
                            value: "\nList of items.".to_string(),
                            style: TextStyle {
                                font: font.clone(),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        },
                    ],
                    alignment: TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                    },
                },
                ..Default::default()
            });
        });
    });
}

pub fn inventory_input(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut turn_state: ResMut<State<TurnState>>,
    mut player_query: Query<&mut Health, With<Player>>,
) {

    // get player max HP
    let mut health = player_query.single_mut();

    let key = keyboard_input.get_pressed().next().cloned();
    if let Some(key) = key {
        health.current = i32::min(health.max, health.current+1);
        // reset keyboard, bevys bug when changing states
        keyboard_input.reset(key);
        // update state
        turn_state.pop().unwrap();
        
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

        // listening to user input on inventory screen
        .add_system_set(
            SystemSet::on_update(TurnState::InventoryPopup)
                .with_system(inventory_input)
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