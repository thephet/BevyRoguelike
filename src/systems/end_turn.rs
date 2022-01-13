use crate::prelude::*;

pub fn end_turn(
    mut turn_state: ResMut<State<TurnState>>,
    player_hp_q: Query<(&Health, &Position), With<Player>>,
    amulet_q: Query<&Position, With<AmuletOfYala>>
) {

    let (player_hp, player_pos) = player_hp_q.single();
    let amulet_pos = amulet_q.single();
    let current_state = turn_state.current().clone();

    // calculate new turn
    let mut new_state = match turn_state.current() {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        TurnState::StartScreen => return,
        _ => current_state
    };

    if player_hp.current < 1 {
        new_state = TurnState::GameOver;
    }

    if player_pos == amulet_pos {
        new_state = TurnState::Victory;
    }

    // change state to new turn
    turn_state.set(new_state).unwrap();
}