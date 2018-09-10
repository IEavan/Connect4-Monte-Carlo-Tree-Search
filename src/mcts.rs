// Monte Carlo Tree Search Functions

use grid::*;
use rand::prelude::*;

enum Finish {
    Win,
    Lose,
    Draw
}

pub fn get_best_move(move_list: Vec<GameMove>, rollouts: u32) -> GameMove {
    if move_list.len() == 0 {panic!("No moves to select best from");}
    move_list.into_iter()
             .map(|x| (estimate_win_prob(&x.next, rollouts), x))
             .fold((2.0, GameMove {previous: GridState::new(), next: GridState::new()}),
                |old, new| if old.0.gt(&new.0) {new} else {old}).1
}

pub fn estimate_win_prob(current_state: &GridState, rollouts: u32) -> f64 {
    let mut wins = 0;
    let mut losses = 0;
    let mut _draws = 0;
    for _i in 0..rollouts {
        match rollout(&current_state) {
            Finish::Win => wins += 1,
            Finish::Lose => losses += 1,
            Finish::Draw => _draws += 1
        }
    }
    (wins - losses) as f64 / 100.0
}

fn rollout(start_state: &GridState) -> Finish {
    let mut rng = thread_rng();

    let mut current_state = start_state.clone();
    loop {
        let moves = current_state.get_moves();
        if moves.len() == 0 {
            break;
        } else {
            let random_move = rng.choose(&moves).unwrap();
            current_state = random_move.next.clone()
        }
    }
    match current_state.winner {
        None => Finish::Draw,
        Some(x) => if x == start_state.turn {Finish::Win} else {Finish::Lose}
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_win_is_best_move() {
        let mut test_grid = GridState::new();
        test_grid.insert(0).unwrap();
        test_grid.insert(8).unwrap();
        test_grid.insert(1).unwrap();
        test_grid.insert(8).unwrap();
        test_grid.insert(2).unwrap();
        test_grid.insert(8).unwrap();
        let best_move = get_best_move(test_grid.get_moves(), 10);
        assert_eq!(best_move.next.winner, Some(Player::Yellow));
    }
}
