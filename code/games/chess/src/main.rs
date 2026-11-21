mod engine;

use engine::*;

#[derive(Debug, Clone, Copy)]
struct Board([Square; 64]);

const fn create_initial_board() -> Board {
    macro_rules! sq {
        ($side:ident, $piece:ident) => {
            Square::from(Side::$side, Piece::$piece)
        };
    }
    Board::EMPTY
        .set(*b"a1", sq!(White, Rook))
        .set(*b"b1", sq!(White, Knight))
        .set(*b"c1", sq!(White, Bishop))
        .set(*b"d1", sq!(White, Queen))
        .set(*b"e1", sq!(White, King))
        .set(*b"f1", sq!(White, Bishop))
        .set(*b"g1", sq!(White, Knight))
        .set(*b"h1", sq!(White, Rook))
        .set(*b"a2", sq!(White, Pawn))
        .set(*b"b2", sq!(White, Pawn))
        .set(*b"c2", sq!(White, Pawn))
        .set(*b"d2", sq!(White, Pawn))
        .set(*b"e2", sq!(White, Pawn))
        .set(*b"f2", sq!(White, Pawn))
        .set(*b"g2", sq!(White, Pawn))
        .set(*b"h2", sq!(White, Pawn))
        .set(*b"a7", sq!(Black, Pawn))
        .set(*b"b7", sq!(Black, Pawn))
        .set(*b"c7", sq!(Black, Pawn))
        .set(*b"d7", sq!(Black, Pawn))
        .set(*b"e7", sq!(Black, Pawn))
        .set(*b"f7", sq!(Black, Pawn))
        .set(*b"g7", sq!(Black, Pawn))
        .set(*b"h7", sq!(Black, Pawn))
        .set(*b"a8", sq!(Black, Rook))
        .set(*b"b8", sq!(Black, Knight))
        .set(*b"c8", sq!(Black, Bishop))
        .set(*b"d8", sq!(Black, Queen))
        .set(*b"e8", sq!(Black, King))
        .set(*b"f8", sq!(Black, Bishop))
        .set(*b"g8", sq!(Black, Knight))
        .set(*b"h8", sq!(Black, Rook))
}

impl Board {
    const INITIAL: Self = create_initial_board();
    const EMPTY: Self = Self([Square::empty(); 64]);

    const fn set(mut self, loc: [u8; 2], square: Square) -> Self {
        let index = 8 * (loc[0] - b'a') + (loc[1] - b'1');
        self.0[index as usize] = square;
        self
    }
}

/// Represents one single square (either black or white) on a chessboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Square(u8);

impl Square {
    const fn to_piece(self) -> Option<(Side, Piece)> {
        let v = self.0;
        if v & 0x80 == 0 {
            return None;
        }
        let side = if v & 0x40 == 0 { Side::White } else { Side::Black };
        let piece = Piece::from_u8(v & 0x07);
        Some((side, piece))
    }

    const fn from(side: Side, piece: Piece) -> Self {
        let v = match side {
            Side::White => 0x80,
            Side::Black => 0xC0,
        };
        Self(v | piece.as_u8())
    }

    const fn empty() -> Self {
        Self(0)
    }
}

#[test]
fn square_bijection() {
    for side in [Side::White, Side::Black] {
        for piece in Piece::ALL {
            let square = Square::from(side, piece);
            println!("{square:?}");
            let (s, p) = square.to_piece().unwrap();
            assert_eq!(side, s);
            assert_eq!(piece, p);
        }
    }
}

fn u8_to_piece(v: u8) -> Option<(Side, Piece)> {
    if v & 0x80 == 0 {
        return None;
    }
    let side = if v & 0x40 == 0 { Side::White } else { Side::Black };
    let piece = Piece::from_u8(v & 0x07);
    Some((side, piece))
}

fn main() {
    log::init(Some(log::LevelFilter::Trace));
    log::info!("Hello, world!");

    println!("{}", Piece::Pawn.as_display_char(Side::White));
    println!("{}", Piece::Knight.as_display_char(Side::White));
    println!("{}", Piece::Bishop.as_display_char(Side::White));
    println!("{}", Piece::Rook.as_display_char(Side::White));
    println!("{}", Piece::Queen.as_display_char(Side::White));
    println!("{}", Piece::King.as_display_char(Side::White));

    println!("{}", Piece::Pawn.as_display_char(Side::Black));
    println!("{}", Piece::Knight.as_display_char(Side::Black));
    println!("{}", Piece::Bishop.as_display_char(Side::Black));
    println!("{}", Piece::Rook.as_display_char(Side::Black));
    println!("{}", Piece::Queen.as_display_char(Side::Black));
    println!("{}", Piece::King.as_display_char(Side::Black));

    let x: [Option<(Side, Piece)>; 64] =
        core::array::from_fn(|i| Board::INITIAL.0[i].to_piece());
    println!("{:?}", x);
}
