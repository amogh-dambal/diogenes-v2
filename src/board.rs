pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub enum Color {
    White,
    Black,
}

#[derive(Debug)]
pub struct Board {
    // 2 color bitboards and
    // 6 piece bitboards
    pieces: [u64; 8],
    empty: u64,
    occupied: u64,
}

impl Board {
    fn pieces(&self, pt: Piece) -> u64 {
        return 0;
    }
}