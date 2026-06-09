use crate::prelude::*;

/// A serializeable data structure for sudoku board games.
#[derive(Debug, PartialEq, Eq)]
pub struct GameData {
    board: [u8; 81],
    pub difficulty: u16,
}

impl GameData {
    /// Create a sudoku board.
    pub fn to_board(&self) -> Board {
        Board::new(self.board)
    }

    // Parse a string where empty cells are '.' chars.
    pub fn from_str(value: &str, difficulty: u16) -> Self {
        assert!(value.is_ascii());
        assert_eq!(value.len(), 81);
        let mut board: [u8; 81] = value.as_bytes().try_into().unwrap();
        for i in 0..81 {
            board[i] = match board[i] {
                b'.' => 0,
                b'0'..=b'9' => (board[i] - b'0') as u8,
                _ => panic!("Invalid data. {value}"),
            }
        }
        Self { board, difficulty }
    }

    pub fn serialize_binary(&self) -> [u8; 42] {
        let mut bin = [0; 42];
        for i in 0..81 {
            let j = i / 2;
            if i % 2 == 0 {
                bin[j] += self.board[i] << 4;
            } else {
                bin[j] += self.board[i];
            }
        }
        bin[41] = (self.difficulty & 0xFF) as u8;
        bin[40] |= ((self.difficulty >> 8) & 0x0F) as u8;
        bin
    }

    pub fn deserialize_binary(bin: &[u8]) -> Option<Self> {
        if bin.len() != 42 {
            return None;
        }
        let mut data = Self { board: [0; 81], difficulty: 0 };

        for i in 0..81 {
            let j = i / 2;
            if i % 2 == 0 {
                data.board[i] = bin[j] >> 4;
            } else {
                data.board[i] = bin[j] & 0x0F;
            }
        }

        data.difficulty = (bin[40] as u16 & 0x0F) << 8;
        data.difficulty |= bin[41] as u16;

        Some(data)
    }

    pub fn deserialize_binaries(bin: &[u8]) -> Vec<Self> {
        assert_eq!(bin.len() % 42, 0);
        let mut vec = Vec::with_capacity(bin.len() / 42);
        for data in bin.chunks(42) {
            vec.push(Self::deserialize_binary(data).unwrap());
        }
        vec
    }
}
