pub const PLAYER_NUM: usize = 2;
pub const PIT_NUM: usize = 6;

const PIT_BIT_NUM: usize = 8;
const PIT_BIT_MASK: i64 = 0xff;
const INITIAL_SEEDS: i64 = 0x040404040404;
const EMPTY_SEEDS: i64 = 0;
const MAX_SEED_NUM: usize = 48;
const HISTORY_SIZE: usize = MAX_SEED_NUM * 3;
const SOW_CYCLE_SIZE: usize = PIT_NUM * 2 + 1;
const SEED_DIFFS: [i64; PIT_NUM * (MAX_SEED_NUM + 1)] = [
    0, 255, 65790, 16843005, 4311810300, 1103823438075, 1103823438074, 1103823438073, 1103823438072, 1103823438071,
    1103823438070, 1103823438069, 1103823438068, 1103823438068, 1103823438323, 1103823503858, 1103840281073,
    1108135248368, 2207646876143, 2207646876142, 2207646876141, 2207646876140, 2207646876139, 2207646876138,
    2207646876137, 2207646876136, 2207646876136, 2207646876391, 2207646941926, 2207663719141, 2211958686436,
    3311470314211, 3311470314210, 3311470314209, 3311470314208, 3311470314207, 3311470314206, 3311470314205,
    3311470314204, 3311470314204, 3311470314459, 3311470379994, 3311487157209, 3315782124504, 4415293752279,
    4415293752278, 4415293752277, 4415293752276, 4415293752275,
    0, 65280, 16842240, 4311809280, 1103823436800, 1103823436544, 1103823436288, 1103823436032, 1103823435776,
    1103823435520, 1103823435264, 1103823435008, 1103823434753, 1103823434753, 1103823500033, 1103840276993,
    1108135244033, 2207646871553, 2207646871297, 2207646871041, 2207646870785, 2207646870529, 2207646870273,
    2207646870017, 2207646869761, 2207646869506, 2207646869506, 2207646934786, 2207663711746, 2211958678786,
    3311470306306, 3311470306050, 3311470305794, 3311470305538, 3311470305282, 3311470305026, 3311470304770,
    3311470304514, 3311470304259, 3311470304259, 3311470369539, 3311487146499, 3315782113539, 4415293741059,
    4415293740803, 4415293740547, 4415293740291, 4415293740035, 4415293739779,
    0, 16711680, 4311613440, 1103823175680, 1103823110144, 1103823044608, 1103822979072, 1103822913536,
    1103822848000, 1103822782464, 1103822716928, 1103822651393, 1103822586113, 1103822586113, 1103839297793,
    1108134199553, 2207645761793, 2207645696257, 2207645630721, 2207645565185, 2207645499649, 2207645434113,
    2207645368577, 2207645303041, 2207645237506, 2207645172226, 2207645172226, 2207661883906, 2211956785666,
    3311468347906, 3311468282370, 3311468216834, 3311468151298, 3311468085762, 3311468020226, 3311467954690,
    3311467889154, 3311467823619, 3311467758339, 3311467758339, 3311484470019, 3315779371779, 4415290934019,
    4415290868483, 4415290802947, 4415290737411, 4415290671875, 4415290606339, 4415290540803,
    0, 4278190080, 1103773040640, 1103756263424, 1103739486208, 1103722708992, 1103705931776, 1103689154560,
    1103672377344, 1103655600128, 1103638822913, 1103622045953, 1103605334273, 1103605334273, 1107883524353,
    2207378374913, 2207361597697, 2207344820481, 2207328043265, 2207311266049, 2207294488833, 2207277711617,
    2207260934401, 2207244157186, 2207227380226, 2207210668546, 2207210668546, 2211488858626, 3310983709186,
    3310966931970, 3310950154754, 3310933377538, 3310916600322, 3310899823106, 3310883045890, 3310866268674,
    3310849491459, 3310832714499, 3310816002819, 3310816002819, 3315094192899, 4414589043459, 4414572266243,
    4414555489027, 4414538711811, 4414521934595, 4414505157379, 4414488380163, 4414471602947,
    0, 1095216660480, 1090921693184, 1086626725888, 1082331758592, 1078036791296, 1073741824000, 1069446856704,
    1065151889408, 1060856922113, 1056561955073, 1052267053313, 1047988863233, 1047988863233, 2143205523713,
    2138910556417, 2134615589121, 2130320621825, 2126025654529, 2121730687233, 2117435719937, 2113140752641,
    2108845785346, 2104550818306, 2100255916546, 2095977726466, 2095977726466, 3191194386946, 3186899419650,
    3182604452354, 3178309485058, 3174014517762, 3169719550466, 3165424583170, 3161129615874, 3156834648579,
    3152539681539, 3148244779779, 3143966589699, 3143966589699, 4239183250179, 4234888282883, 4230593315587,
    4226298348291, 4222003380995, 4217708413699, 4213413446403, 4209118479107, 4204823511812,
    0, -1099511627776, -2199023255552, -3298534883328, -4398046511104, -5497558138880, -6597069766656,
    -7696581394432, -8796093022207, -9895604649727, -10995116211967, -12094611062527, -13189827723007,
    -13189827723007, -14289339350783, -15388850978559, -16488362606335, -17587874234111, -18687385861887,
    -19786897489663, -20886409117439, -21985920745214, -23085432372734, -24184943934974, -25284438785534,
    -26379655446014, -26379655446014, -27479167073790, -28578678701566, -29678190329342, -30777701957118,
    -31877213584894, -32976725212670, -34076236840446, -35175748468221, -36275260095741, -37374771657981,
    -38474266508541, -39569483169021, -39569483169021, -40668994796797, -41768506424573, -42868018052349,
    -43967529680125, -45067041307901, -46166552935677, -47266064563453, -48365576191228, -49465087818748,
];
const OPPONENT_SEED_DIFFS: [i64; PIT_NUM * (MAX_SEED_NUM + 1)] = [
    0, 0, 0, 0, 0, 0, 0, 1, 257, 65793, 16843009, 4311810305, 1103823438081, 1103823438081, 1103823438081,
    1103823438081, 1103823438081, 1103823438081, 1103823438081, 1103823438081, 1103823438082, 1103823438338,
    1103823503874, 1103840281090, 1108135248386, 2207646876162, 2207646876162, 2207646876162, 2207646876162,
    2207646876162, 2207646876162, 2207646876162, 2207646876162, 2207646876163, 2207646876419, 2207646941955,
    2207663719171, 2211958686467, 3311470314243, 3311470314243, 3311470314243, 3311470314243, 3311470314243,
    3311470314243, 3311470314243, 3311470314243, 3311470314244, 3311470314500, 3311470380036,
    0, 0, 0, 0, 0, 0, 1, 257, 65793, 16843009, 4311810305, 1103823438081, 1103823438081, 1103823438081,
    1103823438081, 1103823438081, 1103823438081, 1103823438081, 1103823438081, 1103823438082, 1103823438338,
    1103823503874, 1103840281090, 1108135248386, 2207646876162, 2207646876162, 2207646876162, 2207646876162,
    2207646876162, 2207646876162, 2207646876162, 2207646876162, 2207646876163, 2207646876419, 2207646941955,
    2207663719171, 2211958686467, 3311470314243, 3311470314243, 3311470314243, 3311470314243, 3311470314243,
    3311470314243, 3311470314243, 3311470314243, 3311470314244, 3311470314500, 3311470380036, 3311487157252,
    0, 0, 0, 0, 0, 1, 257, 65793, 16843009, 4311810305, 1103823438081, 1103823438081, 1103823438081, 1103823438081,
    1103823438081, 1103823438081, 1103823438081, 1103823438081, 1103823438082, 1103823438338, 1103823503874,
    1103840281090, 1108135248386, 2207646876162, 2207646876162, 2207646876162, 2207646876162, 2207646876162,
    2207646876162, 2207646876162, 2207646876162, 2207646876163, 2207646876419, 2207646941955, 2207663719171,
    2211958686467, 3311470314243, 3311470314243, 3311470314243, 3311470314243, 3311470314243, 3311470314243,
    3311470314243, 3311470314243, 3311470314244, 3311470314500, 3311470380036, 3311487157252, 3315782124548,
    0, 0, 0, 0, 1, 257, 65793, 16843009, 4311810305, 1103823438081, 1103823438081, 1103823438081, 1103823438081,
    1103823438081, 1103823438081, 1103823438081, 1103823438081, 1103823438082, 1103823438338, 1103823503874,
    1103840281090, 1108135248386, 2207646876162, 2207646876162, 2207646876162, 2207646876162, 2207646876162,
    2207646876162, 2207646876162, 2207646876162, 2207646876163, 2207646876419, 2207646941955, 2207663719171,
    2211958686467, 3311470314243, 3311470314243, 3311470314243, 3311470314243, 3311470314243, 3311470314243,
    3311470314243, 3311470314243, 3311470314244, 3311470314500, 3311470380036, 3311487157252, 3315782124548,
    4415293752324,
    0, 0, 0, 1, 257, 65793, 16843009, 4311810305, 1103823438081, 1103823438081, 1103823438081, 1103823438081,
    1103823438081, 1103823438081, 1103823438081, 1103823438081, 1103823438082, 1103823438338, 1103823503874,
    1103840281090, 1108135248386, 2207646876162, 2207646876162, 2207646876162, 2207646876162, 2207646876162,
    2207646876162, 2207646876162, 2207646876162, 2207646876163, 2207646876419, 2207646941955, 2207663719171,
    2211958686467, 3311470314243, 3311470314243, 3311470314243, 3311470314243, 3311470314243, 3311470314243,
    3311470314243, 3311470314243, 3311470314244, 3311470314500, 3311470380036, 3311487157252, 3315782124548,
    4415293752324, 4415293752324,
    0, 0, 1, 257, 65793, 16843009, 4311810305, 1103823438081, 1103823438081, 1103823438081, 1103823438081,
    1103823438081, 1103823438081, 1103823438081, 1103823438081, 1103823438082, 1103823438338, 1103823503874,
    1103840281090, 1108135248386, 2207646876162, 2207646876162, 2207646876162, 2207646876162, 2207646876162,
    2207646876162, 2207646876162, 2207646876162, 2207646876163, 2207646876419, 2207646941955, 2207663719171,
    2211958686467, 3311470314243, 3311470314243, 3311470314243, 3311470314243, 3311470314243, 3311470314243,
    3311470314243, 3311470314243, 3311470314244, 3311470314500, 3311470380036, 3311487157252, 3315782124548,
    4415293752324, 4415293752324, 4415293752324,
];
const STORE_DIFFS: [isize; PIT_NUM * (MAX_SEED_NUM + 1)] = [
    0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4,
    0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4,
    0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4,
    0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4,
    0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4,
];


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Turn {
    First,
    Second,
}


