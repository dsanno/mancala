use crate::board;
use super::position_map;


pub struct Move(pub Option<usize>, pub isize);


pub fn search(board: &mut board::Board, depth: isize, lower: isize, upper: isize) -> Move {
    let turn = board.turn();
    let opponent = board::opponent_turn(turn);
    let mut max_value = isize::MIN;
    let mut lower_value = lower;
    let mut best_move: Move = Move(None, max_value);
    for i in 0..board::PIT_NUM {
        match board.play(i) {
            Some(_) => {
                let value = if board.is_over() {
                    board.store(turn) - board.store(opponent)
                } else if depth == 1 {
                    board.store(turn) - board.store(opponent)
                } else {
                    if board.turn() == turn {
                        let Move(_, v) = search(board, depth - 1, lower_value, upper);
                        v
                    } else {
                        let Move(_, v) = search(board, depth - 1, -upper, -lower_value);
                        -v
                    }
                };
                board.undo();
                if value > max_value {
                    best_move = Move(Some(i), value);
                    max_value = value;
                    if max_value > lower_value {
                        lower_value = max_value;
                    }
                }
            },
            _ => (),
        }
    }
    best_move
}


const CONFIDENCE_SCALE: f32 = 32.0;
const MAX_VALUE: i32 = 10000;

pub fn search_position_map(board: &mut board::Board, position_map: &position_map::PositionMap, explore: bool) -> Move {
    let turn = board.turn();
    if position_map.get(&board).is_none() {
        return Move(None, 0);
    }
    let log_total_size = (position_map.len() as f32).ln();
    let mut best_move: Option<usize> = None;
    let mut best_value = -MAX_VALUE as f32;
    for i in 0..board::PIT_NUM {
        match board.play(i) {
            Some(_) => (),
            _ => continue,
        }
        let value = match position_map.get(&board) {
            Some(position_value) => {
                let value = if board.turn() == turn {
                    position_value.value
                } else {
                    -position_value.value
                };
                if explore {
                    value as f32 + CONFIDENCE_SCALE * (log_total_size / position_value.visit as f32).sqrt()
                } else {
                    value as f32
                }
            },
            _ => if explore {
                MAX_VALUE as f32
            } else {
                -MAX_VALUE as f32
            },
        };
        if value > best_value {
            best_value = value;
            best_move = Some(i);
        }
        board.undo();
    }
    Move(best_move, best_value as isize)
}

pub fn register_position_map(board: &mut board::Board, position_map: &mut position_map::PositionMap, value: isize) -> () {
    let turn = board.turn();

    let mut best_move: Option<usize> = None;
    let mut best_value = -MAX_VALUE;
    for i in 0..board::PIT_NUM {
        match board.play(i) {
            Some(_) => (),
            _ => continue,
        }
        if let Some(position_value) = position_map.get(&board) {
            let child_value = if board.turn() == turn {
                position_value.value
            } else {
                -position_value.value
            };
            if child_value > best_value {
                best_value = child_value;
                best_move = Some(i);
            }
        }
        board.undo();
    }
    if let Some(_) = best_move {
        position_map.insert(&board, best_value as isize);
    } else {
        position_map.insert(&board, value);
    }
}
