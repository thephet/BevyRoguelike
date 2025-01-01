use crate::prelude::*;

#[derive(Component)]
struct ToolTipText;

#[derive(Component)]
struct ToolTipBox;

fn tooltip_ui(
    mut commands: Commands,
    font_manager: Res<FontManager>,
) {
    let game_log= GameLog::new();
    commands.insert_resource(game_log);

    commands
    // root node, just a black rectangle where the text will be
    .spawn((NodeBundle {
        // by default we set visible to false so it starts hidden
        visibility: Visibility::Hidden,
        style: Style {
            width: Val::Px(200.0),
            height: Val::Px(30.0),
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
        ..Default::default()
    }, ToolTipBox))
    .with_children(|parent| {
        // text
        parent.spawn((TextBundle {
            visibility: Visibility::Hidden,
            style: Style {
                height: Val::Px(20. * 1.),
                margin: UiRect::all(Val::Auto),
                ..Default::default()
            },
            text: Text::from_section(
                "Goblin. HP: 2 / 2",
                TextStyle {
                    font: font_manager.font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            ..Default::default()
        }, ToolTipText));
    });
}


// when leaving user input state, hide tooltip
fn hide_tooltip(
    mut text_box_query : ParamSet<(
        Query<&mut Visibility, With<ToolTipText>>,
        Query<&mut Visibility, With<ToolTipBox>>
    )>,
) {
    // update tooltip visibility
    for mut visible in text_box_query.p0().iter_mut() {
        *visible = Visibility::Hidden;
    }

    // update box visibility
    for mut visible in text_box_query.p1().iter_mut() {
        *visible = Visibility::Hidden;
    } 
}

// when user left clicks, update tooltip and make it visible
fn update_tooltip(
    // need to get window dimensions
    wnds: Query<&Window, With<PrimaryWindow>>,
    // to get the mouse clicks
    buttons: Res<ButtonInput<MouseButton>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    // query to get all the entities with Name component
    q_names: Query<(&Naming, &Position, Option<&Health>)>,
    // // query to get tooltip text and box
    mut text_box_query : ParamSet<(
        Query<(&mut Text, &mut Visibility), With<ToolTipText>>,
        Query<(&mut Style, &mut Visibility), With<ToolTipBox>>
    )>,
    // query to get the player field of view
    player_fov_q: Query<&FieldOfView, With<Player>>,
) {
    // if the user left clicks
    if buttons.just_pressed(MouseButton::Left) {
        // get the primary window
        let wnd = wnds.get_single().unwrap();

        // check if the cursor is in the primary window
        if let Some(pos) = wnd.cursor_position() {

            // assuming there is exactly one main camera entity, so this is OK
            let (camera, camera_transform) = q_camera.single();

            let tile_size_x = wnd.width() / SCREEN_WIDTH as f32;
            let tile_size_y = wnd.height() / SCREEN_HEIGHT as f32;

            // apply the camera transform
            let point_wld = camera.viewport_to_world_2d(camera_transform, pos).unwrap();

            // transform world coordinates to our grid
            let grid_x = (point_wld.x / tile_size_x) + (SCREEN_WIDTH / 2) as f32;
            let grid_y = (point_wld.y / tile_size_y) + (SCREEN_HEIGHT / 2) as f32 - (UI_HEIGHT / 2) as f32;
            let grid_position = Position{x: grid_x as i32, y: grid_y as i32, z:0};

            // now we go through all the entities with name to see which one is the nearest
            // some variables placeholders to save the entity name and its health
            let mut good_click = false;
            let mut s = String::new();
            let mut maxh = 0;
            let mut currenth = 0;
            // obtain also player fov
            let player_fov = player_fov_q.single();

            q_names.iter()
                .filter(|(_, pos, _)| 
                    **pos == grid_position && player_fov.visible_tiles.contains( &((**pos).into()) ) )
                .for_each(|(name, _, health)| {
                    s = name.0.clone();
                    good_click = true;
                    // if it also has health component
                    if let Some(health) = health {
                        maxh = health.max;
                        currenth = health.current;
                    }
                });

            // update tooltip text
            for (mut text, mut visible) in text_box_query.p0().iter_mut() {
                if currenth > 0 {
                    text.sections[0].value = format!("{} HP: {} / {}", s, currenth, maxh);
                } else {
                    text.sections[0].value = format!("{}", s);
                }
                *visible = Visibility::Visible;
            }

            // update box position
            for (mut boxnode, mut visible) in text_box_query.p1().iter_mut() {
                if good_click {
                    boxnode.left = Val::Px(pos.x-100.0);
                    boxnode.top = Val::Px(pos.y-40.0);
                    *visible = Visibility::Visible;
                } else {
                    *visible = Visibility::Hidden;
                }
                
            }
        }
    }
}

pub struct TooltipsPlugin;
impl Plugin for TooltipsPlugin {
    fn build(&self, app: &mut App) {
        app

        .add_systems(OnExit(TurnState::StartScreen), tooltip_ui)
        .add_systems(Update, update_tooltip.run_if(in_state(TurnState::AwaitingInput)))        
        .add_systems(OnExit(TurnState::AwaitingInput), hide_tooltip);
    }
}