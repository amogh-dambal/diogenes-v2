use std::collections::VecDeque;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Write;
use std::ops::Index;
use std::ops::IndexMut;
use std::str::FromStr;

use num_traits::FromPrimitive;
use strum::IntoEnumIterator;

use crate::bitboard::Bitboard;
use crate::board;
use crate::board::{File, Rank};
use crate::castling::CastlingRights;
use crate::error::DiogenesError;
use crate::error::DiogenesResult;
use crate::r#move::Move;
use crate::square::Square;
use crate::color::Color;
use crate::piece::Piece;

const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/// Represents a denormalized set of bitboards that represent
/// all chess pieces on the board in a given position.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PieceSet([Bitboard; 14]);

impl Debug for PieceSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "white pieces:")?;
        write!(f, "{}", self.0[0])?;
        writeln!(f, "black pieces:")?;
        write!(f, "{}", self.0[1])?;
        for piece in Piece::iter() {
            writeln!(f, "{piece:?}:")?;
            write!(f, "{}", self[piece])?;
        }

        writeln!(f)
    }
}

impl PieceSet {
    /// Returns a bitboard corresponding to the empty squares.
    /// 
    /// Equivalent to an XOR of [Self::occupied()] and the 
    /// universal bitboard.
    pub fn empty(&self) -> Bitboard {
        Bitboard::new(u64::MAX) ^ self.occupied()
    }

    /// Returns a bitboard corresponding to the occupied squares.
    /// 
    /// This is the equivalent of an XOR of [Self::empty()] and 
    /// the universal bitboard.
    pub fn occupied(&self) -> Bitboard {
        self[Color::White] | self[Color::Black]
    }
}

/// provided input string must be a FEN string.
impl FromStr for PieceSet {
    type Err = DiogenesError;

    /// Parses a [`PieceSet`] from a FEN (Forsyth-Edwards Notation) input string.
    fn from_str(fen: &str) -> DiogenesResult<Self> {
        let mut val = Self([Bitboard::default(); 14]);
        fen.rsplit("/").zip(Rank::iter()).try_for_each(|(pieces, rank)| {
            let mut f: u32 = 0;
            for ch in pieces.chars() {
                match ch.to_digit(10) {
                    Some(skip) => {
                        if skip != 8 {
                            f += skip;
                        }
                    }
                    None => {
                        let file = File::from_u32(f)
                            .ok_or(DiogenesError::InvalidFenError{
                                fen: fen.to_string(),
                                reason: format!("invalid file: {f:?}"),
                            })?;
                        let color = match ch.is_ascii_uppercase() {
                            true => Color::White,
                            false => Color::Black,
                        };
                        let piece = Piece::try_from(ch)?;
                        let sq_idx: usize = board::try_index(file, rank).expect("invalid file + rank index");

                        let bb: u64 = 1 << sq_idx;
                        
                        val[piece] |= bb;
                        val[color] |= bb;
                        f += 1;
                    }
                }
            }
            Ok::<(), DiogenesError>(())
        })?;

        Ok(val)
    }
}

impl Index<Piece> for PieceSet {
    type Output = Bitboard;
    
    fn index(&self, piece: Piece) -> &Self::Output {
        match piece {
            Piece::WPawn => &self.0[2],
            Piece::WKnight => &self.0[3],
            Piece::WBishop => &self.0[4],
            Piece::WRook => &self.0[5],
            Piece::WQueen => &self.0[6],
            Piece::WKing => &self.0[7],
            Piece::BPawn => &self.0[8],
            Piece::BKnight => &self.0[9],
            Piece::BBishop => &self.0[10],
            Piece::BRook => &self.0[11],
            Piece::BQueen => &self.0[12],
            Piece::BKing => &self.0[13],
        }
    }
}

impl IndexMut<Piece> for PieceSet {
    fn index_mut(&mut self, piece: Piece) -> &mut Self::Output {
        match piece {
            Piece::WPawn => &mut self.0[2],
            Piece::WKnight => &mut self.0[3],
            Piece::WBishop => &mut self.0[4],
            Piece::WRook => &mut self.0[5],
            Piece::WQueen => &mut self.0[6],
            Piece::WKing => &mut self.0[7],
            Piece::BPawn => &mut self.0[8],
            Piece::BKnight => &mut self.0[9],
            Piece::BBishop => &mut self.0[10],
            Piece::BRook => &mut self.0[11],
            Piece::BQueen => &mut self.0[12],
            Piece::BKing => &mut self.0[13],
        }
    }
}

impl Index<Color> for PieceSet {
    type Output = Bitboard;

