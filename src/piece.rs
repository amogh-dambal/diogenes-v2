use std::{fmt::Display, ops::Index, slice::SliceIndex, str::FromStr};

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

use crate::{bitboard::{Bitboard}, color::Color, error::DiogenesError};

#[derive(Clone, Copy, Debug, PartialEq, strum::EnumIter, strum::EnumCount, FromPrimitive, ToPrimitive)]
pub enum Piece {
    WPawn,
    WKnight,
    WBishop,
    WRook,
    WQueen,
    WKing,
    BPawn,
    BKnight,
    BBishop,
    BRook,
    BQueen,
    BKing,
}

impl From<Piece> for usize {
    fn from(piece: Piece) -> Self {
        match piece {
            Piece::WPawn => 0,
            Piece::WKnight => 1,
            Piece::WBishop => 2,
            Piece::WRook => 3,
            Piece::WQueen => 4,
            Piece::WKing => 5,
            Piece::BPawn => 6,
            Piece::BKnight => 7,
            Piece::BBishop => 8,
            Piece::BRook => 9,
            Piece::BQueen => 10,
            Piece::BKing => 11,
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = DiogenesError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'p' => Ok(Piece::BPawn),
            'n' => Ok(Piece::BKnight),
            'b' => Ok(Piece::BBishop),
            'r' => Ok(Piece::BRook),
            'q' => Ok(Piece::BQueen),
            'k' => Ok(Piece::BKing),
            'P' => Ok(Piece::WPawn),
            'N' => Ok(Piece::WKnight),
            'B' => Ok(Piece::WBishop),
            'R' => Ok(Piece::WRook),
            'Q' => Ok(Piece::WQueen),
            'K' => Ok(Piece::WKing),
            val => Err(DiogenesError::InvalidPieceError(val.to_string()))
        }
    }
}

impl FromStr for Piece {
    type Err = DiogenesError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "p" => Ok(Piece::BPawn),
            "n" => Ok(Piece::BKnight),
            "b" => Ok(Piece::BBishop),
            "r" => Ok(Piece::BRook),
            "q" => Ok(Piece::BQueen),
            "k" => Ok(Piece::BKing),
            "P" => Ok(Piece::WPawn),
            "N" => Ok(Piece::WKnight),
            "B" => Ok(Piece::WBishop),
            "R" => Ok(Piece::WRook),
            "Q" => Ok(Piece::WQueen),
            "K" => Ok(Piece::WKing),
            val => Err(DiogenesError::InvalidPieceError(val.into()))
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::BPawn => write!(f, "p"),
            Piece::BKnight => write!(f, "n"),
            Piece::BBishop => write!(f, "b"),
            Piece::BRook => write!(f, "r"),
            Piece::BQueen => write!(f, "q"),
            Piece::BKing => write!(f, "k"),
            Piece::WPawn => write!(f, "P"),
            Piece::WKnight => write!(f, "N"),
            Piece::WBishop => write!(f, "B"),
            Piece::WRook => write!(f, "R"),
            Piece::WQueen => write!(f, "Q"),
            Piece::WKing => write!(f, "K"),
        }
    }
}

impl Piece {
    pub const UNIQUE: usize = 6;

    pub fn color(&self) -> Color {
        match self {
            Self::WPawn | 
            Self::WKnight | 
            Self::WBishop | 
            Self::WRook | 
            Self::WQueen | 
            Self::WKing => Color::White,
            Self::BPawn | 
            Self::BKnight | 
            Self::BBishop | 
            Self::BRook | 
            Self::BQueen | 
            Self::BKing => Color::Black,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Piece;
    #[test]
    fn test_str() {
        let piece_strs: [&str; 12] = [
            "P", "N", "B", "R", "Q", "K",
            "p", "n", "b", "r", "q", "k",
        ];
        let pieces: [Piece; 12] = [
            Piece::WPawn, Piece::WKnight, Piece::WBishop, Piece::WRook, Piece::WQueen, Piece::WKing,
            Piece::BPawn, Piece::BKnight, Piece::BBishop, Piece::BRook, Piece::BQueen, Piece::BKing,
        ];

        for (s, p) in piece_strs.iter().zip(pieces) {
            assert_eq!(p.to_string(), s.to_string());

            let res = Piece::from_str(s);
            assert!(res.is_ok());

            let val = res.unwrap();
            assert_eq!(val, p);
        }
    }
}