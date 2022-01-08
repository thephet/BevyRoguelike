#![warn(clippy::pedantic)]

mod map;
mod map_builder;
mod components;
mod resources;
mod renderutils;
mod spawner;
mod systems;
mod utils;
mod hud;

mod prelude {
    pub use bevy::prelude::*;
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 80;
    pub const UI_HEIGHT: i32 = 10;
    pub use rand::Rng;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::components::*;
    pub use crate::resources::*;
    pub use crate::renderutils::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::utils::*;
    pub use crate::hud::*;
}

use prelude::*;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Setup the sprite sheet
    let texture_handle = asset_server.load("terminal8x8_transparent.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // add sprite atlas as resource
    commands.insert_resource(CharsetAsset { atlas: texture_atlas_handle.clone() });

    // insert map builder as resource
    let mb = MapBuilder::new();
    commands.insert_resource(mb);
    
    // Add a 2D Camera
    let mut cam = OrthographicCameraBundle::new_2d();
    // cam.orthographic_projection.scale = 0.5;
    cam.transform.scale = Vec3::new(0.5, 0.5, 1.0);
    commands.spawn_bundle(cam)
        .insert(MainCamera);
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
}


fn main() {

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Roguelike Game".to_string(),
            width: SCREEN_WIDTH as f32 * 10.0,
            height: SCREEN_HEIGHT as f32 * 10.0,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_stage("map_spawn", SystemStage::single(spawn_map_tiles))
        .add_startup_stage_after("map_spawn", "player_spawn", SystemStage::single(spawn_player))
        .add_startup_stage_after("map_spawn", "enemies_spawn", SystemStage::single(spawn_enemies))
        .add_plugin(SystemsPlugin)
        .add_plugin(UIPlugin)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .run();
}