#[derive(Copy, Clone)]
struct BoardState {
    turn: Turn,
    stores: [isize; PLAYER_NUM],
    seed_states: [i64; PLAYER_NUM],
}

impl Default for BoardState {
    fn default() -> BoardState {
        BoardState {
            turn: Turn::First,
            stores: [0, 0],
            seed_states: [INITIAL_SEEDS, INITIAL_SEEDS],
        }
    }
}

impl BoardState {
    pub fn seed(&self, turn: Turn, index: usize) -> isize {
        From::from(self.seed_states[turn as usize].to_le_bytes()[index])
    }

    pub fn play(&mut self, index: usize) -> Option<()> {
        let seed_num = self.seed(self.turn, index) as usize;
        let diff_index = index * (MAX_SEED_NUM + 1) + seed_num;
        let seed_diff = SEED_DIFFS[diff_index];
        if seed_diff == 0 {
            return None;
        }

        let opponent = opponent_turn(self.turn);
        let current_turn_index = self.turn as usize;
        let opponent_turn_index = opponent as usize;

        self.seed_states[current_turn_index] += SEED_DIFFS[diff_index];
        self.seed_states[opponent_turn_index] += OPPONENT_SEED_DIFFS[diff_index];
        self.stores[current_turn_index] += STORE_DIFFS[diff_index];
        let last_index = (index + seed_num) % SOW_CYCLE_SIZE;
        if last_index < PIT_NUM && self.seed(self.turn, last_index) == 1 {
            let opponent_index = PIT_NUM - last_index - 1;
            if self.seed(opponent, opponent_index) > 0 {
                self.stores[current_turn_index] += self.seed(self.turn, last_index) + self.seed(opponent, opponent_index);
                self.seed_states[current_turn_index] &= !(PIT_BIT_MASK << (last_index * PIT_BIT_NUM));
                self.seed_states[opponent_turn_index] &= !(PIT_BIT_MASK << (opponent_index * PIT_BIT_NUM));
            }
        }
        let first_seed_states = self.seed_states[Turn::First as usize];
        let second_seed_states = self.seed_states[Turn::Second as usize];
        if first_seed_states == 0 || second_seed_states == 0 {
            self.stores[Turn::First as usize] += sum_seeds(first_seed_states);
            self.stores[Turn::Second as usize] +=  sum_seeds(second_seed_states);
            self.seed_states[Turn::First as usize] = EMPTY_SEEDS;
            self.seed_states[Turn::Second as usize] = EMPTY_SEEDS;
        }
        if last_index != PIT_NUM {
            self.turn = opponent;
        }
        Some(())
    }
}

