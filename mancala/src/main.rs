#[macro_use]
extern crate clap;
use clap::App;
use clap::AppSettings;
use clap::Arg;
use clap::SubCommand;

extern crate mancala;
use mancala::board;
use mancala::engine::com;
use mancala::engine::evaluator;
use mancala::engine::position_map;

const EVALUATION_FILE_PATH: &str = "eval.dat";
const POSITION_FILE_PATH: &str = "position.dat";

const SUB_COMMAND_PLAY: &str = "play";
const ABOUT_PLAY: &str = "Play with computer";

const SUB_COMMAND_SELF_PLAY: &str = "self";
const ABOUT_SELF_PLAY: &str = "Computer starts self playing";

const SUB_COMMAND_EVALUATE: &str = "eval";
const ABOUT_EVALUATE: &str = "Evaluate moves";

const ARG_DEPTH: &str = "depth";
const HELP_DEPTH: &str = "Depth of com player thinking (2-20)";
const ERROR_DEPTH_TYPE: &str = "Depth must be an integer";
const ERROR_DEPTH_RANGE: &str = "Depth must be in the range of 2 to 20";
const MIN_DEPTH: isize = 2;
const MAX_DEPTH: isize = 20;

const ARG_SELF_PLAY_NUM: &str = "self_play_num";
const HELP_SELF_PLAY_NUM: &str = "Number of self play";
const ERROR_SELF_PLAY_NUM_TYPE: &str = "Number of self play must be an integer";

const ARG_EVALUATE_DEPTH: &str = "evaluation_depth";
const HELP_EVALUATE_DEPTH: &str = "Depth of evaluation (2-20)";

enum Command {
    Undo,
    Quit,
    Number(usize),
}

