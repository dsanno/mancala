use crate::board;


pub struct Move(pub Option<usize>, pub isize);


pub fn search(board: &mut board::Board, depth: isize, lower: isize, upper: isize) -> Move {
    let turn = board.turn();
    let opponent = board::opponent_turn(turn);
    let mut max_value = isize::MIN;
    let mut lower_value = lower;
    let mut best_move: Move = Move(None, max_value);
    for i in 0..board::PIT_NUM {
        match board.play(i) {
            Ok(_) => {
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