    fn index(&self, color: Color) -> &Self::Output {
        match color {
            Color::White => &self.0[0],
            Color::Black => &self.0[1],
        }
    }
}

impl IndexMut<Color> for PieceSet {
    fn index_mut(&mut self, color: Color) -> &mut Self::Output {
        match color {
            Color::White => &mut self.0[0],
            Color::Black => &mut self.0[1],
        }
    }
}

/// The fundamental construct for the engine that keeps track of the board representation.
#[derive(Clone, PartialEq, Eq)]
pub struct Position {
    /// Denormalized set of bitboards that contain set-wise representations
    /// of the chess pieces currently on the board.
    pieces: PieceSet,

    /// All currently empty squares.
    /// TODO: evaluate the tradeoff between [`PieceSet::empty`] and 
    /// keeping a separate value in [`Position`] that requires additional updates.
    empty: Bitboard,

    /// All currently occupied squares.
    /// TODO: see tradeoff above.
    occupied: Bitboard,
    
    /// The current number of full, completed moves.
    fullmove: i32,

    /// The current ply of the position, which is the number
    /// of half-moves. Should usually be double the full move
    /// count.
    /// See <https://www.chessprogramming.org/Ply>
    ply: i32,

    /// Whose turn it is to move - [`Color::White`] or [`Color::Black`]
    side_to_move: Color,

    /// The current open square to which a pawn can move via a capture
    /// en-passant. If there is no such square open in the current state
    /// this should be [`None`].
    /// 
    /// This will usually be [`None`].
    ep: Option<Square>,

    /// A bitfield representation of each side's ability to castle in 
    /// either direction.
    castling_rights: CastlingRights,

    /// The list of all made moves in the current game.
    history: VecDeque<Move>,
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for rank in Rank::iter().rev() {
            for file in File::iter() {
                let square = board::try_square(file, rank).unwrap();
                let ch = match self.piece(square) {
                    Some(p) => format!("{}", p),
                    None => String::from("."),
                };
                write!(s, "{} ", ch)?;
            }
            writeln!(s).unwrap();
        }

        write!(f, "{}", s)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fen())
    }
}

impl FromStr for Position {
    type Err = DiogenesError;

    /// Parse the provided FEN string and build bitboards that represent
    /// that position internally.
    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        Self::try_from_fen(fen)
    }
}

impl Default for Position {
    /// Returns a [`Position`] representing the starting position of a 
    /// chess board for a standard game of chess.
    /// 
    /// Since [`STARTING_FEN`] is well-formed, [`Self::try_from_fen`]
    /// is guaranteed not to panic.
    fn default() -> Self {
        Self::try_from_fen(STARTING_FEN).unwrap()
    }
}

impl Position {
    /// Retrieves the [`crate::piece::Piece`] at the specific index.
    /// Returns [`None`] if the square is empty.
    pub fn piece<S: Into<Square>>(&self, square: S) -> Option<Piece> {
        let mask = Bitboard::new_from_square_ref(&square.into());

        if (self.empty & mask).bool() {
            return None;
        }

        if (self.pieces[Color::White] & mask).bool() {
            (self.pieces[Piece::WPawn] & mask).bool().then_some(Piece::WPawn)
                .or((self.pieces[Piece::WKnight] & mask).bool().then_some(Piece::WKnight))
                .or((self.pieces[Piece::WBishop] & mask).bool().then_some(Piece::WBishop))
                .or((self.pieces[Piece::WRook] & mask).bool().then_some(Piece::WRook))
                .or((self.pieces[Piece::WQueen] & mask).bool().then_some(Piece::WQueen))
                .or((self.pieces[Piece::WKing] & mask).bool().then_some(Piece::WKing))
        } else {
            (self.pieces[Piece::BPawn] & mask).bool().then_some(Piece::BPawn)
                .or((self.pieces[Piece::BKnight] & mask).bool().then_some(Piece::BKnight))
                .or((self.pieces[Piece::BBishop] & mask).bool().then_some(Piece::BBishop))
                .or((self.pieces[Piece::BRook] & mask).bool().then_some(Piece::BRook))
                .or((self.pieces[Piece::BQueen] & mask).bool().then_some(Piece::BQueen))
                .or((self.pieces[Piece::BKing] & mask).bool().then_some(Piece::BKing))
        }
    }

