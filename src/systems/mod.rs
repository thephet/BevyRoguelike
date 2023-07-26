use crate::prelude::*;

mod player_input;
mod camera;
mod combat;
// mod random_move;
mod chasing;
mod end_turn;
mod movement;
mod fov;
mod update_entities_visibility;
mod use_items;

struct AwaitingInputPlugin;
impl Plugin for AwaitingInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(player_input::PlayerInputPlugin)

            .add_systems(
                Update,
                (
                    fov::fov,
                    update_entities_visibility::update_entities_visibility,
                    camera::camera_move,
                ).run_if(in_state(TurnState::AwaitingInput))
            );
    }
}

struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app

        .add_systems(
            Update,
            (
                use_items::use_items,
                combat::combat,
                movement::movement,
                fov::fov,
                update_entities_visibility::update_entities_visibility,
                camera::camera_move,
                end_turn::end_turn
            ).chain()
            .run_if(in_state(TurnState::PlayerTurn))
        );
    }
}

struct MonsterPlugin;
impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app

        .add_systems(
            Update,
            (
                chasing::chasing,
                combat::combat,
                movement::movement,
                fov::fov,
                end_turn::end_turn
            ).chain()
            .run_if(in_state(TurnState::MonsterTurn))
        );
    }
}

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(AwaitingInputPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(MonsterPlugin);
    }
}