extern crate mancala;
use mancala::board;
use mancala::engine::com;
use mancala::engine::position_map;


const POSITION_FILE_PATH: &str = "position.dat";

fn main() {
    self_play(10000);
//    play();
}

fn play() {
    let depth = 12;
    let max_score = 48;
    let position_map = match position_map::PositionMap::load(POSITION_FILE_PATH) {
        Ok(position_map) => position_map,
        Err(_) => Default::default(),
    };
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
                        } else if let None = b.play(n - 1) {
                            println!("{} is invalid move", n);
                        }
                    },
                    Err(message) => {
                        println!("{}", message);
                    },
                }
            }

            while b.turn() != player_turn && !b.is_over() {
                print_board(&b, player_turn);
                let (pos, value) = if let com::Move(Some(pos), value) = com::search_position_map(&mut b, &position_map, false) {
                    (pos, value)
                } else if let com::Move(Some(pos), value) = com::search(&mut b, depth, -max_score, max_score) {
                    (pos, value)
                } else {
                    println!("Unknown error");
                    std::process::exit(1);
                };
                println!("Com plays {}, value is {}", pos + 1, value);
                if let None = b.play(pos) {
                    println!("Unknown error");
                    std::process::exit(1);
                }
            }
            if b.is_over() {
                break;
            }
        }
    }
}

fn self_play(num: isize) {
    let depth = 10;
    let max_score = 48;
    let mut position_map = match position_map::PositionMap::load(POSITION_FILE_PATH) {
        Ok(position_map) => position_map,
        Err(_) => Default::default(),
    };
    for i in 0..num {
        let mut b: board::Board = Default::default();
        while !b.is_over() {
            let pos = if let com::Move(Some(pos), _) = com::search_position_map(&mut b, &position_map, true) {
                pos
            } else if let com::Move(Some(pos), _) = com::search(&mut b, depth, -max_score, max_score) {
                pos
            } else {
                println!("Unknown error");
                std::process::exit(1);
            };
            b.play(pos);
        }
        let value = b.store(board::Turn::First) - b.store(board::Turn::Second);
        for _ in 0..6 {
            b.undo();
        }
        loop {
            if b.turn() == board::Turn::First {
                com::register_position_map(&mut b, &mut position_map, value);
            } else {
                com::register_position_map(&mut b, &mut position_map, -value);
            }
            if let None = b.undo() {
                break
            }
        }
        if (i + 1) % 100 == 0 {
            println!("{} games done", i + 1);
        }
        if (i + 1) % 100 == 0 {
            match position_map.save(POSITION_FILE_PATH) {
                Ok(_) => (),
                Err(_) => println!("Cannot save position map"),
            }
        }
    }
    match position_map.save(POSITION_FILE_PATH) {
        Ok(_) => (),
        Err(_) => println!("Cannot save position map"),
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
    for _ in 0..(board::PIT_NUM - 1) {
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
