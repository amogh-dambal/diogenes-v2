use std::{fmt::Display, ops::Index, str::FromStr};

use crate::{color::Color, error::DiogenesError};

const KINGSIDES: [u8; 2] = [0b00001000, 0b00000010];
const QUEENSIDES: [u8; 2] = [0b00000100, 0b00000001];

impl Index<Color> for [u8; 2] {
    type Output = u8;

    fn index(&self, color: Color) -> &Self::Output {
        match color {
            Color::White => &self[0],
            Color::Black => &self[1],
        }
    }
}

/// A bitfield structure which encodes information about which side
/// can castle in which direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CastlingRights {
    /// An 8-bit integer which encodes information about which side can
    /// castle in which direction.
    /// 
    /// (Lowercase is Black, uppercase is White)
    /// 
    /// X X X X K Q k q
    /// ^             ^
    /// MSB (7)       LSB (0)
    /// 
    /// This encoding can be serialized from a component of a FEN string.
    data: u8,
}

impl FromStr for CastlingRights {
    type Err = DiogenesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 4 {
            return Err(DiogenesError::InvalidFenError { 
                fen: s.to_string(),
                reason: format!("castling rights field has length {} which exceeds max value of 4", s.len()) 
            });
        }
        
        // TODO: Make this more robust to strings like Qqq
        let mut cr: CastlingRights = CastlingRights{data: 0};
        if s.eq("-") {
            return Ok(cr);
        }

        if s.contains("K") {
            cr.data |= KINGSIDES[Color::White];
        }
        if s.contains("Q") {
            cr.data |= QUEENSIDES[Color::White];
        }
        if s.contains("k") {
            cr.data |= KINGSIDES[Color::Black];
        }
        if s.contains("q") {
            cr.data |= QUEENSIDES[Color::Black];
        }
        
        Ok(cr)
    }
}

impl Display for CastlingRights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.data == 0 {
            return write!(f, "-");
        }

        let mut s = String::new();
        if self.kingside(Color::White) {
            s += "K";
        }

        if self.queenside(Color::White) {
            s += "Q";
        }

        if self.kingside(Color::Black) {
            s += "k";
        }

        if self.queenside(Color::Black) {
            s += "q";
        }

        write!(f, "{}", s)
    }
}

impl Default for CastlingRights {
    fn default() -> Self {
        CastlingRights{
            data: 0b00001111
        }
    }
}

impl CastlingRights {
    /// Returns true if the provided color can castle kingsides.
    pub fn kingside(&self, c: Color) -> bool {
        (self.data & KINGSIDES[c]) != 0
    }

    /// Returns true if the provided color can castle queensides.
    pub fn queenside(&self, c: Color) -> bool {
        (self.data & QUEENSIDES[c]) != 0
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::rstest;

    use crate::castling::CastlingRights;
    use crate::color::Color;

    #[rstest]
    #[case::all("KQkq", true, true, true, true)]
    #[case::only_white_all_sides("KQ", true, true, false, false)]
    #[case::only_black_all_sides("kq", false, false, true, true)]
    #[case::only_white_kingside("K", true, false, false, false)]
    #[case::only_white_queenside("Q", false, true, false, false)]
    #[case::only_black_kingside("k", false, false, true, false)]
    #[case::only_black_queenside("q", false, false, false, true)]
    #[case::only_kingside("Kk", true, false, true, false)]
    #[case::only_queenside("Qq", false, true, false, true)]
    fn test_from_str(
        #[case] input: &str,
        #[case] wk: bool,
        #[case] wq: bool,
        #[case] bk: bool,
        #[case] bq: bool,
    ) {
        let cr = CastlingRights::from_str(input);
        assert!(cr.is_ok(), "expected OK, received Err {cr:?}");
        
        let cr = cr.unwrap();
        assert_eq!(bk, cr.kingside(Color::Black));
        assert_eq!(bq, cr.queenside(Color::Black));
        assert_eq!(wk, cr.kingside(Color::White));
        assert_eq!(wq, cr.queenside(Color::White))
    }
}