pub struct Board {
    state: BoardState,
    history: Vec<BoardState>,
}

impl Default for Board {
    fn default() -> Board {
        Board {
            state: Default::default(),
            history: Vec::with_capacity(HISTORY_SIZE),
        }
    }
}

impl Board {
    pub fn reset(&mut self) {
        self.state = Default::default();
        self.history.clear();
    }

    pub fn reset_with_seeds(&mut self, first_seeds: [isize; PIT_NUM], second_seeds: [isize; PIT_NUM]) {
        let mut first_seeds_bytes: [u8; 8] = [0; 8];
        let mut second_seeds_bytes: [u8; 8] = [0; 8];
        for i in 0..PIT_NUM {
            first_seeds_bytes[i] = first_seeds[i] as u8; // u8::try_from(first_seeds[i]).unwrap();
            second_seeds_bytes[i] = second_seeds[i] as u8; // u8::try_from(second_seeds[i]).unwrap();
        }
        self.reset();
        self.state.seed_states = [i64::from_le_bytes(first_seeds_bytes), i64::from_le_bytes(second_seeds_bytes)];
    }

    pub fn turn(&self) -> Turn {
        self.state.turn
    }

    pub fn store(&self, turn: Turn) -> isize {
        self.state.stores[turn as usize]
    }

