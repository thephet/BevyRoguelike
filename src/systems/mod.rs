use crate::prelude::*;

mod player_input;
mod camera;
mod collisions;
mod random_move;
mod end_turn;
mod movement;
mod mouse;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_state(TurnState::AwaitingInput)
            .add_plugin(AwaitingInputPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(MonsterPlugin);
    }
}

struct AwaitingInputPlugin;
impl Plugin for AwaitingInputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_update(TurnState::AwaitingInput)
                .label("awaiting_input")
                .with_system(mouse::cursor_click.system())
                .with_system(player_input::player_input.system())
                .with_system(camera::camera_move.system())
            );
    }
}


struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(TurnState::PlayerTurn)
                .label("player")
                .with_system(movement::movement.system())
                .with_system(collisions::collisions.system())
                .with_system(end_turn::end_turn.system())
            );
    }
}

struct MonsterPlugin;
impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(TurnState::MonsterTurn)
                .label("monster")
                .with_system(random_move::random_move.system())
                .with_system(movement::movement.system())
                .with_system(end_turn::end_turn.system())
            );
    }
}