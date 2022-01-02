use crate::prelude::*;

fn setup_ui(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font: Handle<Font> = asset_server.load("fonts/dos.ttf");

    commands

    // root node, just a black rectangle where the UI will be
    .spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
            justify_content: JustifyContent::SpaceBetween,
            ..Default::default()
        },
        material: color_materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
        ..Default::default()
    })
    // left vertical fill (content).
    .with_children(|parent| {
        // First the border rectangle
        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                border: Rect::all(Val::Px(5.0)),
                ..Default::default()
            },
            material: color_materials.add(Color::rgb(0.65, 0.65, 0.65).into()),
            ..Default::default()
        })
        // now inner rectangle
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    //align_items: AlignItems::Stretch,
                    ..Default::default()
                },
                material: color_materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
                ..Default::default()
            })

            // text
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                        style: Style {
                            align_self: AlignSelf::FlexEnd,
                            margin: Rect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        // Use `Text` directly
                        text: Text {
                            // Construct a `Vec` of `TextSection`s
                            sections: vec![
                                TextSection {
                                    value: "Log...".to_string(),
                                    style: TextStyle {
                                        font: font.clone(),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                },
                                TextSection {
                                    value: "\nUse the arrow keys to move.".to_string(),
                                    style: TextStyle {
                                        font: font.clone(),
                                        font_size: 20.0,
                                        color: Color::GOLD,
                                    },
                                },
                                TextSection {
                                    value: "\nBump into the enemies to attack them.".to_string(),
                                    style: TextStyle {
                                        font: font.clone(),
                                        font_size: 20.0,
                                        color: Color::GOLD,
                                    },
                                },
                                TextSection {
                                    value: "\nFind the amulet to win the game.".to_string(),
                                    style: TextStyle {
                                        font: font.clone(),
                                        font_size: 20.0,
                                        color: Color::GOLD,
                                    },
                                },
                            ],
                            alignment: TextAlignment {
                                horizontal: HorizontalAlign::Left,
                                vertical: VerticalAlign::Center,
                            },
                        },
                        ..Default::default()
                    })
                    .insert(LogUI);
            });
        });

        // right segment of the UI, first border
        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                border: Rect::all(Val::Px(5.0)),
                ..Default::default()
            },
            material: color_materials.add(Color::rgb(0.65, 0.65, 0.65).into()),
            ..Default::default()
        })
        // now inner rectangle
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::FlexEnd,
                    ..Default::default()
                },
                material: color_materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
                ..Default::default()
            })
            // top level with HP information
            // here we will place both the HP text and the HP bar
            .with_children(|parent| {
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(33.0)),
                        // Place content up to down
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    material: color_materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
                    ..Default::default()
                })
                // container where to place the HP text
                .with_children(|parent| {
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(35.0), Val::Percent(100.0)),
                            // Place content up to down
                            flex_direction: FlexDirection::ColumnReverse,
                            ..Default::default()
                        },
                        material: color_materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
                        ..Default::default()
                    })
                    // the actual HP text
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            style: Style {
                                // Set height to font size * number of text lines
                                size: Size::new(Val::Auto, Val::Px(20. * 1.)),
                                // Set left margin to auto to push the text to the right
                                margin: Rect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    bottom: Val::Auto,
                                    top: Val::Auto,
                                },
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "HP: 20 / 20".to_string(),
                                TextStyle {
                                    font_size: 20.0,
                                    font: font.clone(),
                                    color: Color::rgb(0.99, 0.99, 0.99),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(HPText);
                    });
                    // outside HP bar
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(65.0), Val::Px(20. * 1.)),
                            border: Rect::all(Val::Px(5.0)),
                            margin: Rect {
                                left: Val::Auto,
                                right: Val::Auto,
                                bottom: Val::Auto,
                                top: Val::Auto,
                            },
                            ..Default::default()
                        },
                        material: color_materials.add(Color::rgb(0.5, 0.1, 0.1).into()),
                        ..Default::default()
                    })
                    // inside HP bar
                    .with_children(|parent| {
                        parent.spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                                ..Default::default()
                            },
                            material: color_materials.add(Color::rgb(0.99, 0.1, 0.1).into()),
                            ..Default::default()
                        })
                        .insert(HPBar);
                    });
                });
            });
        });
    });
}

struct LogUI;

struct HPText;
struct HPBar;

fn update_hp_text_and_bar(
    mut text_query: Query<&mut Text, With<HPText>>,
    mut bar_query: Query<&mut Style, With<HPBar>>,
    player_query: Query<&Health, With<Player>>,
) {

    // get player max HP and current hp
    if let Ok(player_hp) = player_query.single() {
        let (current, max) = (player_hp.current, player_hp.max);

        // update HP text
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("HP: {} / {}", current, max);
        }

        // update HP bar
        let bar_fill = (current as f32 / max as f32) * 100.0;
        for mut bar in bar_query.iter_mut() {
            bar.size.width = Val::Percent(bar_fill);
        }
    }
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_ui.system())
           .add_system(update_hp_text_and_bar.system());
    }
}