#![warn(clippy::pedantic)]

mod map_builder;
mod components;
mod resources;
mod render_utils;
mod spawner;
mod systems;
mod utils;
mod ui;

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy::window::PrimaryWindow;
    pub use bracket_lib::geometry::Point;
    pub use bracket_lib::terminal::DistanceAlg;
    pub use bracket_lib::pathfinding::DijkstraMap;
    pub use bracket_lib::pathfinding::Algorithm2D;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 80;
    pub const UI_HEIGHT: i32 = 10;
    pub use rand::Rng;
    pub use crate::map_builder::*;
    pub use crate::components::*;
    pub use crate::resources::*;
    pub use crate::render_utils::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::utils::*;
    pub use crate::ui::*;
}

use prelude::*;

fn initial_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Setup the sprite sheet
    let texture_handle: Handle<Image> = asset_server.load("terminal8x8_transparent.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(8, 8), 16, 16, None, None);
    let layout_handle = atlases.add(layout);
    // add sprite atlas as resource
    commands.insert_resource(CharsetAsset { atlas: layout_handle.clone(), texture: texture_handle.clone() });
    
    // Add a 2D Camera
    commands.spawn((
        MainCamera,
        Camera2d,
        Transform::from_scale(Vec3::new(0.5, 0.5, 1.0)),
    ));
}

fn transition_to_in_menu(mut app_state: ResMut<NextState<TurnState>>) {
    app_state.set(TurnState::StartScreen);
}

fn main() {

    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Roguelike Game".to_string(),
                    resolution: (SCREEN_WIDTH as f32 * 10.0, SCREEN_HEIGHT as f32 * 10.0).into(),
                    ..Default::default()
                }),
                ..Default::default()
            }))
        .init_state::<TurnState>()
        .init_state::<PopUpState>()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        //.add_systems(Startup, setup)
        .add_systems(OnEnter(TurnState::Setup), initial_setup)
        .add_systems(Update, transition_to_in_menu.run_if(in_state(TurnState::Setup)))
        .add_plugins(MapPlugin)
        .add_plugins(SpawnerPlugin)
        .add_plugins(SystemsPlugin)
        .add_plugins(UIPlugin)
        .add_systems(PostUpdate, (position_translation, size_scaling))
        .run();
}