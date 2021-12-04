#![warn(clippy::pedantic)]

mod map;
mod components;
mod renderutils;

mod prelude {
    pub use bevy::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::components::*;
    pub use crate::renderutils::*;
    // resource type
    pub struct CharsetAsset {
        pub atlas: Handle<TextureAtlas>,
    }
}

use prelude::*;
use bevy::render::pass::ClearColor;


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Setup the sprite sheet
    let texture_handle = asset_server.load("terminal8x8.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // add sprite atlas as resource
    commands.insert_resource(CharsetAsset { atlas: texture_atlas_handle.clone() });

    // Add a 2D Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // insert map as resource
    commands.insert_resource(Map::new());
}


fn main() {

    App::build()
        .insert_resource(WindowDescriptor {
            title: "Roguelike Game".to_string(),
            width: 80.0 * 10.0,
            height: 50.0 * 10.0,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage("map_spawn", SystemStage::single(spawn_map_tiles.system()))
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation.system())
                .with_system(size_scaling.system()),
        )
        .run();
}