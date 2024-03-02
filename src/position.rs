use std::collections::VecDeque;

use super::castling::CastlingRights;
use super::fen;
use super::r#move::Move;
use super::square::Square;
use super::types::Color;
use super::types::Piece;
use super::types::NUM_COLORS;


#[derive(Debug)]
pub struct Position {
    /**
     * Need to keep track of:
     - pieces on the board
     - side to move
     - castling rights
     - EP target squares
     - set of reversible moves
     */
    pieces: [u64; 8],
    empty: u64,
    occupied: u64,

    fullmove: i32,
    ply: i32,
    side_to_move: Color,
    ep: Square,
    castling_rights: CastlingRights,
    history: VecDeque<Move>,
}

impl Position {
    pub fn new() -> Position {
        return Position::from_fen(fen::STARTING_FEN);
    }

    pub fn new_fen(fen: &str) -> Position {
        return Position::from_fen(fen);
    }

    pub fn pieces_of_type(&self, pt: Piece) -> u64 {
        let idx: usize = (NUM_COLORS + pt as i8) as usize;
        return self.pieces[idx];
    }

    pub fn pieces_of_color(&self, c: Color) -> u64 {
        return self.pieces[c as usize];
    }

    pub fn pieces_of_color_type(&self, c: Color, pt: Piece) -> u64 {
        return self.pieces[c as usize] & self.pieces_of_type(pt);
    }

    // Parse a FEN string to construct a specific position object
    fn from_fen(fen: &str) -> Position {
        let fields: Vec<&str> = fen.split(" ").collect();

        let pieces: [u64; 8] = fen::parse_pieces(fields[0]);
        let mut empty: u64 = 0;
        for pbb in pieces {
            empty |= pbb;
        }
        let occupied: u64 = !empty;

        let active_color: Color;
        match fields[1] {
            "w" => {active_color = Color::White},
            "b" => {active_color = Color::Black},
            _ => {panic!("FEN string may be invalid")},
        }

        let castling_rights: CastlingRights = CastlingRights::from_str(fields[2]);
        let ply: i32 = fields[3].parse::<i32>().expect("Invalid FEN string");
        let fullmove: i32 = fields[4].parse::<i32>().expect("Invalid FEN string!");

        return Position{
            empty,
            occupied,
            ep: Square::from_str(fields[3]),
            castling_rights,
            side_to_move: active_color,
            pieces: pieces,
            ply,
            fullmove,
            history: VecDeque::new(),
        }
    }
}