    pub fn seed(&self, turn: Turn, index: usize) -> isize {
        From::from(self.state.seed_states[turn as usize].to_le_bytes()[index])
    }

    pub fn is_over(&self) -> bool {
        self.state.seed_states[Turn::First as usize] == 0 || self.state.seed_states[Turn::Second as usize] == 0
    }

    pub fn play(&mut self, index: usize) -> Option<()> {
        self.history.push(self.state);
        let result = self.state.play(index);
        if let None = result {
            self.history.pop();
        }
        result
    }

    pub fn undo(&mut self) -> Option<()> {
        match self.history.pop() {
            Some(state) => {
                self.state = state;
                Some(())
            },
            None => None,
        }
    }

    pub(crate) fn seed_states(&self) -> [i64; PLAYER_NUM] {
        self.state.seed_states
    }
}


pub fn opponent_turn(turn: Turn) -> Turn {
    if turn == Turn::First {
        Turn::Second
    } else {
        Turn::First
    }
}


fn sum_seeds(seed_state: i64) -> isize {
    seed_state.to_le_bytes().iter().sum::<u8>() as isize
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    fn assert_board(board: &Board, turn: Turn, stores: [isize; PLAYER_NUM], first_seeds: [isize; PIT_NUM],
                    second_seeds: [isize; PIT_NUM]){
        assert_eq!(board.turn(), turn);
        assert_eq!(board.store(Turn::First), stores[Turn::First as usize]);
        assert_eq!(board.store(Turn::Second), stores[Turn::Second as usize]);
        for i in 0..PIT_NUM {
            let actual = board.seed(Turn::First, i);
            let expected = first_seeds[i];
            assert_eq!(actual, expected, "first player's {}th seeds is expected {} but acturally {}", i, expected, actual);

            let actual = board.seed(Turn::Second, i);
            let expected = second_seeds[i];
            assert_eq!(actual, expected, "second player's {}th seeds is expected {} but acturally {}", i, expected, actual);
        }
    }

    fn create_board(turn: Turn, stores: [isize; PLAYER_NUM], first_seeds: [isize; PIT_NUM],
                    second_seeds: [isize; PIT_NUM]) -> Board {
        let mut first_seeds_bytes: [u8; 8] = [0; 8];
        let mut second_seeds_bytes: [u8; 8] = [0; 8];
        for i in 0..PIT_NUM {
            first_seeds_bytes[i] = u8::try_from(first_seeds[i]).unwrap();
            second_seeds_bytes[i] = u8::try_from(second_seeds[i]).unwrap();
        }
        Board {
            state: BoardState {
                turn: turn,
                stores: stores,
                seed_states: [i64::from_le_bytes(first_seeds_bytes), i64::from_le_bytes(second_seeds_bytes)],
            },
            ..Default::default()
        }
    }

    #[test]
    fn stores_in_default_board_are_zeros() {
        let board: Board = Default::default();

        assert_eq!(board.store(Turn::First), 0);
        assert_eq!(board.store(Turn::Second), 0);
    }

    #[test]
    fn turn_in_default_board_is_first() {
        let board: Board = Default::default();

        assert_eq!(board.turn(), Turn::First);
    }

    #[test]
    fn pits_in_default_board_have_default_seeds() {
        let board: Board = Default::default();

        assert_eq!(board.seed(Turn::First, 0), 4);
        assert_eq!(board.seed(Turn::First, 1), 4);
        assert_eq!(board.seed(Turn::First, 2), 4);
        assert_eq!(board.seed(Turn::First, 3), 4);
        assert_eq!(board.seed(Turn::First, 4), 4);
        assert_eq!(board.seed(Turn::First, 5), 4);

        assert_eq!(board.seed(Turn::Second, 0), 4);
        assert_eq!(board.seed(Turn::Second, 1), 4);
        assert_eq!(board.seed(Turn::Second, 2), 4);
        assert_eq!(board.seed(Turn::Second, 3), 4);
        assert_eq!(board.seed(Turn::Second, 4), 4);
        assert_eq!(board.seed(Turn::Second, 5), 4);
    }

    #[test]
    fn play_sows_seeds_into_pits() {
        let mut board: Board = Default::default();
        let result = board.play(0);

        assert_eq!(result, Some(()));
        assert_board(&board, Turn::Second, [0, 0], [0, 5, 5, 5, 5, 4], [4, 4, 4, 4, 4, 4]);
    }

    #[test]
    fn play_sows_seeds_into_stores() {
        let mut board: Board = Default::default();
        let result = board.play(3);

        assert_eq!(result, Some(()));
        assert_board(&board, Turn::Second, [1, 0], [4, 4, 4, 0, 5, 5], [5, 4, 4, 4, 4, 4]);
    }

    #[test]
    fn play_does_not_change_turn_if_sow_ends_at_store() {
        let mut board: Board = Default::default();
        let result = board.play(2);

        assert_eq!(result, Some(()));
        assert_board(&board, Turn::First, [1, 0], [4, 4, 0, 5, 5, 5], [4, 4, 4, 4, 4, 4]);
    }

    #[test]
    fn play_returns_error_if_empty_pit_is_passed() {
        let mut board: Board = Default::default();
        let result = board.play(2);

        assert_eq!(result, Some(()));

        let result = board.play(2);

        assert_eq!(result, None);
        assert_board(&board, Turn::First, [1, 0], [4, 4, 0, 5, 5, 5], [4, 4, 4, 4, 4, 4]);
    }

    #[test]
    fn play_capture_seeds_if_sow_ends_at_current_player_empty_pit() {
        let mut board: Board = Default::default();
        let result = board.play(5);

        assert_eq!(result, Some(()));
        assert_board(&board, Turn::Second, [1, 0], [4, 4, 4, 4, 4, 0], [5, 5, 5, 4, 4, 4]);

        let result = board.play(5);

        assert_eq!(result, Some(()));
        assert_board(&board, Turn::First, [1, 1], [5, 5, 5, 4, 4, 0], [5, 5, 5, 4, 4, 0]);

        let result = board.play(0);

        assert_eq!(result, Some(()));
        assert_board(&board, Turn::Second, [7, 1], [0, 6, 6, 5, 5, 0], [0, 5, 5, 4, 4, 0]);
    }

    #[test]
    fn play_does_not_capture_if_counterpart_of_last_pit_is_empty() {
        let mut board: Board = Default::default();
        let result = board.play(5);

        assert_eq!(result, Some(()));
        assert_board(&board, Turn::Second, [1, 0], [4, 4, 4, 4, 4, 0], [5, 5, 5, 4, 4, 4]);

        let result = board.play(0);

        assert_eq!(result, Some(()));
        assert_board(&board, Turn::First, [1, 0], [4, 4, 4, 4, 4, 0], [0, 6, 6, 5, 5, 5]);

        let result = board.play(1);

        assert_eq!(result, Some(()));
        assert_board(&board, Turn::Second, [1, 0], [4, 0, 5, 5, 5, 1], [0, 6, 6, 5, 5, 5]);
    }

    #[test]
    fn undo_reverses_board_state_by_one_move() {
        let mut board: Board = Default::default();
        let result = board.play(3);

        assert_eq!(result, Some(()));
        assert_board(&board, Turn::Second, [1, 0], [4, 4, 4, 0, 5, 5], [5, 4, 4, 4, 4, 4]);

        let result = board.undo();

        assert_eq!(result, Some(()));
        assert_board(&board, Turn::First, [0, 0], [4, 4, 4, 4, 4, 4], [4, 4, 4, 4, 4, 4]);
    }

    #[test]
    fn game_ends_when_either_side_has_no_seed() {
        let mut board = create_board(Turn::First, [0, 0], [0, 0, 0, 0, 0, 1], [0, 0, 0, 0, 0, 1]);

        assert_board(&board, Turn::First, [0, 0], [0, 0, 0, 0, 0, 1], [0, 0, 0, 0, 0, 1]);

        let result = board.play(5);

        assert_eq!(board.is_over(), true);
        assert_board(&board, Turn::First, [1, 1], [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn reset_with_seeds_resets_board_with_passed_seeds() {
        let mut board: Board = Default::default();
        board.reset_with_seeds([0, 1, 2, 3, 4, 5], [1, 2, 3, 4, 5, 0]);

        assert_board(&board, Turn::First, [0, 0], [0, 1, 2, 3, 4, 5], [1, 2, 3, 4, 5, 0]);
    }

    #[test]
    fn undo_failes_right_after_reset_with_seesds() {
        let mut board: Board = Default::default();

        assert_eq!(board.undo(), None);
    }
}