fn main() {
    let play_command = SubCommand::with_name(SUB_COMMAND_PLAY)
        .about(ABOUT_PLAY)
        .arg(
            Arg::with_name(ARG_DEPTH)
                .help(HELP_DEPTH)
                .required(true)
        );
    let self_command = SubCommand::with_name(SUB_COMMAND_SELF_PLAY)
        .about(ABOUT_SELF_PLAY)
        .arg(
            Arg::with_name(ARG_DEPTH)
                .help(HELP_DEPTH)
                .required(true)
        )
        .arg(
            Arg::with_name(ARG_SELF_PLAY_NUM)
                .help(HELP_SELF_PLAY_NUM)
                .required(true)
        );
    let evaluate_command = SubCommand::with_name(SUB_COMMAND_EVALUATE)
        .about(ABOUT_EVALUATE)
        .arg(
            Arg::with_name(ARG_EVALUATE_DEPTH)
                .help(HELP_EVALUATE_DEPTH)
                .required(true)
        );
let app = App::new(crate_name!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(play_command)
        .subcommand(self_command)
        .subcommand(evaluate_command);

    let matches = app.get_matches();
    if let Some(ref matches) = matches.subcommand_matches(SUB_COMMAND_PLAY) {
        let depth = value_t!(matches.value_of(ARG_DEPTH), isize).unwrap_or_else(|e| {
            println!("{}", ERROR_DEPTH_TYPE);
            e.exit();
        });
        if depth < MIN_DEPTH || depth > MAX_DEPTH {
            println!("{}", ERROR_DEPTH_RANGE);
            std::process::exit(1);
        }
        play(depth);
    } else if let Some(ref matches) = matches.subcommand_matches(SUB_COMMAND_SELF_PLAY) {
        let depth = value_t!(matches.value_of(ARG_DEPTH), isize).unwrap_or_else(|e| {
            println!("{}", ERROR_DEPTH_TYPE);
            e.exit();
        });
        let self_play_num = value_t!(matches.value_of(ARG_SELF_PLAY_NUM), usize).unwrap_or_else(|e| {
            println!("{}", ERROR_SELF_PLAY_NUM_TYPE);
            e.exit();
        });
        if depth < MIN_DEPTH || depth > MAX_DEPTH {
            println!("{}", ERROR_DEPTH_RANGE);
            std::process::exit(1);
        }
        self_play(self_play_num, depth);
    } else if let Some(ref matches) = matches.subcommand_matches(SUB_COMMAND_EVALUATE) {
        let depth = value_t!(matches.value_of(ARG_EVALUATE_DEPTH), isize).unwrap_or_else(|e| {
            println!("{}", ERROR_DEPTH_TYPE);
            e.exit();
        });
        if depth < MIN_DEPTH || depth > MAX_DEPTH {
            println!("{}", ERROR_DEPTH_RANGE);
            std::process::exit(1);
        }
        evaluate(depth);
    }
}

fn play(depth: isize) {
    let position_map = match position_map::PositionMap::load(POSITION_FILE_PATH) {
        Ok(position_map) => position_map,
        Err(_) => {
            println!("Warning: cannot load position file");
            Default::default()
        },
    };
    let evaluator = match evaluator::Evaluator::load(EVALUATION_FILE_PATH) {
        Ok(evaluator) => evaluator,
        Err(_) => {
            println!("Warning: cannot load position file");
            evaluator::Evaluator::new()
        },
    };
    loop {
        let mut b: board::Board = Default::default();
        let player_turn;
        loop {
            println!("Input 1 for first player or 2 for second player");
            match read_command() {
                Some(Command::Number(1)) => {
                    player_turn = board::Turn::First;
                    break;
                },
                Some(Command::Number(2)) => {
                    player_turn = board::Turn::Second;
                    break;
                },
                _ => {
                    println!("Invalid input");
                },
            }
        }
        loop {
            while b.turn() == player_turn && !b.is_over() {
                print_board(&b, player_turn);
                println!("Your turn, input 1-{}", board::PIT_NUM);
                match read_command() {
                    Some(Command::Quit) => {
                        println!("Thank you for playing");
                        return ();
                    },
                    Some(Command::Undo) => {
                        b.undo();
                        while b.turn() != player_turn {
                            b.undo();
                        }
                    },
                    Some(Command::Number(n)) => {
                        if  n >= board::PIT_NUM + 1 {
                            println!("{} is invalid number for move", n);
                        } else if let None = b.play(n - 1) {
                            println!("{} is invalid move", n);
                        }
                    },
                    _ => println!("Invalid command"),
                }
            }

            while b.turn() != player_turn && !b.is_over() {
                print_board(&b, player_turn);
                println!("Com is thinking");
                if let com::Move(Some(pos), value) = com::find_best_move(&mut b, depth, &evaluator, &position_map, false) {
                    println!("Com plays {}, value is {}", pos + 1, value);
                    if let None = b.play(pos) {
                        println!("Unknown error");
                        std::process::exit(1);
                    }
                } else {
                    println!("Unknown error");
                    std::process::exit(1);

                }
            }
            if b.is_over() {
                print_board(&b, player_turn);
                let opponent = board::opponent_turn(player_turn);
                if b.store(player_turn) > b.store(opponent) {
                    println!("You win!");
                } else if b.store(opponent) > b.store(player_turn) {
                    println!("Com win!");
                } else {
                    println!("Draw!");
                }
                break;
            }
        }
    }
}

fn evaluate(depth: isize) {
    let position_map = match position_map::PositionMap::load(POSITION_FILE_PATH) {
        Ok(position_map) => position_map,
        Err(_) => {
            println!("Warning: cannot load position file");
            Default::default()
        },
    };
    let evaluator = match evaluator::Evaluator::load(EVALUATION_FILE_PATH) {
        Ok(evaluator) => evaluator,
        Err(_) => {
            println!("Warning: cannot load position file");
            evaluator::Evaluator::new()
        },
    };
    loop {
        let mut b: board::Board = Default::default();
        loop {
            print_board(&b, board::Turn::First);
            if !b.is_over() {
                let turn = b.turn();
                println!("Move values:");
                for pos in 0..board::PIT_NUM {
                    if let None = b.play(pos) {
                        continue;
                    }
                    if let com::Move(Some(_), value) = com::find_best_move(&mut b, depth - 1, &evaluator, &position_map, false) {
                        if b.turn() == turn {
                            println!("{}: {}", pos + 1, value)
                        } else {
                            println!("{}: {}", pos + 1, -value)
                        }
                    }
                    b.undo();
                }
            }
            if b.turn() == board::Turn::First {
                println!("South to play, input 1-{}", board::PIT_NUM);
            } else {
                println!("North to play, input 1-{}", board::PIT_NUM);
            }
            match read_command() {
                Some(Command::Quit) => {
                    println!("Thank you for playing");
                    return ();
                },
                Some(Command::Undo) => {
                    b.undo();
                },
                Some(Command::Number(n)) => {
                    if  n >= board::PIT_NUM + 1 {
                        println!("{} is invalid number for move", n);
                    } else if let None = b.play(n - 1) {
                        println!("{} is invalid move", n);
                    }
                },
                _ => println!("Invalid command"),
            }
        }
    }
}

fn self_play(num: usize, depth: isize) {
    let mut position_map = match position_map::PositionMap::load(POSITION_FILE_PATH) {
        Ok(position_map) => position_map,
        Err(_) => Default::default(),
    };
    let evaluator = match evaluator::Evaluator::load(EVALUATION_FILE_PATH) {
        Ok(evaluator) => evaluator,
        Err(_) => {
            println!("Warning: cannot load position file");
            evaluator::Evaluator::new()
        },
    };
    for i in 0..num {
        let mut b: board::Board = Default::default();
        let mut move_stack: Vec<usize> = Vec::new();
        while !b.is_over() {
            if let com::Move(Some(pos), _) = com::find_best_move(&mut b, depth, &evaluator, &position_map, true) {
                b.play(pos);
                move_stack.push(pos);
            } else {
                println!("Unknown error");
                std::process::exit(1);
            }
        }
        let value = (b.store(board::Turn::First) - b.store(board::Turn::Second)) * evaluator::VALUE_PER_SEED;
        loop {
            if let None = b.undo() {
                break;
            }
            let previous_move = move_stack.pop().unwrap();
            if let Some(_) = position_map.get(&b) {
                b.play(previous_move);
                if b.turn() == board::Turn::First {
                    position_map.insert(&b, value);
                } else {
                    position_map.insert(&b, -value);
                }
                b.undo();
                break;
            }
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
        if (i + 1) % 1000 == 0 {
            println!("{} games done", i + 1);
        }
        if (i + 1) % 10000 == 0 {
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

fn read_command() -> Option<Command> {
    let mut line = String::new();
    if let Err(_) = std::io::stdin().read_line(&mut line) {
        return None;
    }
    match line.trim() {
        "q" => Some(Command::Quit),
        "quit" => Some(Command::Quit),
        "u" => Some(Command::Undo),
        "undo" => Some(Command::Undo),
        s => match s.parse() {
            Ok(n) => Some(Command::Number(n)),
            _ => None,
        }
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
