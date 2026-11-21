#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

impl Piece {
    pub const ALL: [Self; 6] =
        [Self::King, Self::Queen, Self::Bishop, Self::Knight, Self::Rook, Self::Pawn];

    /// The offset from the King's UTF-8 codepoint.
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::King => 0,
            Self::Queen => 1,
            Self::Rook => 2,
            Self::Bishop => 3,
            Self::Knight => 4,
            Self::Pawn => 5,
        }
    }

    pub const fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::King,
            1 => Self::Queen,
            2 => Self::Rook,
            3 => Self::Bishop,
            4 => Self::Knight,
            5 => Self::Pawn,
            _ => panic!(),
        }
    }

    pub const fn as_display_char(self, side: Side) -> char {
        unsafe {
            char::from_u32_unchecked(side.king_utf8_codepoint() + self.as_u8() as u32)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    White,
    Black,
}

impl Side {
    /// The UTF-8 codepoint of the King.
    pub const fn king_utf8_codepoint(self) -> u32 {
        match self {
            Self::White => 0x2654,
            Self::Black => 0x265A,
        }
    }
}
