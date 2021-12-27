use bevy::prelude::*;

fn setup_ui(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font: Handle<Font> = asset_server.load("fonts/dos.ttf");
    let material = color_materials.add(Color::NONE.into());

    commands
    .spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            border: Rect::all(Val::Px(5.0)),
            ..Default::default()
        },
        material: color_materials.add(Color::rgb(0.65, 0.65, 0.65).into()),
        ..Default::default()
    })
    .with_children(|parent| {
        // left vertical fill (content)
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::FlexEnd,
                    ..Default::default()
                },
                material: color_materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
                ..Default::default()
            })
            .with_children(|parent| {
                // text
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Text Example",
                        TextStyle {
                            font: asset_server.load("fonts/dos.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });
            });
        });

}

struct TimeText;

fn update_time_text(time: Res<Time>, mut query: Query<(&mut Text, &TimeText)>) {
    // Song starts 3 seconds after real time
    let secs = time.seconds_since_startup();

    // Don't do anything before the song starts
    if secs < 0. {
        return;
    }

    for (mut text, _marker) in query.iter_mut() {
        text.sections[0].value = format!("Time: {:.2}", secs);
    }
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_ui.system())
            .add_system(update_time_text.system());
    }
}