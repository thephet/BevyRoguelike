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

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(AwaitingInputPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(MonsterPlugin);
    }
}

struct AwaitingInputPlugin;
impl Plugin for AwaitingInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(TurnState::AwaitingInput)
                .label("awaiting_input")
                .with_system(player_input::player_input)
                .with_system(camera::camera_move)
                .with_system(fov::fov)
                .with_system(update_entities_visibility::update_entities_visibility)
            );
    }
}

struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(TurnState::PlayerTurn)
                .label("player")
                .with_system(use_items::use_items)
                .with_system(combat::combat)
                .with_system(movement::movement)
                .with_system(fov::fov)
                .with_system(end_turn::end_turn.label("player_end"))
            );
    }
}

struct MonsterPlugin;
impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_stage_after(CoreStage::Update, "enemies_state", SystemStage::parallel())
            .add_system_set(
                SystemSet::on_enter(TurnState::MonsterTurn)
                .label("enemies")
                .with_system(chasing::chasing.after("player_end").label("chasing"))
                .with_system(combat::combat.after("chasing").label("combat"))
                .with_system(movement::movement.after("combat").label("enemies_move"))
                .with_system(fov::fov.after("enemies_move").label("enemies_fov"))
                .with_system(end_turn::end_turn.after("enemies_fov"))
            );
    }
}