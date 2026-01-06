use crate::prelude::*;

use bevy::color::palettes::css::YELLOW;


// UI components
#[derive(Component)]
struct LogUI;

#[derive(Component)]
struct HPText;

#[derive(Component)]
struct HPBar;

#[derive(Component)]
struct InventoryText;

#[derive(Component)]
struct DungeonLevelText;

fn bottom_hud(
    mut commands: Commands,
    font_manager: Res<FontManager>,
) {

    commands
    // root node, just a black rectangle where the UI will be
    .spawn((Node {
        width: Val::Percent(100.0),
        height: Val::Px(100.0),
        position_type: PositionType::Absolute,
        left: Val::Px(0.0),
        bottom: Val::Px(0.0),
        ..Default::default()
    },
    BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
    TopUINode))
    // left vertical fill (content).
    .with_children(|parent| {
        // First the border rectangle
        parent.spawn((Node {
            width: Val::Percent(50.0),
            height: Val::Percent(100.0),
            border: UiRect::all(Val::Px(5.0)),
            ..Default::default()
        },
            BorderColor(Color::srgb(0.65, 0.65, 0.65)),
        ))
        // now inner rectangle
        .with_children(|parent| {
            parent.spawn((Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Stretch,
                    ..Default::default()
                },
                BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
            ))
            // text
            .with_children(|parent| {

                parent.spawn((
                    // Root text node: first "section"
                    Text::new("Log...\n"),
                    TextFont {
                        font: font_manager.font.clone(),
                        font_size: 20.0,
                        line_height: bevy::text::LineHeight::RelativeToFont(1.05),
                        ..Default::default()
                    },
                    TextColor(YELLOW.into()),
                    TextLayout::new_with_justify(JustifyText::Left),
                    Node {
                        margin: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    LogUI,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextSpan::new("Use the arrow keys to move.\n"),
                        TextFont {
                            font: font_manager.font.clone(),
                            font_size: 20.0,
                            line_height: bevy::text::LineHeight::RelativeToFont(1.05),
                            ..Default::default()
                        },
                        TextColor(YELLOW.into()),
                        LogUI,
                    ));

                    parent.spawn((
                        TextSpan::new("Bump into the enemies to attack them.\n"),
                        TextFont {
                            font: font_manager.font.clone(),
                            font_size: 20.0,
                            line_height: bevy::text::LineHeight::RelativeToFont(1.05),
                            ..Default::default()
                        },
                        TextColor(YELLOW.into()),
                        LogUI,
                    ));

                    parent.spawn((
                        TextSpan::new("Find the amulet to win the game.\n"),
                        TextFont {
                            font: font_manager.font.clone(),
                            font_size: 20.0,
                            line_height: bevy::text::LineHeight::RelativeToFont(1.05),
                            ..Default::default()
                        },
                        TextColor(YELLOW.into()),
                        LogUI,
                    ));
                });

            });
        });
        // right segment of the UI, first border
        parent.spawn((Node {
            width: Val::Percent(50.0),
            height: Val::Percent(100.0),
            border: UiRect::all(Val::Px(5.0)),
            ..Default::default()
        },
        BorderColor(Color::srgb(0.65, 0.65, 0.65)),
        ))
        // now inner rectangle
        .with_children(|parent| {
            parent.spawn((Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
            ))
            // top level with HP information
            // here we will place both the HP text and the HP bar
            .with_children(|parent| {
                parent.spawn((Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(33.0),
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
                ))
                // container where to place the HP text
                .with_children(|parent| {
                    parent
                        .spawn((Node {
                                width: Val::Percent(35.0),
                                height: Val::Percent(100.0),
                                ..Default::default()
                            },
                            BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
                        ))
                        // the actual HP text
                        .with_children(|parent| {

                            parent.spawn((
                                // The text content
                                Text::new("HP: 17 / 20"),
                                // Font/size
                                TextFont {
                                    font: font_manager.font.clone(),
                                    font_size: 20.0,
                                    ..Default::default()
                                },
                                TextColor(Color::srgb(0.99, 0.99, 0.99)),
                                TextLayout::new_with_justify(JustifyText::Left),
                                Node {
                                    height: Val::Px(20.0 * 1.0), // font size * lines
                                    margin: UiRect {
                                        left: Val::Auto,
                                        right: Val::Auto,
                                        bottom: Val::Auto,
                                        top: Val::Auto,
                                    },
                                    ..Default::default()
                                },
                                HPText,
                            ));

                        });
                    // outside HP bar
                    parent
                        .spawn((Node {
                                width: Val::Percent(63.0),
                                height: Val::Percent(90.0),
                                border: UiRect::all(Val::Px(5.0)),
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    bottom: Val::Auto,
                                    top: Val::Auto,
                                },
                                ..Default::default()
                            },
                            BackgroundColor(Color::srgb(0.5, 0.1, 0.1)),
                        ))
                        // inside HP bar
                        .with_children(|parent| {
                            parent.spawn((Node {
                                    border: UiRect::all(Val::Px(3.0)),
                                    width: Val::Percent(50.0),
                                    height: Val::Percent(90.0),
                                    margin: UiRect {
                                        //left: Val::Auto,
                                        bottom: Val::Auto,
                                        top: Val::Auto,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                BackgroundColor(Color::srgb(0.99, 0.1, 0.1)),
                            HPBar));
                        });
                });

                // Node for the Inventory text
                parent.spawn((Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(33.0),
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
                ))
                    // container where to place the Inventory text
                    .with_children(|parent| {
                        parent
                            .spawn((Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..Default::default()
                            },
                            BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
                            ))
                        // the actual Inventory text
                        .with_children(|parent| {

                            parent.spawn((
                                Text::new("(I)nventory"),
                                TextFont {
                                    font: font_manager.font.clone(),
                                    font_size: 20.0,
                                    ..Default::default()
                                },
                                TextColor(Color::srgb(0.99, 0.99, 0.99)),
                                TextLayout::new_with_justify(JustifyText::Left),
                                Node {
                                    height: Val::Px(20.0 * 1.0),
                                    margin: UiRect {
                                        left: Val::Auto,
                                        right: Val::Auto,
                                        bottom: Val::Auto,
                                        top: Val::Auto,
                                    },
                                    ..Default::default()
                                },
                                InventoryText,
                            ));

                        });
                    })

                    // container where to place the Equipment text
                    .with_children(|parent| {
                        parent
                            .spawn((Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..Default::default()
                            },
                            BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
                            ))
                        // the actual Equipment text
                        .with_children(|parent| {

                            parent.spawn((
                                Text::new("(E)quipment"),
                                TextFont {
                                    font: font_manager.font.clone(),
                                    font_size: 20.0,
                                    ..Default::default()
                                },
                                TextColor(Color::srgb(0.99, 0.99, 0.99)),
                                TextLayout::new_with_justify(JustifyText::Left),
                                Node {
                                    height: Val::Px(20.0 * 1.0),
                                    margin: UiRect {
                                        left: Val::Auto,
                                        right: Val::Auto,
                                        bottom: Val::Auto,
                                        top: Val::Auto,
                                    },
                                    ..Default::default()
                                },
                                InventoryText,
                            ));

                        });
                    });

                // Node for the Dungeon Level text
                parent.spawn((Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(33.0),
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
                ))
                // container where to place the Dungeon level text
                .with_children(|parent| {
                    parent.spawn((Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..Default::default()
                            },
                        BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
                    ))
                    // the actual Dungeon level text
                    .with_children(|parent| {

                        parent.spawn((
                            Text::new("Dungeon Level: 1"),
                            TextFont {
                                font: font_manager.font.clone(),
                                font_size: 20.0,
                                ..Default::default()
                            },
                            TextColor(Color::srgb(0.99, 0.99, 0.99)),
                            TextLayout::new_with_justify(JustifyText::Left),
                            Node {
                                height: Val::Px(20.0 * 1.0),
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    bottom: Val::Auto,
                                    top: Val::Auto,
                                },
                                ..Default::default()
                            },
                            DungeonLevelText,
                        ));

                    });
                });

            });
        });
    });
}


