use std::str::FromStr;
use num_derive::FromPrimitive;
use num_derive::ToPrimitive;
use num_traits::ToPrimitive;

use crate::bitboard::Bitboard;
use crate::board::File;
use crate::board::Rank;
use crate::error::DiogenesError;

/// Uses Little Endian Rank-File square mapping
/// to translate indices in a 64-bit integer to 
/// a square on the chess board. In this mapping,
/// LSB points to A1 and the MSB points to H8. 
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    FromPrimitive,
    ToPrimitive,
    strum::Display,
    strum::EnumCount,
    strum::EnumIter,
)]
#[strum(serialize_all = "lowercase")]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl From<Square> for Bitboard {
    fn from(sq: Square) -> Self {
        let i = sq.to_u64().unwrap();
        Self::new(1 << i)
    }
}

impl Square {
    /// Get the file of the given square from [`File::A`] to [`File::H`]
    pub fn file(&self) -> File {
        match self {
            Self::A1 | Self::A2 | Self::A3 | Self::A4 | Self::A5 | Self::A6 | Self::A7 | Self::A8 => File::A,
            Self::B1 | Self::B2 | Self::B3 | Self::B4 | Self::B5 | Self::B6 | Self::B7 | Self::B8 => File::B,
            Self::C1 | Self::C2 | Self::C3 | Self::C4 | Self::C5 | Self::C6 | Self::C7 | Self::C8 => File::C,
            Self::D1 | Self::D2 | Self::D3 | Self::D4 | Self::D5 | Self::D6 | Self::D7 | Self::D8 => File::D,
            Self::E1 | Self::E2 | Self::E3 | Self::E4 | Self::E5 | Self::E6 | Self::E7 | Self::E8 => File::E,
            Self::F1 | Self::F2 | Self::F3 | Self::F4 | Self::F5 | Self::F6 | Self::F7 | Self::F8 => File::F,
            Self::G1 | Self::G2 | Self::G3 | Self::G4 | Self::G5 | Self::G6 | Self::G7 | Self::G8 => File::G,
            Self::H1 | Self::H2 | Self::H3 | Self::H4 | Self::H5 | Self::H6 | Self::H7 | Self::H8 => File::H,
        }
    }

    /// Get the rank (column) of the square from [`Rank::ONE`] to [`Rank::EIGHT`]
    pub fn rank(&self) -> Rank {
        match self {
            Self::A1 | Self::B1 | Self::C1 | Self::D1 | Self::E1 | Self::F1 | Self::G1 | Self::H1 => Rank::ONE,
            Self::A2 | Self::B2 | Self::C2 | Self::D2 | Self::E2 | Self::F2 | Self::G2 | Self::H2 => Rank::TWO,
            Self::A3 | Self::B3 | Self::C3 | Self::D3 | Self::E3 | Self::F3 | Self::G3 | Self::H3 => Rank::THREE,
            Self::A4 | Self::B4 | Self::C4 | Self::D4 | Self::E4 | Self::F4 | Self::G4 | Self::H4 => Rank::FOUR,
            Self::A5 | Self::B5 | Self::C5 | Self::D5 | Self::E5 | Self::F5 | Self::G5 | Self::H5 => Rank::FIVE,
            Self::A6 | Self::B6 | Self::C6 | Self::D6 | Self::E6 | Self::F6 | Self::G6 | Self::H6 => Rank::SIX,
            Self::A7 | Self::B7 | Self::C7 | Self::D7 | Self::E7 | Self::F7 | Self::G7 | Self::H7 => Rank::SEVEN,
            Self::A8 | Self::B8 | Self::C8 | Self::D8 | Self::E8 | Self::F8 | Self::G8 | Self::H8 => Rank::EIGHT,
        }
    }
}


