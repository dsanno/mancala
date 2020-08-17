use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use crate::board;


pub const VALUE_PER_SEED: isize = 100;

const PATTERN_NUM: usize = 60;
const PATTERN_SIZE: usize = 15 * 15 * 15;
const MAX_SEED: usize = 48;
const SEED_TO_INDEX_0: [usize; MAX_SEED] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
];
const SEED_TO_INDEX_1: [usize; MAX_SEED] = [
    0, 15, 30, 45, 60, 75, 90, 105, 120, 135, 150, 165, 180, 195, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210,
    210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210, 210,
    210,
];
const SEED_TO_INDEX_2: [usize; MAX_SEED] = [
    0, 225, 450, 675, 900, 1125, 1350, 1575, 1800, 2025, 2250, 2475, 2700, 2925, 3150, 3150, 3150, 3150, 3150, 3150,
    3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150,
    3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150, 3150,
];


pub struct Evaluator {
    pattern_values: Vec<Vec<i32>>,
}


impl Evaluator {
    pub fn new() -> Evaluator {
        let mut evaluator = Evaluator {
            pattern_values: Vec::with_capacity(PATTERN_NUM),
        };
        for _ in 0..PATTERN_NUM {
            evaluator.pattern_values.push(vec![0; PATTERN_SIZE]);
        }
        evaluator
    }

    pub fn load(file_path: &str) -> std::io::Result<Evaluator> {
        let mut evaluator = Evaluator::new();
        let mut reader = BufReader::new(File::open(file_path)?);
        let mut buffer = [0; 4];
        for i in 0..PATTERN_NUM {
            for j in 0..PATTERN_SIZE {
                reader.read_exact(&mut buffer)?;
                evaluator.pattern_values[i][j] = i32::from_le_bytes(buffer);
            }
        }
        Ok(evaluator)
    }

