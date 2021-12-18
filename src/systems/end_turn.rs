use crate::prelude::*;

pub fn end_turn(
    mut turn_state: ResMut<State<TurnState>>
) {
    // calculate new turn
    let new_state = match turn_state.current() {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
    };

    // change state to new turn
    turn_state.set(new_state).unwrap();
}