#[derive(Clone, Copy)]
pub enum Piece {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

impl Piece {
    /// The offset from the King's UTF-8 codepoint.
    pub const fn as_u32(self) -> u32 {
        match self {
            Self::King => 0,
            Self::Queen => 1,
            Self::Rook => 2,
            Self::Bishop => 3,
            Self::Knight => 4,
            Self::Pawn => 5,
        }
    }

    pub const fn as_display_char(self, side: Side) -> char {
        unsafe { char::from_u32_unchecked(side.king_utf8_codepoint() + self.as_u32()) }
    }
}

#[derive(Clone, Copy)]
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
