use crate::prelude::*;

mod player_input;
mod camera;
mod collisions;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::new()
                .label("movement")
                .with_system(player_input::player_input.system())
                .with_system(camera::camera_move.system())
                .with_system(collisions::collisions.system())
            );
    }
}