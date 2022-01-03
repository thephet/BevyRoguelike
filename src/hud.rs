use crate::prelude::*;


fn tooltip_ui(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font: Handle<Font> = asset_server.load("fonts/dos.ttf");

    commands
    // root node, just a black rectangle where the text will be
    .spawn_bundle(NodeBundle {
        // by default we set visible to false so it starts hidden
        visible: Visible { is_visible: false, is_transparent: true },
        style: Style {
            size: Size::new(Val::Px(200.0), Val::Px(30.0)),
            flex_direction: FlexDirection::ColumnReverse,
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        material: color_materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
        ..Default::default()
    })
    .with_children(|parent| {
        // text
        parent.spawn_bundle(TextBundle {
            visible: Visible { is_visible: false, is_transparent: true },
            style: Style {
                size: Size::new(Val::Auto, Val::Px(20. * 1.)),
                margin: Rect::all(Val::Auto),
                ..Default::default()
            },
            text: Text::with_section(
                "Goblin. HP: 2 / 2",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .insert(ToolTipText);
    })
    .insert(ToolTipBox);
}


fn bottom_ui(
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
struct ToolTipText;
struct ToolTipBox;

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

// when leaving user input state, hide tooltip
fn hide_tooltip(
    mut text_box_query : QuerySet<(
        Query<&mut Visible, With<ToolTipText>>,
        Query<&mut Visible, With<ToolTipBox>>
    )>,
) {
    // update tooltip visiblity
    for mut visible in text_box_query.q0_mut().iter_mut() {
        visible.is_visible = false;
    }

    // update box visibility
    for mut visible in text_box_query.q1_mut().iter_mut() {
        visible.is_visible = false;
    } 
}

// when user left clicks, update tooltip and make it visible
fn update_tooltip(
    // need to get window dimensions
    wnds: Res<Windows>,
    // to get the mouse clicks
    buttons: Res<Input<MouseButton>>,
    // query to get camera transform
    q_camera: Query<&Transform, With<MainCamera>>,
    // query to get all the entities with Name component
    q_names: Query<(&Naming, &Health, &Position)>,
    // // query to get tooltip text and box
    mut text_box_query : QuerySet<(
        Query<(&mut Text, &mut Visible), With<ToolTipText>>,
        Query<(&mut Style, &mut Visible), With<ToolTipBox>>
    )>,
) {
    // if the user left clicks
    if buttons.just_pressed(MouseButton::Left) {
        // get the primary window
        let wnd = wnds.get_primary().unwrap();

        // check if the cursor is in the primary window
        if let Some(pos) = wnd.cursor_position() {
            // get the size of the window
            let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            // the default orthographic projection is in pixels from the center;
            // just undo the translation
            let p = pos - size / 2.0;

            // assuming there is exactly one main camera entity, so this is OK
            let camera_transform = q_camera.single().unwrap();

            let tile_size_x = wnd.width() / SCREEN_WIDTH as f32;
            let tile_size_y = wnd.height() / SCREEN_HEIGHT as f32;

            // apply the camera transform
            let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

            // transform world coordinates to our grid
            let grid_x = (pos_wld.x / tile_size_x) + (SCREEN_WIDTH / 2) as f32;
            let grid_y = (pos_wld.y / tile_size_y) + (SCREEN_HEIGHT / 2) as f32 - (UI_HEIGHT/2) as f32;

            // now we go through all the entities with name to see which one is the nearest
            // some variables placeholders to save the entity name and its health
            let mut distance = 9999.9;
            let mut s = String::new();
            let mut maxh = 0;
            let mut currenth = 0;
            for (name, health, pos) in q_names.iter() {
                // calculate eucledian distance from click to all entities with Naming
                let mut dist = (grid_x - pos.x as f32).powi(2) + (grid_y - pos.y as f32).powi(2);
                dist = dist.sqrt();
                
                if dist < distance {
                    distance = dist;
                    s = name.0.clone();
                    maxh = health.max;
                    currenth = health.current;
                }
            }

            // update tooltip text
            for (mut text, mut visible) in text_box_query.q0_mut().iter_mut() {
                text.sections[0].value = format!("{} HP: {} / {}", s, currenth, maxh);
                visible.is_visible = true;
            }

            // update box position
            for (mut boxnode, mut visible) in text_box_query.q1_mut().iter_mut() {
                boxnode.position.left = Val::Px(pos.x-100.0);
                boxnode.position.bottom = Val::Px(pos.y);
                visible.is_visible = true;
            }
        }
    }
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(bottom_ui.system())
            .add_startup_system(tooltip_ui.system())
           .add_system(update_hp_text_and_bar.system())
           
           .add_system_set(
            SystemSet::on_update(TurnState::AwaitingInput)
             .with_system(update_tooltip.system())
            )

           .add_system_set(
               SystemSet::on_exit(TurnState::AwaitingInput)
                .with_system(hide_tooltip.system())
           );
    }
}