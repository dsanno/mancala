use crate::board;
use super::position_map;
use super::evaluator;


const MIN_VISIT: i32 = 10;
const CONFIDENCE_SCALE: f32 = 32.0 * evaluator::VALUE_PER_SEED as f32;
const MAX_VALUE: isize = 100000;
const EXPLORE_BONUS: isize = 50000;

pub struct Move(pub Option<usize>, pub isize);


pub fn find_best_move(board: &mut board::Board, depth: isize, evaluator: &evaluator::Evaluator,
                      position_map: &position_map::PositionMap, ending: &position_map::PositionMap, explore: bool)
                      -> Move {
    let turn = board.turn();
    let opponent = board::opponent_turn(turn);
    let log_total_size = (position_map.len() as f32).ln();
    let mut best_move: Move = Move(None, 0);
    let mut best_confidence = -MAX_VALUE;
    let upper = MAX_VALUE;
    let mut lower = -MAX_VALUE;
    for i in 0..board::PIT_NUM {
        if board.play(i).is_none() {
            continue;
        }
        let position_map_result = if let Some(position_value) = position_map.get(&board) {
            if !explore && position_value.visit < MIN_VISIT {
                None
            } else {
                let value = if board.turn() == turn {
                    position_value.value
                } else {
                    -position_value.value
                };
                let confidence  = if explore {
                    value + (CONFIDENCE_SCALE * (log_total_size / position_value.visit as f32).sqrt()) as i32
                } else {
                    value
                };
                Some((value as isize, confidence as isize))
            }
        } else {
            None
        };
        let (value, confidence) = if let Some((v, c)) = position_map_result {
            (v, c)
        }  else {
            let v = if board.is_over() {
                (board.store(turn) - board.store(opponent)) * evaluator::VALUE_PER_SEED
            } else if let Some(position_value) = ending.get(&board) {
                if board.turn() == turn {
                    position_value.value as isize
                } else {
                    -position_value.value as isize
                }
            } else if depth == 1 {
                if board.turn() == turn {
                    evaluator.evaluate(&board)
                } else {
                    -evaluator.evaluate(&board)
                }
            } else {
                if board.turn() == turn {
                    let Move(_, v) = search(board, depth - 1, lower, upper, evaluator, ending);
                    v
                } else {
                    let Move(_, v) = search(board, depth - 1, -upper, -lower, evaluator, ending);
                    -v
                }
            };
            let c = if explore {
                v + EXPLORE_BONUS
            } else {
                v
            };
            if v > lower {
                lower = v;
            }
            (v, c)
        };
        if confidence > best_confidence {
            best_move = Move(Some(i), value);
            best_confidence = confidence;
        }
        board.undo();
    }
    best_move
}


fn search(board: &mut board::Board, depth: isize, lower: isize, upper: isize, evaluator: &evaluator::Evaluator,
          ending: &position_map::PositionMap) -> Move {
    let turn = board.turn();
    let opponent = board::opponent_turn(turn);
    let mut max_value = -MAX_VALUE;
    let mut lower_value = lower;
    let mut best_move: Move = Move(None, max_value);
    let moves = if depth >= 4 {
        let mut move_and_values = (0..board::PIT_NUM).filter_map(|i| {
            if let Some(_) = board.play(i) {
                let value = evaluator.evaluate(&board);
                let value = if board.turn() == turn {
                    -value
                } else {
                    value
                };
                board.undo();
                Some((i, value))
            } else {
                None
            }
        }).collect::<Vec<_>>();
        move_and_values.sort_unstable_by(|&(_, a), &(_, b)| a.cmp(&b));
        move_and_values.iter().map(|(pos, _)| *pos).collect::<Vec<_>>()
    } else {
        (0..board::PIT_NUM).rev().collect::<Vec<_>>()
    };
    for i in moves {
        match board.play(i) {
            Some(_) => {
                let value = if board.is_over() {
                    (board.store(turn) - board.store(opponent)) * evaluator::VALUE_PER_SEED
                } else if depth == 1 {
                    let v = evaluator.evaluate(&board);
                    if board.turn() == turn {
                        v
                    } else {
                        -v
                    }
                } else if let Some(position_value) = ending.get(&board) {
                    if board.turn() == turn {
                        position_value.value as isize
                    } else {
                        -position_value.value as isize
                    }
                } else {
                    if board.turn() == turn {
                        let Move(_, v) = search(board, depth - 1, lower_value, upper, evaluator, ending);
                        v
                    } else {
                        let Move(_, v) = search(board, depth - 1, -upper, -lower_value, evaluator, ending);
                        -v
                    }
                };
                board.undo();
                if value > max_value {
                    best_move = Move(Some(i), value);
                    max_value = value;
                    if value >= upper {
                        return best_move;
                    } else if max_value > lower_value {
                        lower_value = max_value;
                    }
                }
            },
            _ => (),
        }
    }
    best_move
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
                position_value.value as isize
            } else {
                -position_value.value as isize
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