    /// Serialize this position to a FEN string.
    fn fen(&self) -> String {
        let mut pieces = String::new();

        for rank in Rank::iter().rev() {
            let mut empty = 0;
            for file in File::iter() {
                if let Some(piece) = self.piece(board::try_square(file, rank).unwrap()) {
                    if empty > 0 {
                        pieces.push_str(&empty.to_string());
                        empty = 0;
                    }
                    pieces.push_str(piece.to_string().as_str());
                } else {
                    empty += 1;
                }
            }

            if empty > 0 {
                pieces.push_str(empty.to_string().as_str());
            }

            pieces.push('/');
        }
        // Remove the trailing slash from the constructed string
        pieces.pop();

        let active_color: String;
        match self.side_to_move {
            Color::White => {active_color = "w".to_string()}
            Color::Black => {active_color = "b".to_string()}
        }

        let ep: String = self.ep.map(|sq| sq.to_string()).unwrap_or(String::from("-"));
        let cr: String = self.castling_rights.to_string();
        let ply: String = self.ply.to_string();
        let fullmove = self.fullmove.to_string();

        [pieces, active_color, cr, ep, ply, fullmove].join(" ")
    }

    /// Deserialize a position from a FEN string.
    fn try_from_fen(fen: &str) -> DiogenesResult<Position> {
        let fields: Vec<&str> = fen.split(" ").collect();
        
        // Read pieces from the first component of the FEN
        let pieces = PieceSet::from_str(fields[0])?;
        let occupied = pieces.occupied();
        let empty = pieces.empty();

        // Read position metadata
        let active = Color::from_str(fields[1])?;
        let castling_rights = CastlingRights::from_str(fields[2])?;
        let ep_square = match fields[3] {
            "-" => None,
            sq => Some(Square::from_str(sq)?),
        };
        let ply: i32 = fields[4].parse::<i32>()?;
        let fullmove: i32 = fields[5].parse::<i32>()?;

        Ok(Position{
            empty,
            occupied,
            ep: ep_square,
            castling_rights,
            side_to_move: active,
            pieces,
            ply,
            fullmove,
            history: VecDeque::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::rstest;

    use crate::bitboard::Bitboard;
    use crate::piece::Piece;
    use crate::position::{Position, STARTING_FEN};
    use crate::square::Square;

    use super::PieceSet;

    #[test]
    fn test_get_piece() {
        let pos = Position::default();
        assert_eq!(pos.piece(Square::A1), Some(Piece::WRook), "White rook should be at A1");
        assert_eq!(pos.piece(Square::B1), Some(Piece::WKnight));
        assert_eq!(pos.piece(Square::H1), Some(Piece::WRook), "White rook should be at A1");
    }

    #[test]
    fn test_build_position_from_starting_fen() {
        let res = Position::from_str(STARTING_FEN);
        assert!(res.is_ok());
        let pos = res.unwrap();
        
        let s = pos.to_string();
        assert_eq!(s, STARTING_FEN);
    }

    #[test]
    fn test_parse_piece_set() {
        let white_pieces = Bitboard::new(0xFFFF);
        let black_pieces = Bitboard::new(0xFFFF).flip_vertical();
        let white_pawns = Bitboard::new(0xFF00);
        let white_knights = Bitboard::new(0x42);
        let white_bishops = Bitboard::new(0x24);
        let white_rooks = Bitboard::new(0x81);
        let white_queens = Bitboard::new(0x08);
        let white_kings = Bitboard::new(0x10);
        let black_pawns = Bitboard::new(0xFF00).flip_vertical();
        let black_knights = Bitboard::new(0x42).flip_vertical();
        let black_bishops = Bitboard::new(0x24).flip_vertical();
        let black_rooks = Bitboard::new(0x81).flip_vertical();
        let black_queens = Bitboard::new(0x08).flip_vertical();
        let black_kings = Bitboard::new(0x10).flip_vertical();

        let actual = PieceSet::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        let expected: PieceSet = PieceSet([
            white_pieces,
            black_pieces,
            white_pawns,
            white_knights,
            white_bishops,
            white_rooks,
            white_queens,
            white_kings,
            black_pawns,
            black_knights,
            black_bishops,
            black_rooks,
            black_queens,
            black_kings,
        ]);

        assert!(actual.is_ok(), "expected OK but received {actual:?}");
        let actual = actual.unwrap();
        assert_eq!(expected, actual, "expected {expected:?} but received {actual:?}");
    }

    #[rstest]
    #[case::one("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1")]
    #[case::two("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2")]
    #[case::three("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2")]
    #[case::four("4k3/8/8/8/8/8/4P3/4K3 w - - 5 39")]
    #[case::five("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1")]
    fn test_build_position_from_random_fen(
        #[case] fen: &str,
    ) {
        let pos = Position::from_str(fen);
        assert!(pos.is_ok());
        let pos = pos.unwrap();

        let s = pos.to_string();
        assert_eq!(s, fen.to_string());
    }

}