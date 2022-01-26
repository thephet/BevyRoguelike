use crate::prelude::*;

#[derive(Component)]
struct MenuUI;

#[derive(Component)]
struct GameOverUI;

fn setup_menu(
    mut commands: Commands,
    font: Res<Handle<Font>>,
) {
    commands
    .spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        color: UiColor(Color::rgb(0.0, 0.0, 0.0)),
        ..Default::default()
    })
    .insert(MenuUI)
    .with_children(|parent| {
        // Spawn menu text
        parent.spawn_bundle(TextBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Px(140. * 1.)),
                margin: Rect {
                    left: Val::Auto,
                    right: Val::Auto,
                    bottom: Val::Auto,
                    top: Val::Auto,
                },
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "Rogue Quest".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 100.0,
                            color: Color::GOLD,
                        },
                    },
                    TextSection {
                        value: "\nPress any key to start game.".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
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
}

// function to kill either start screen or game over screen
fn despawn_menu(
    mut commands: Commands, 
    query_startscreen: Query<Entity, With<MenuUI>>,
    query_gameoverscreen: Query<Entity, With<GameOverUI>>,
) {
    for e in query_startscreen.iter() {
        commands.entity(e).despawn_recursive();
    }
    for e in query_gameoverscreen.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn gameover_screen(
    mut commands: Commands,
    font: Res<Handle<Font>>,
    top_ui_node_q: Query<Entity, With<TopUINode>>,
) {

    // First we need to remove ALL the other stuff around the game
    let top_ui_node = top_ui_node_q.single();
    commands.entity(top_ui_node).despawn_recursive();

    commands
    .spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        color: UiColor(Color::rgb(0.0, 0.0, 0.0)),
        ..Default::default()
    })
    .insert(GameOverUI)
    .with_children(|parent| {
        // Spawn menu text
        parent.spawn_bundle(TextBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Px(140. * 1.)),
                margin: Rect {
                    left: Val::Auto,
                    right: Val::Auto,
                    bottom: Val::Auto,
                    top: Val::Auto,
                },
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "Game Over".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 100.0,
                            color: Color::RED,
                        },
                    },
                    TextSection {
                        value: "\nPress any key to continue.".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
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
}

fn victory_screen(
    mut commands: Commands,
    font: Res<Handle<Font>>,
    top_ui_node_q: Query<Entity, With<TopUINode>>,
) {

    // First we need to remove ALL the other stuff around the game
    let top_ui_node = top_ui_node_q.single();
    commands.entity(top_ui_node).despawn_recursive();

    commands
    .spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        color: UiColor(Color::rgb(0.0, 0.0, 0.0)),
        ..Default::default()
    })
    .insert(GameOverUI)
    .with_children(|parent| {
        // Spawn menu text
        parent.spawn_bundle(TextBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Px(140. * 1.)),
                margin: Rect {
                    left: Val::Auto,
                    right: Val::Auto,
                    bottom: Val::Auto,
                    top: Val::Auto,
                },
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "You win!".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 100.0,
                            color: Color::GOLD,
                        },
                    },
                    TextSection {
                        value: "\nPress any key to continue.".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
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
}

pub fn start_screen_input(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut turn_state: ResMut<State<TurnState>>
) {

    let key = keyboard_input.get_pressed().next().cloned();
    if let Some(key) = key {
        // reset keyboard, bevys bug when changing states
        keyboard_input.reset(key);
        // update state
        if *turn_state.current() == TurnState::StartScreen {
            turn_state.set(TurnState::AwaitingInput).unwrap();
        } else {
            turn_state.set(TurnState::StartScreen).unwrap();
        }
        
    }
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // setup when entering the start screen
            .add_system_set(
                SystemSet::on_enter(TurnState::StartScreen)
                    .with_system(setup_menu)
            )
            // setup when on the start screen
            .add_system_set(
                SystemSet::on_update(TurnState::StartScreen)
                    .with_system(start_screen_input)
            )
            // cleanup when exiting the start screen
            .add_system_set(
                SystemSet::on_exit(TurnState::StartScreen)
                    .with_system(despawn_menu)
            )

            // setup when entering the gameover screen
            .add_system_set(
                SystemSet::on_enter(TurnState::GameOver)
                    .with_system(gameover_screen)
            )
            // setup when on the start screen
            .add_system_set(
                SystemSet::on_update(TurnState::GameOver)
                    .with_system(start_screen_input)
            )
            // cleanup when exiting the start screen
            .add_system_set(
                SystemSet::on_exit(TurnState::GameOver)
                    .with_system(despawn_menu)
            )

            // setup when entering the gameover screen
            .add_system_set(
                SystemSet::on_enter(TurnState::Victory)
                    .with_system(victory_screen)
            )
            // setup when on the start screen
            .add_system_set(
                SystemSet::on_update(TurnState::Victory)
                    .with_system(start_screen_input)
            )
            // cleanup when exiting the start screen
            .add_system_set(
                SystemSet::on_exit(TurnState::Victory)
                    .with_system(despawn_menu)
            );
    }
}