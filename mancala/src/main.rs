extern crate mancala;
use mancala::board;
use mancala::engine::com;


fn main() {
    let depth = 10;
    let max_score = 48;
    loop {
        let mut b: board::Board = Default::default();
        let player_turn = board::Turn::First;

        loop {
            while b.turn() == player_turn && !b.is_over() {
                print_board(&b, player_turn);
                println!("Your turn");
                match read_number() {
                    Ok(n) => {
                        if n == 0 {
                            b.undo();
                            while b.turn() != player_turn {
                                b.undo();
                            }
                        } else if n >= board::PIT_NUM + 1 {
                            println!("{} is invalid number for move", n);
                        } else {
                            match b.play(n - 1) {
                                Ok(_) => (),
                                Err(_) => println!("{} is invalid move", n),
                            }
                        }
                    },
                    Err(message) => {
                        println!("{}", message);
                    },
                }
            }

            while b.turn() != player_turn && !b.is_over() {
                print_board(&b, player_turn);
                let com::Move(next_move, value) = com::search(&mut b, depth, -max_score, max_score);
                match next_move {
                    Some(pos) => {
                        println!("Com plays {}, value is {}", pos + 1, value);
                        match b.play(pos) {
                            Ok(_) => (),
                            Err(_) => {
                                println!("Unknown error");
                                std::process::exit(1);
                            },
                        }
                    },
                    _ => {
                        println!("Unknown error");
                        std::process::exit(1);
                    },
                }
            }
            if b.is_over() {
                break;
            }
        }
    }
}

fn read_number() -> Result<usize, String> {
    let mut line = String::new();
    if let Err(_) = std::io::stdin().read_line(&mut line) {
        return Err(String::from("Cannot read line"));
    }
    match line.trim().parse() {
        Ok(n) => Ok(n),
        Err(_) => Err(String::from("Input is not a number")),
    }
}


fn print_board(b: &board::Board, front: board::Turn) {
    let back = board::opponent_turn(front);
    println!("---------------------------");
    println!("     (6)(5)(4)(3)(2)(1)");
    println!("");
    print!("    |");
    for i in (0..board::PIT_NUM).rev() {
        print!("{:>2}|", b.seed(back, i));
    }
    println!("");
    print!(" {:>2} |", b.store(back));
    for i in 0..(board::PIT_NUM - 1) {
        print!("   ");
    }
    println!("  | {:>2}", b.store(front));
    print!("    |");
    for i in 0..board::PIT_NUM {
        print!("{:>2}|", b.seed(front, i));
    }
    println!("");
    println!("");
    println!("     (1)(2)(3)(4)(5)(6)");
    println!("---------------------------");
}
