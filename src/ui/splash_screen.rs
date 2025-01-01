use crate::prelude::*;
use bevy::color::palettes::css::*;


#[derive(Component)]
struct MenuUI;

#[derive(Component)]
struct GameOverUI;

fn splash_screen(
    mut commands: Commands,
    font_manager: Res<FontManager>,
    turn_state: Res<State<TurnState>>,
    top_ui_node_q: Query<Entity, With<TopUINode>>,
) {
    // If we are not in StartScreen we need to remove ALL the other UI stuff around the game
    if *turn_state.get() != TurnState::StartScreen {
        let top_ui_node = top_ui_node_q.single();
        commands.entity(top_ui_node).despawn_recursive();
    }

    commands
    .spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..Default::default()
        },
        background_color: BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
        ..Default::default()
    })
    .insert(MenuUI)
    .with_children(|parent| {

        // chose title based on State
        let mut title = "";
        let mut title_color = GOLD.into();
        if *turn_state.get() == TurnState::StartScreen {
            title = "Rogue Quest";
        } else if *turn_state.get() == TurnState::GameOver {
            title = "Game Over";
            title_color = RED.into();
        } else if *turn_state.get() == TurnState::Victory {
            title = "You win!";
        } else if *turn_state.get() == TurnState::NextLevel {
            title = "Level Completed";
        }

        // Spawn menu text
        parent.spawn(TextBundle {
            style: Style {
                height: Val::Px(140. * 1.),
                margin: UiRect {
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
                        value: title.to_string(),
                        style: TextStyle {
                            font: font_manager.font.clone(),
                            font_size: 100.0,
                            color: title_color,
                        },
                    },
                    TextSection {
                        value: "\nPress any key to start game.".to_string(),
                        style: TextStyle {
                            font: font_manager.font.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                justify: JustifyText::Center,
                ..default()
            },
            ..Default::default()
        });
    });
}

// function to kill the current menu screen
fn despawn_splashscreen(
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

pub fn start_screen_input(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    turn_state: ResMut<State<TurnState>>,
    mut next_state: ResMut<NextState<TurnState>>
) {

    let key = keyboard_input.get_just_pressed().next().cloned();

    if let Some(_key) = key {
        // update state
        if (*turn_state.get() == TurnState::StartScreen) || 
            (*turn_state.get() == TurnState::NextLevel) 
        {
            next_state.set(TurnState::AwaitingInput);
        } else {
            next_state.set(TurnState::StartScreen);
        }
        
    }
    
    keyboard_input.reset_all();
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // setup when entering the start screen
            .add_systems(OnEnter(TurnState::StartScreen), splash_screen)
            // setup when on the start screen
            .add_systems(Update, start_screen_input.run_if(in_state(TurnState::StartScreen)))
            // cleanup when exiting the start screen
            .add_systems(OnExit(TurnState::StartScreen), despawn_splashscreen)

            // setup when entering the gameover screen
            .add_systems(OnEnter(TurnState::GameOver), splash_screen)
            // setup when on the gameover screen
            .add_systems(Update, start_screen_input.run_if(in_state(TurnState::GameOver)))
            // cleanup when exiting the gameover screen
            .add_systems(OnExit(TurnState::GameOver), despawn_splashscreen)


            // setup when entering the victory screen
            .add_systems(OnEnter(TurnState::Victory), splash_screen)
            // setup when on the victory screen
            .add_systems(Update, start_screen_input.run_if(in_state(TurnState::Victory)))
            // cleanup when exiting the victory screen
            .add_systems(OnExit(TurnState::Victory), despawn_splashscreen)

            // setup when entering the next level screen
            .add_systems(OnEnter(TurnState::NextLevel), splash_screen)
            // setup when on the next level screen
            .add_systems(Update, start_screen_input.run_if(in_state(TurnState::NextLevel)))
            // cleanup when exiting the next level screen
            .add_systems(OnExit(TurnState::NextLevel), despawn_splashscreen);
    }
}