fn update_game_log(
    game_log: Res<GameLog>,
    // Parent LogUI entity: root Text + its children
    mut log_ui_q: Query<(&mut Text, &Children), With<LogUI>>,
    // All text spans (children "sections")
    mut spans_q: Query<&mut TextSpan>,
) {
    if let Ok((mut root_text, children)) = log_ui_q.single_mut() {
        for (i, entry) in game_log.entries.iter().enumerate() {
            if i == 0 {
                // First log entry goes into the root Text
                root_text.0 = entry.clone();
            } else {
                // Remaining entries go into the child spans
                let span_index = i - 1;

                if let Some(&child_entity) = children.get(span_index) {
                    if let Ok(mut span) = spans_q.get_mut(child_entity) {
                        span.0 = entry.clone();
                    }
                }
            }
        }
    }
}


fn update_dungeonleveltext(
    player_q: Query<&Player>,
    mut text_query: Query<&mut Text, Added<DungeonLevelText>>,
) {
    let Ok(level) = player_q.single() else {
        panic!("Can't get map level")
    };

    // Update dungeon level text
    for mut text in text_query.iter_mut() {
        text.0 = format!("Dungeon Level: {}", level.map_level + 1);
    }
}


fn update_hp_text_and_bar(
    mut text_query: Query<&mut Text, With<HPText>>,
    mut bar_query: Query<&mut Node, With<HPBar>>,
    player_query: Query<&Health, With<Player>>,
) {
    for player_hp in &player_query {
        let (current, max) = (player_hp.current, player_hp.max);

        // ---------- update HP text ----------
        for mut text in &mut text_query {
            text.0 = format!("HP: {} / {}", current, max);
        }

        // ---------- update HP bar fill ----------
        let bar_fill = (current as f32 / max as f32) * 100.0;

        for mut bar in &mut bar_query {
            bar.width = Val::Percent(bar_fill);
        }
    }
}

pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app 

            .add_systems(OnExit(TurnState::StartScreen), bottom_hud)
            .add_systems(OnExit(TurnState::NextLevel), bottom_hud)

            .add_systems(
                Update,
                (update_hp_text_and_bar, update_game_log, update_dungeonleveltext)
                .run_if(in_state(TurnState::AwaitingInput))
            );
    }
}