impl FromStr for Square {
    type Err = DiogenesError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(DiogenesError::InvalidSquareError(s.to_string()));
        }

        let s = s.to_uppercase();
        match s.as_str() {
            "A1" => Ok(Self::A1), "A2" => Ok(Self::A2), "A3" => Ok(Self::A3), "A4" => Ok(Self::A4), "A5" => Ok(Self::A5), "A6" => Ok(Self::A6), "A7" => Ok(Self::A7), "A8" => Ok(Self::A8),
            "B1" => Ok(Self::B1), "B2" => Ok(Self::B2), "B3" => Ok(Self::B3), "B4" => Ok(Self::B4), "B5" => Ok(Self::B5), "B6" => Ok(Self::B6), "B7" => Ok(Self::B7), "B8" => Ok(Self::B8),
            "C1" => Ok(Self::C1), "C2" => Ok(Self::C2), "C3" => Ok(Self::C3), "C4" => Ok(Self::C4), "C5" => Ok(Self::C5), "C6" => Ok(Self::C6), "C7" => Ok(Self::C7), "C8" => Ok(Self::C8),
            "D1" => Ok(Self::D1), "D2" => Ok(Self::D2), "D3" => Ok(Self::D3), "D4" => Ok(Self::D4), "D5" => Ok(Self::D5), "D6" => Ok(Self::D6), "D7" => Ok(Self::D7), "D8" => Ok(Self::D8),
            "E1" => Ok(Self::E1), "E2" => Ok(Self::E2), "E3" => Ok(Self::E3), "E4" => Ok(Self::E4), "E5" => Ok(Self::E5), "E6" => Ok(Self::E6), "E7" => Ok(Self::E7), "E8" => Ok(Self::E8),
            "F1" => Ok(Self::F1), "F2" => Ok(Self::F2), "F3" => Ok(Self::F3), "F4" => Ok(Self::F4), "F5" => Ok(Self::F5), "F6" => Ok(Self::F6), "F7" => Ok(Self::F7), "F8" => Ok(Self::F8),
            "G1" => Ok(Self::G1), "G2" => Ok(Self::G2), "G3" => Ok(Self::G3), "G4" => Ok(Self::G4), "G5" => Ok(Self::G5), "G6" => Ok(Self::G6), "G7" => Ok(Self::G7), "G8" => Ok(Self::G8),
            "H1" => Ok(Self::H1), "H2" => Ok(Self::H2), "H3" => Ok(Self::H3), "H4" => Ok(Self::H4), "H5" => Ok(Self::H5), "H6" => Ok(Self::H6), "H7" => Ok(Self::H7), "H8" => Ok(Self::H8),
            _ => Err(DiogenesError::InvalidSquareError(s))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{iter::zip, str::FromStr};

    use strum::IntoEnumIterator;

    use crate::{bitboard::Bitboard, error::DiogenesError, square::Square};

    #[test]
    fn test_square_to_bitboard() {
        for (i, sq) in Square::iter().enumerate() {
            let expected = Bitboard::new(1 << i);
            let actual: Bitboard = sq.into();
        }
    }

    #[test]
    fn test_square_to_from_str() {
        let square_strs:  [&str; 64] = [
            "a1","b1","c1","d1","e1","f1","g1","h1",
            "a2","b2","c2","d2","e2","f2","g2","h2",
            "a3","b3","c3","d3","e3","f3","g3","h3",
            "a4","b4","c4","d4","e4","f4","g4","h4",
            "a5","b5","c5","d5","e5","f5","g5","h5",
            "a6","b6","c6","d6","e6","f6","g6","h6",
            "a7","b7","c7","d7","e7","f7","g7","h7",
            "a8","b8","c8","d8","e8","f8","g8","h8",
        ];

        for (s, expected_sq) in zip(square_strs, Square::iter()) {
            let res = Square::from_str(s);
            assert!(res.is_ok());
            let actual_sq = res.unwrap();
            assert_eq!(expected_sq, actual_sq);
        }
    }

    #[test]
    fn test_from_str_invalid() {
        let res = Square::from_str("invalidsquare");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, DiogenesError::InvalidSquareError("invalidsquare".into()))
    }
}
