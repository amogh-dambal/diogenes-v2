use core::fmt;
use std::str::FromStr;
use num_derive::FromPrimitive;
use num_derive::ToPrimitive;
use strum_macros::EnumCount;
use strum_macros::EnumIter;

/// Uses Little Endian Rank-File square mapping
/// to translate indices in a 64-bit integer to 
/// a square on the chess board. In this mapping,
/// LSB points to A1 and the MSB points to H8. 
#[derive(Clone, Copy, Debug, Default, FromPrimitive, PartialEq, ToPrimitive, EnumCount, EnumIter)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
    #[default]
    NONE
}

impl FromStr for Square {
    type Err = &'static str;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // "-" is the only special case for a square string.
        // Usually used to configure the en-passant square.
        if s.eq("-") {
            return Ok(Square::NONE);
        }
        if s.len() != 2 {
            return Err("Invalid string for square");
        }
        
        // Create an iterator over the received string
        // so we can go one character at a time.
        let mut chars = s.chars();

        // Get the 'file' character
        let offset: u32;
        match chars.next() {
            Some(ch) => {
                let file = ch.to_ascii_lowercase();
                match file {
                    'a' => {offset = 0}
                    'b' => {offset = 1}
                    'c' => {offset = 2}
                    'd' => {offset = 3}
                    'e' => {offset = 4}
                    'f' => {offset = 5}
                    'g' => {offset = 6}
                    'h' => {offset = 7}
                    _ => {
                        return Err("Unknown file character!")
                    }
                }
            }
            None => {
                return Err("Invalid square string - could not find file character");
            }
        }
        
        // Get the 'rank' character
        let rank: u32;
        match chars.next() {
            Some(ch) => {
                match ch.to_digit(10) {
                    Some(val) => {rank = val}
                    None => {return Err("Invalid rank character")}
                }
            }
            None => {
                return Err("Invalid square string - could not find rank character")
            }
        }
        
        // Compute the "index" into a 64 square board using
        // the obtained file and rank.
        let sq_val: u32 = offset + (rank - 1) * 8;
        
        match num_traits::FromPrimitive::from_u32(sq_val) {
            Some(square) => {
                return Ok(square);
            }
            None => {
                return Err("Invalid square value!");
            }
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Square::NONE {
            return write!(f, "-");
        }
        else {
            let s = format!("{:?}", &self).to_ascii_lowercase();
            return write!(f, "{}", s);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{iter::zip, str::FromStr};

    use strum::IntoEnumIterator;

    use crate::square::Square;

    #[test]
    fn test_from_str() {
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
    fn test_from_str_none() {
        let res = Square::from_str("-");
        assert!(res.is_ok());
        let sq = res.unwrap();
        assert_eq!(sq, Square::NONE);
    }

    #[test]
    fn test_from_str_invalid() {
        // Not a square string - something random
        let res = Square::from_str("invalidsquare");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, "Invalid string for square")
    }
}
