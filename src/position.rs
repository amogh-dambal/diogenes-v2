use std::collections::VecDeque;
use std::fmt::Display;
use std::fmt::Write;
use std::str::FromStr;

use strum::IntoEnumIterator;

use crate::bitboard::Bitboard;
use crate::board::{File, Rank};
use crate::castling::CastlingRights;
use crate::fen;
use crate::r#move::Move;
use crate::square::Square;
use crate::color::{Color, NUM_COLORS};
use crate::piece::{Piece, NUM_PIECE_ITER, NUM_UNIQUE_PIECES};


#[derive(Debug)]
pub struct Position {
    pieces: [Bitboard; 8],
    empty: Bitboard,
    occupied: Bitboard,

    fullmove: i32,
    ply: i32,
    side_to_move: Color,
    ep: Square,
    castling_rights: CastlingRights,
    history: VecDeque<Move>,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for rank in Rank::iter().rev() {
            for file in File::iter() {
                let idx = Square::index(file, rank);
                write!(s, "{} ", self.get_piece_at(idx)).unwrap();
            }
            write!(s, "\n").unwrap();
        }

        return write!(f, "{}", s);
    }
}

impl Position {
    pub fn new() -> Position {
        return Position::from_fen(fen::STARTING_FEN);
    }

    pub fn new_fen(fen: &str) -> Position {
        return Position::from_fen(fen);
    }

    pub fn pieces(&self, pt: &Piece) -> Bitboard {
        let piece_val: i8 = *pt as i8;
        let piece_key: usize = (NUM_COLORS + (piece_val % NUM_UNIQUE_PIECES)) as usize;
        return self.pieces[piece_key] & self.color_pieces(&pt.color());
    }

    pub fn all_pieces(&self, pt: &Piece) -> Bitboard {
        let piece_val: i8 = *pt as i8;
        let piece_key: usize = (NUM_COLORS + (piece_val % NUM_UNIQUE_PIECES)) as usize;

        return self.pieces[piece_key];
    }

    pub fn color_pieces(&self, c: &Color) -> Bitboard {
        return self.pieces[*c as usize];
    }

    fn get_piece_at(&self, idx: usize) -> Piece {
        let mask: u64 = 1 << idx;
        if (self.empty & mask).bool() {
            return Piece::None;
        }
        else {
            for piece in Piece::iter().take(NUM_PIECE_ITER) {
                let piece_bb: Bitboard = self.pieces(&piece);
                let bb: Bitboard = piece_bb & mask;
                if bb.bool() {
                    return piece;
                }                    
            }
            return Piece::None;
        }
    }
    
    pub fn to_fen(&self) -> String {
        println!("Converting board:\n{} to FEN:", &self);
        let mut pieces = String::new();

        for rank in Rank::iter().rev() {
            let mut empty_cnt = 0;
            for file in File::iter() {
                match self.get_piece_at(Square::index(file, rank)) {
                    Piece::None => {
                        empty_cnt += 1;
                    }
                    piece => {
                        if empty_cnt > 0 {
                            pieces.push_str(empty_cnt.to_string().as_str());
                            empty_cnt = 0;
                        }
                        pieces.push_str(piece.to_string().as_str());
                    }
                }
            }
            if empty_cnt > 0 {
                pieces.push_str(empty_cnt.to_string().as_str());
            }
            pieces.push_str("/");
        }
        // Remove the trailing slash from the constructed string
        pieces.pop();

        let active_color: String;
        match self.side_to_move {
            Color::White => {active_color = "w".to_string()}
            Color::Black => {active_color = "b".to_string()}
            _ => {panic!("Invalid position")}
        }

        let ep: String = self.ep.to_string();
        let cr: String = self.castling_rights.to_string();
        let ply: String = self.ply.to_string();
        let fullmove = self.fullmove.to_string();

        return [pieces, active_color, cr, ep, ply, fullmove].join(" ");
    }

    fn from_fen(fen: &str) -> Position {
        let fields: Vec<&str> = fen.split(" ").collect();
        
        let pieces: [Bitboard; 8] = fen::parse_pieces(fields[0]);
        let mut occupied: Bitboard = Bitboard::default();
        for pbb in pieces {
            occupied |= pbb;
        }
        let empty: Bitboard = !occupied;

        let active_color: Color;
        match fields[1] {
            "w" => {active_color = Color::White},
            "b" => {active_color = Color::Black},
            _ => {panic!("FEN string may be invalid")},
        }

        let castling_rights: CastlingRights;
        match CastlingRights::from_str(fields[2]) {
            Ok(cr) => {castling_rights = cr}
            Err(_) => {panic!("Invalid FEN string!")}
        }

        let ep_square = Square::from_str(fields[3]);
        let ply: i32 = fields[4].parse::<i32>().expect("Invalid FEN string");
        let fullmove: i32 = fields[5].parse::<i32>().expect("Invalid FEN string!");

        return Position{
            empty,
            occupied,
            ep: ep_square,
            castling_rights,
            side_to_move: active_color,
            pieces,
            ply,
            fullmove,
            history: VecDeque::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fen;
    use crate::position::Position;

    #[test]
    fn test_build_position_from_starting_fen() {
        let pos: Position = Position::new_fen(fen::STARTING_FEN);
        let s = pos.to_fen();
        assert_eq!(s, fen::STARTING_FEN);
    }

    #[test]
    fn test_build_position_from_random_fen() {
        let test_fens: [&str; 4] = [
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2",
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2",
            "4k3/8/8/8/8/8/4P3/4K3 w - - 5 39",
        ];
        for test_fen in test_fens {
            let pos = Position::new_fen(test_fen);
            let s = pos.to_fen();
            assert_eq!(s, test_fen);
        }
    }

    #[test]
    fn test_build_position_default() {
        let default_pos: Position = Position::new();
        let pos: Position = Position::new_fen(fen::STARTING_FEN);
        assert_eq!(pos.to_fen(), default_pos.to_fen());
    }
}