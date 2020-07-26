use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

use crate::board;
use crate::board::Board;
use super::evaluator;

#[derive(PartialEq, Eq, Hash)]
pub struct PositionKey(i64, i64);
pub struct PositionValue {
    pub value: i32,
    pub visit: i32,
}

impl PositionKey {
    pub fn to_le_bytes(&self) -> [u8; 16] {
        let mut result: [u8; 16] = [0; 16];
        copy_i64_to_slice(self.0, &mut result[0..8]);
        copy_i64_to_slice(self.1, &mut result[8..16]);
        result
    }

    pub fn from_le_bytes(bytes: &[u8]) -> PositionKey {
        let x = slice_to_i64(&bytes[0..8]);
        let y = slice_to_i64(&bytes[8..16]);
        PositionKey(x, y)
    }
}

impl PositionValue {
    pub fn to_le_bytes(&self) -> [u8; 8] {
        let mut result: [u8; 8] = [0; 8];
        copy_i32_to_slice(self.value, &mut result[0..4]);
        copy_i32_to_slice(self.visit, &mut result[4..8]);
        result
    }

    pub fn from_le_bytes(bytes: &[u8]) -> PositionValue {
        let value = slice_to_i32(&bytes[0..4]);
        let visit = slice_to_i32(&bytes[4..8]);
        PositionValue {
            value: value,
            visit: visit,
        }
    }
}

pub struct PositionMap(HashMap<PositionKey, PositionValue>);

impl Default for PositionMap {
    fn default() -> PositionMap {
        PositionMap(HashMap::new())
    }
}

impl PositionMap {
    pub fn load(file_path: &str) -> std::io::Result<PositionMap> {
        let mut reader = BufReader::new(File::open(file_path)?);

        let mut buffer = [0; 8];
        reader.read_exact(&mut buffer)?;
        let header = PositionFileHeader::from_le_bytes(&buffer);

        let mut position_map = HashMap::with_capacity(header.record_num as usize);
        let mut buffer = [0; 16 + 8];
        for _ in 0..header.record_num {
            reader.read_exact(&mut buffer)?;
            let key = PositionKey::from_le_bytes(&buffer[0..16]);
            let value = PositionValue::from_le_bytes(&buffer[16..24]);
            position_map.insert(key, value);
        }

        Ok(PositionMap(position_map))
    }

    pub fn save(&self, file_path: &str) -> std::io::Result<()> {
        let PositionMap(position_map) = self;
        let mut writer = BufWriter::new(File::create(file_path)?);
        let header = PositionFileHeader {
            version: 1,
            record_num: position_map.len() as u32,
        };
        writer.write_all(&header.to_le_bytes())?;
        for (key, value) in position_map.iter() {
            writer.write_all(&key.to_le_bytes())?;
            writer.write_all(&value.to_le_bytes())?;
        }
        writer.flush()?;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, board: &Board) -> Option<PositionValue> {
        let PositionMap(position_map) = self;
        let turn = board.turn();
        let opponent = board::opponent_turn(turn);
        let seed_states = board.seed_states();
        let key = PositionKey(seed_states[turn as usize], seed_states[opponent as usize]);
        if let Some(value) = position_map.get(&key) {
            Some(PositionValue {
                value: value.value + ((board.store(turn) - board.store(opponent)) * evaluator::VALUE_PER_SEED)  as i32,
                visit: value.visit,
            })
        } else {
            None
        }
    }

    pub fn insert(&mut self, board: &Board, value: isize) -> () {
        let PositionMap(position_map) = self;
        let turn = board.turn();
        let opponent = board::opponent_turn(turn);
        let seed_states = board.seed_states();
        let key = PositionKey(seed_states[turn as usize], seed_states[opponent as usize]);
        let visit = if let Some(v) = position_map.get(&key) {
            v.visit
        } else {
            0
        };
        let position_value = PositionValue {
            value: (value - (board.store(turn) - board.store(opponent)) * evaluator::VALUE_PER_SEED) as i32,
            visit: visit + 1,
        };
        position_map.insert(key, position_value);
    }
}

pub struct PositionFileHeader {
    pub version: u32,
    pub record_num: u32,
}

impl PositionFileHeader {
    pub fn to_le_bytes(&self) -> [u8; 8] {
        let mut result: [u8; 8] = [0; 8];
        copy_u32_to_slice(self.version, &mut result[0..4]);
        copy_u32_to_slice(self.record_num, &mut result[4..8]);
        result
    }

    pub fn from_le_bytes(bytes: &[u8]) -> PositionFileHeader {
        let version = slice_to_u32(&bytes[0..4]);
        let record_num = slice_to_u32(&bytes[4..8]);
        PositionFileHeader {
            version: version,
            record_num: record_num,
        }
    }
}


fn copy_i32_to_slice(x: i32, slice: &mut [u8]) {
    slice.copy_from_slice(&x.to_le_bytes());
}

fn copy_i64_to_slice(x: i64, slice: &mut [u8]) {
    slice.copy_from_slice(&x.to_le_bytes());
}

fn copy_u32_to_slice(x: u32, slice: &mut [u8]) {
    slice.copy_from_slice(&x.to_le_bytes());
}

fn slice_to_i32(slice: &[u8]) -> i32 {
    let mut buffer: [u8; 4] = [0; 4];
    buffer.copy_from_slice(slice);
    i32::from_le_bytes(buffer)
}

fn slice_to_i64(slice: &[u8]) -> i64 {
    let mut buffer: [u8; 8] = [0; 8];
    buffer.copy_from_slice(slice);
    i64::from_le_bytes(buffer)
}

fn slice_to_u32(slice: &[u8]) -> u32 {
    let mut buffer: [u8; 4] = [0; 4];
    buffer.copy_from_slice(slice);
    u32::from_le_bytes(buffer)
}
