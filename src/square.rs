use core::fmt;
use num_derive::FromPrimitive;
use num_derive::ToPrimitive;
use num_traits::ToPrimitive;

use crate::board::File;
use crate::board::Rank;


/// Uses Little Endian Rank-File square mapping
/// to translate indices in a 64-bit integer to 
/// a square on the chess board. In this mapping,
/// LSB points to A1 and the MSB points to H8. 
#[derive(Debug, Default, FromPrimitive, PartialEq, ToPrimitive)]
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


impl Square {
    pub fn index(file: File, rank: Rank) -> usize {
        let f: u64;
        match file.to_u64() {
            Some(val) => {f = val;}
            None => panic!("Invalid file!")
        }

        let r: u64;
        match rank.to_u64() {
            Some(val) => {r = val;}
            None => panic!("Invalid file!")
        }

        return ((8 * r) + f) as usize;
    }

    pub fn from_str(s: &str) -> Square {
        if s.eq("-") || s.len() != 2 {
            return Square::NONE;
        }

        let file: char = s.chars().nth(0).expect("Invalid file character").to_ascii_lowercase();
        let offset: u32;
        match file {
            'a' => {offset = 0}
            'b' => {offset = 1}
            'c' => {offset = 2}
            'd' => {offset = 3}
            'e' => {offset = 4}
            'f' => {offset = 5}
            'g' => {offset = 6}
            'h' => {offset = 7}
            _ => {panic!("Unknown file character! Invalid FEN!")}
        }

        let rank: u32 = s.chars()
            .nth(1)
            .unwrap()
            .to_digit(10)
            .unwrap();

        let sq: u32 = offset + (rank - 1) * 8;
        
        return num_traits::FromPrimitive::from_u32(sq).expect("Invalid square value");
    }
}

#[cfg(test)]
mod tests {
    use crate::square::Square;

    #[test]
    fn test_from_str() {
        assert_eq!(Square::D5, Square::from_str("d5"));
        assert_eq!(Square::A3, Square::from_str("a3"));
        assert_eq!(Square::F5, Square::from_str("f5"));
        assert_eq!(Square::B8, Square::from_str("b8"));
    }

    #[test]
    fn test_from_str_none() {
        let s = "-";
        let sq = Square::from_str(s);
        assert_eq!(sq, Square::NONE);
    }

    #[test]
    fn test_from_str_invalid() {
        let s = "NOTAValidSquare";
        let sq = Square::from_str(s);
        assert_eq!(sq, Square::NONE);
    }
}