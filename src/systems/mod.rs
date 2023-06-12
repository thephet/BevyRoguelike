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
            .add_plugin(player_input::PlayerInputPlugin)

            .add_systems(
                (
                    camera::camera_move,
                    fov::fov,
                    update_entities_visibility::update_entities_visibility
                ).in_set(OnUpdate(TurnState::AwaitingInput))
            );
    }
}

struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app

        .add_systems(
            (
                use_items::use_items,
                combat::combat,
                movement::movement,
                fov::fov,
                end_turn::end_turn
            ).in_set(OnUpdate(TurnState::PlayerTurn))
        );
    }
}

struct MonsterPlugin;
impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app

        .add_systems(
            (
                chasing::chasing,
                combat::combat,
                movement::movement,
                fov::fov,
                end_turn::end_turn
            ).in_set(OnUpdate(TurnState::PlayerTurn))
        );
    }
}

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(AwaitingInputPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(MonsterPlugin);
    }
}