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
    pub use bevy::winit::WinitSettings;
    pub use bevy::window::PrimaryWindow;
    pub use bracket_lib::prelude::*;
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Setup the sprite sheet
    let texture_handle = asset_server.load("terminal8x8_transparent.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 16, 16, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // add sprite atlas as resource
    commands.insert_resource(CharsetAsset { atlas: texture_atlas_handle.clone() });
    
    // Add a 2D Camera
    let mut cam = Camera2dBundle::default();
    cam.transform.scale = Vec3::new(0.5, 0.5, 1.0);
    commands.spawn((MainCamera, cam));
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
        .add_state::<TurnState>()
        .add_state::<PopUpState>()
        //.insert_resource(WinitSettings::desktop_app())
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        .add_plugins(MapPlugin)
        .add_plugins(SpawnerPlugin)
        .add_plugins(SystemsPlugin)
        .add_plugins(UIPlugin)
        .add_systems(PostUpdate, (position_translation, size_scaling))
        .run();
}