    pub fn evaluate(&self, board: &board::Board) -> isize{
        let turn = board.turn();
        let opponent = board::opponent_turn(turn);
        let seed_states = board.seed_states();
        let seeds = seed_states[turn as usize].to_le_bytes();
        let opponent_seeds = seed_states[opponent as usize].to_le_bytes();

        let mut value = 0;
        value += self.pattern_values[0][SEED_TO_INDEX_0[seeds[0] as usize] + SEED_TO_INDEX_1[seeds[1] as usize] + SEED_TO_INDEX_2[opponent_seeds[5] as usize]];
        value += self.pattern_values[1][SEED_TO_INDEX_0[seeds[0] as usize] + SEED_TO_INDEX_1[seeds[1] as usize] + SEED_TO_INDEX_2[opponent_seeds[4] as usize]];
        value += self.pattern_values[2][SEED_TO_INDEX_0[seeds[0] as usize] + SEED_TO_INDEX_1[opponent_seeds[5] as usize] + SEED_TO_INDEX_2[opponent_seeds[4] as usize]];
        value += self.pattern_values[3][SEED_TO_INDEX_0[seeds[1] as usize] + SEED_TO_INDEX_1[opponent_seeds[5] as usize] + SEED_TO_INDEX_2[opponent_seeds[4] as usize]];

        value += self.pattern_values[4][SEED_TO_INDEX_0[seeds[0] as usize] + SEED_TO_INDEX_1[seeds[2] as usize] + SEED_TO_INDEX_2[opponent_seeds[5] as usize]];
        value += self.pattern_values[5][SEED_TO_INDEX_0[seeds[0] as usize] + SEED_TO_INDEX_1[seeds[2] as usize] + SEED_TO_INDEX_2[opponent_seeds[3] as usize]];
        value += self.pattern_values[6][SEED_TO_INDEX_0[seeds[0] as usize] + SEED_TO_INDEX_1[opponent_seeds[5] as usize] + SEED_TO_INDEX_2[opponent_seeds[3] as usize]];
        value += self.pattern_values[7][SEED_TO_INDEX_0[seeds[2] as usize] + SEED_TO_INDEX_1[opponent_seeds[5] as usize] + SEED_TO_INDEX_2[opponent_seeds[3] as usize]];

        value += self.pattern_values[8][SEED_TO_INDEX_0[seeds[0] as usize] + SEED_TO_INDEX_1[seeds[3] as usize] + SEED_TO_INDEX_2[opponent_seeds[5] as usize]];
        value += self.pattern_values[9][SEED_TO_INDEX_0[seeds[0] as usize] + SEED_TO_INDEX_1[seeds[3] as usize] + SEED_TO_INDEX_2[opponent_seeds[2] as usize]];
        value += self.pattern_values[10][SEED_TO_INDEX_0[seeds[0] as usize] + SEED_TO_INDEX_1[opponent_seeds[5] as usize] + SEED_TO_INDEX_2[opponent_seeds[2] as usize]];
        value += self.pattern_values[11][SEED_TO_INDEX_0[seeds[3] as usize] + SEED_TO_INDEX_1[opponent_seeds[5] as usize] + SEED_TO_INDEX_2[opponent_seeds[2] as usize]];

        value += self.pattern_values[12][SEED_TO_INDEX_0[seeds[0] as usize] + SEED_TO_INDEX_1[seeds[4] as usize] + SEED_TO_INDEX_2[opponent_seeds[5] as usize]];
        value += self.pattern_values[13][SEED_TO_INDEX_0[seeds[0] as usize] + SEED_TO_INDEX_1[seeds[4] as usize] + SEED_TO_INDEX_2[opponent_seeds[1] as usize]];
        value += self.pattern_values[14][SEED_TO_INDEX_0[seeds[0] as usize] + SEED_TO_INDEX_1[opponent_seeds[5] as usize] + SEED_TO_INDEX_2[opponent_seeds[1] as usize]];
        value += self.pattern_values[15][SEED_TO_INDEX_0[seeds[4] as usize] + SEED_TO_INDEX_1[opponent_seeds[5] as usize] + SEED_TO_INDEX_2[opponent_seeds[1] as usize]];

        value += self.pattern_values[16][SEED_TO_INDEX_0[seeds[0] as usize] + SEED_TO_INDEX_1[seeds[5] as usize] + SEED_TO_INDEX_2[opponent_seeds[5] as usize]];
        value += self.pattern_values[17][SEED_TO_INDEX_0[seeds[0] as usize] + SEED_TO_INDEX_1[seeds[5] as usize] + SEED_TO_INDEX_2[opponent_seeds[0] as usize]];
        value += self.pattern_values[18][SEED_TO_INDEX_0[seeds[0] as usize] + SEED_TO_INDEX_1[opponent_seeds[5] as usize] + SEED_TO_INDEX_2[opponent_seeds[0] as usize]];
        value += self.pattern_values[19][SEED_TO_INDEX_0[seeds[5] as usize] + SEED_TO_INDEX_1[opponent_seeds[5] as usize] + SEED_TO_INDEX_2[opponent_seeds[0] as usize]];

        value += self.pattern_values[20][SEED_TO_INDEX_0[seeds[1] as usize] + SEED_TO_INDEX_1[seeds[2] as usize] + SEED_TO_INDEX_2[opponent_seeds[4] as usize]];
        value += self.pattern_values[21][SEED_TO_INDEX_0[seeds[1] as usize] + SEED_TO_INDEX_1[seeds[2] as usize] + SEED_TO_INDEX_2[opponent_seeds[3] as usize]];
        value += self.pattern_values[22][SEED_TO_INDEX_0[seeds[1] as usize] + SEED_TO_INDEX_1[opponent_seeds[4] as usize] + SEED_TO_INDEX_2[opponent_seeds[3] as usize]];
        value += self.pattern_values[23][SEED_TO_INDEX_0[seeds[2] as usize] + SEED_TO_INDEX_1[opponent_seeds[4] as usize] + SEED_TO_INDEX_2[opponent_seeds[3] as usize]];

        value += self.pattern_values[24][SEED_TO_INDEX_0[seeds[1] as usize] + SEED_TO_INDEX_1[seeds[3] as usize] + SEED_TO_INDEX_2[opponent_seeds[4] as usize]];
        value += self.pattern_values[25][SEED_TO_INDEX_0[seeds[1] as usize] + SEED_TO_INDEX_1[seeds[3] as usize] + SEED_TO_INDEX_2[opponent_seeds[2] as usize]];
        value += self.pattern_values[26][SEED_TO_INDEX_0[seeds[1] as usize] + SEED_TO_INDEX_1[opponent_seeds[4] as usize] + SEED_TO_INDEX_2[opponent_seeds[2] as usize]];
        value += self.pattern_values[27][SEED_TO_INDEX_0[seeds[3] as usize] + SEED_TO_INDEX_1[opponent_seeds[4] as usize] + SEED_TO_INDEX_2[opponent_seeds[2] as usize]];

        value += self.pattern_values[28][SEED_TO_INDEX_0[seeds[1] as usize] + SEED_TO_INDEX_1[seeds[4] as usize] + SEED_TO_INDEX_2[opponent_seeds[4] as usize]];
        value += self.pattern_values[29][SEED_TO_INDEX_0[seeds[1] as usize] + SEED_TO_INDEX_1[seeds[4] as usize] + SEED_TO_INDEX_2[opponent_seeds[1] as usize]];
        value += self.pattern_values[30][SEED_TO_INDEX_0[seeds[1] as usize] + SEED_TO_INDEX_1[opponent_seeds[4] as usize] + SEED_TO_INDEX_2[opponent_seeds[1] as usize]];
        value += self.pattern_values[31][SEED_TO_INDEX_0[seeds[4] as usize] + SEED_TO_INDEX_1[opponent_seeds[4] as usize] + SEED_TO_INDEX_2[opponent_seeds[1] as usize]];

        value += self.pattern_values[32][SEED_TO_INDEX_0[seeds[1] as usize] + SEED_TO_INDEX_1[seeds[5] as usize] + SEED_TO_INDEX_2[opponent_seeds[4] as usize]];
        value += self.pattern_values[33][SEED_TO_INDEX_0[seeds[1] as usize] + SEED_TO_INDEX_1[seeds[5] as usize] + SEED_TO_INDEX_2[opponent_seeds[0] as usize]];
        value += self.pattern_values[34][SEED_TO_INDEX_0[seeds[1] as usize] + SEED_TO_INDEX_1[opponent_seeds[4] as usize] + SEED_TO_INDEX_2[opponent_seeds[0] as usize]];
        value += self.pattern_values[35][SEED_TO_INDEX_0[seeds[5] as usize] + SEED_TO_INDEX_1[opponent_seeds[4] as usize] + SEED_TO_INDEX_2[opponent_seeds[0] as usize]];

        value += self.pattern_values[36][SEED_TO_INDEX_0[seeds[2] as usize] + SEED_TO_INDEX_1[seeds[3] as usize] + SEED_TO_INDEX_2[opponent_seeds[3] as usize]];
        value += self.pattern_values[37][SEED_TO_INDEX_0[seeds[2] as usize] + SEED_TO_INDEX_1[seeds[3] as usize] + SEED_TO_INDEX_2[opponent_seeds[2] as usize]];
        value += self.pattern_values[38][SEED_TO_INDEX_0[seeds[2] as usize] + SEED_TO_INDEX_1[opponent_seeds[3] as usize] + SEED_TO_INDEX_2[opponent_seeds[2] as usize]];
        value += self.pattern_values[39][SEED_TO_INDEX_0[seeds[3] as usize] + SEED_TO_INDEX_1[opponent_seeds[3] as usize] + SEED_TO_INDEX_2[opponent_seeds[2] as usize]];

        value += self.pattern_values[40][SEED_TO_INDEX_0[seeds[2] as usize] + SEED_TO_INDEX_1[seeds[4] as usize] + SEED_TO_INDEX_2[opponent_seeds[3] as usize]];
        value += self.pattern_values[41][SEED_TO_INDEX_0[seeds[2] as usize] + SEED_TO_INDEX_1[seeds[4] as usize] + SEED_TO_INDEX_2[opponent_seeds[1] as usize]];
        value += self.pattern_values[42][SEED_TO_INDEX_0[seeds[2] as usize] + SEED_TO_INDEX_1[opponent_seeds[3] as usize] + SEED_TO_INDEX_2[opponent_seeds[1] as usize]];
        value += self.pattern_values[43][SEED_TO_INDEX_0[seeds[4] as usize] + SEED_TO_INDEX_1[opponent_seeds[3] as usize] + SEED_TO_INDEX_2[opponent_seeds[1] as usize]];

        value += self.pattern_values[44][SEED_TO_INDEX_0[seeds[2] as usize] + SEED_TO_INDEX_1[seeds[5] as usize] + SEED_TO_INDEX_2[opponent_seeds[3] as usize]];
        value += self.pattern_values[45][SEED_TO_INDEX_0[seeds[2] as usize] + SEED_TO_INDEX_1[seeds[5] as usize] + SEED_TO_INDEX_2[opponent_seeds[0] as usize]];
        value += self.pattern_values[46][SEED_TO_INDEX_0[seeds[2] as usize] + SEED_TO_INDEX_1[opponent_seeds[3] as usize] + SEED_TO_INDEX_2[opponent_seeds[0] as usize]];
        value += self.pattern_values[47][SEED_TO_INDEX_0[seeds[5] as usize] + SEED_TO_INDEX_1[opponent_seeds[3] as usize] + SEED_TO_INDEX_2[opponent_seeds[0] as usize]];

        value += self.pattern_values[48][SEED_TO_INDEX_0[seeds[3] as usize] + SEED_TO_INDEX_1[seeds[4] as usize] + SEED_TO_INDEX_2[opponent_seeds[2] as usize]];
        value += self.pattern_values[49][SEED_TO_INDEX_0[seeds[3] as usize] + SEED_TO_INDEX_1[seeds[4] as usize] + SEED_TO_INDEX_2[opponent_seeds[1] as usize]];
        value += self.pattern_values[50][SEED_TO_INDEX_0[seeds[3] as usize] + SEED_TO_INDEX_1[opponent_seeds[2] as usize] + SEED_TO_INDEX_2[opponent_seeds[1] as usize]];
        value += self.pattern_values[51][SEED_TO_INDEX_0[seeds[4] as usize] + SEED_TO_INDEX_1[opponent_seeds[2] as usize] + SEED_TO_INDEX_2[opponent_seeds[1] as usize]];

        value += self.pattern_values[52][SEED_TO_INDEX_0[seeds[3] as usize] + SEED_TO_INDEX_1[seeds[5] as usize] + SEED_TO_INDEX_2[opponent_seeds[2] as usize]];
        value += self.pattern_values[53][SEED_TO_INDEX_0[seeds[3] as usize] + SEED_TO_INDEX_1[seeds[5] as usize] + SEED_TO_INDEX_2[opponent_seeds[0] as usize]];
        value += self.pattern_values[54][SEED_TO_INDEX_0[seeds[3] as usize] + SEED_TO_INDEX_1[opponent_seeds[2] as usize] + SEED_TO_INDEX_2[opponent_seeds[0] as usize]];
        value += self.pattern_values[55][SEED_TO_INDEX_0[seeds[5] as usize] + SEED_TO_INDEX_1[opponent_seeds[2] as usize] + SEED_TO_INDEX_2[opponent_seeds[0] as usize]];

        value += self.pattern_values[56][SEED_TO_INDEX_0[seeds[4] as usize] + SEED_TO_INDEX_1[seeds[5] as usize] + SEED_TO_INDEX_2[opponent_seeds[1] as usize]];
        value += self.pattern_values[57][SEED_TO_INDEX_0[seeds[4] as usize] + SEED_TO_INDEX_1[seeds[5] as usize] + SEED_TO_INDEX_2[opponent_seeds[0] as usize]];
        value += self.pattern_values[58][SEED_TO_INDEX_0[seeds[4] as usize] + SEED_TO_INDEX_1[opponent_seeds[1] as usize] + SEED_TO_INDEX_2[opponent_seeds[0] as usize]];
        value += self.pattern_values[59][SEED_TO_INDEX_0[seeds[5] as usize] + SEED_TO_INDEX_1[opponent_seeds[1] as usize] + SEED_TO_INDEX_2[opponent_seeds[0] as usize]];

        (board.store(turn) - board.store(opponent)) * VALUE_PER_SEED + value as isize
    }
}