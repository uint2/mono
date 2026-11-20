mod engine;

use engine::*;

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
}
