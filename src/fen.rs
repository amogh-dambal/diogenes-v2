use num_traits::FromPrimitive;
use strum::IntoEnumIterator;

use crate::bitboard::Bitboard;
use crate::board::{self, File, Rank};
use crate::piece::{Piece, NUM_UNIQUE_PIECES};
use crate::color::{Color, NUM_COLORS};


pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/// Parse the first field in a FEN string and build bitboards
/// that represent the position internally.
pub fn parse_pieces(field: &str) -> [Bitboard; 8] {
    let mut piece_bbs: [Bitboard; 8] = [Bitboard::default(); 8];
    for (piece_chars, rank) in field.rsplit("/").zip(Rank::iter()) {
        let mut file_idx: u32 = 0;
        for piece in piece_chars.chars() {
            println!("file: {:?}, rank: {:?}", file_idx, rank);
            match piece.to_digit(10) {
                Some(skip) => {
                    if skip != 8 {
                        file_idx += skip;
                    }
                }
                None => {
                    let file: File;
                    match File::from_u32(file_idx) {
                        Some(f) => {file = f;}
                        None => panic!("Invalid file!")
                    }

                    let sq_idx: usize = board::index(file, rank);
                    let bb: u64 = 1 << sq_idx;
    
                    let p: Piece;
                    match Piece::try_from(piece) {
                        Ok(val) => {p = val}
                        Err(_) => panic!("Invalid FEN string")
                    }

                    let piece_idx: usize = (NUM_COLORS + (p as i8 % NUM_UNIQUE_PIECES)) as usize;
                    piece_bbs[piece_idx] |= bb;
                    
                    let c: Color;
                    if piece.is_ascii_uppercase() {
                        c = Color::White;
                    } 
                    else {
                        c = Color::Black;
                    }
                    piece_bbs[c as usize] |= bb;
                    file_idx += 1;
                }
            }
        }
    }

    return piece_bbs;
}

#[cfg(test)]
mod tests {
    use crate::bitboard::Bitboard;
    use super::parse_pieces;

    #[test]
    fn test_parse_pieces() {
        let white_pieces = Bitboard::new(0xFFFF);
        let black_pieces = Bitboard::new(0xFFFF).flip_vertical();
        let pawns = Bitboard::new(0xFF00) | Bitboard::new(0xFF00).flip_vertical();
        let knights = Bitboard::new(0x42) | Bitboard::new(0x42).flip_vertical();
        let bishops = Bitboard::new(0x24) | Bitboard::new(0x24).flip_vertical();
        let rooks = Bitboard::new(0x81) | Bitboard::new(0x81).flip_vertical();
        let queens = Bitboard::new(0x08) | Bitboard::new(0x08).flip_vertical();
        let kings = Bitboard::new(0x10) | Bitboard::new(0x10).flip_vertical();

        let actual_bbs = parse_pieces("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        let expected_bbs: [Bitboard; 8] = [
            white_pieces, // white pieces
            black_pieces, // black pieces
            pawns, // pawns
            knights, // knights
            bishops, // bishops
            rooks, // rooks
            queens, // queens
            kings, // kings
        ];

        for (actual, expected) in actual_bbs.iter().zip(expected_bbs) {
            assert_eq!(*actual, expected, "Testing bitboards {:#064b} and {:#064b}", actual, expected);
        }
    }
}