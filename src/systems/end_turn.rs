use crate::prelude::*;

pub fn end_turn(
    turn_state: ResMut<State<TurnState>>,
    mut next_state: ResMut<NextState<TurnState>>,
    player_hp_q: Query<(&Health, &Position), With<Player>>,
    amulet_q: Query<&Position, With<AmuletOfYala>>,
    exit_q: Query<&Position, With<ExitTile>>
) {

    let Ok((player_hp, player_pos)) = player_hp_q.single() else {
        panic!("Can't get player_hp and player_pos")
    };
    //let current_state: TurnState = *turn_state.get().clone();
    let current_state: TurnState = turn_state.clone();

    // amulet position if spawned, otherwise set it as -1, -1
    let amulet_default = Position::new_from2d(-1, -1);
    let amulet_pos = amulet_q.iter().nth(0).unwrap_or(&amulet_default);

    // exit position if spawned, otherwise set it as -1, -1
    let exit_default = Position::new_from2d(-1, -1);
    let exit_pos = exit_q.iter().nth(0).unwrap_or(&exit_default);

    // calculate new turn
    let mut new_state = match *turn_state.get() {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        TurnState::StartScreen => return,
        TurnState::NextLevel => TurnState::AwaitingInput,
        _ => current_state
    };

    if player_hp.current < 1 {
        new_state = TurnState::GameOver;
    }

    if player_pos == amulet_pos {
        new_state = TurnState::Victory;
    }

    if player_pos == exit_pos {
        new_state = TurnState::NextLevel;
    }

    // change state to new turn
    next_state.set(new_state);
}