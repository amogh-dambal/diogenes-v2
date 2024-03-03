use std::{fmt::Display, str::FromStr};

use crate::color::Color;

#[derive(Debug)]
pub struct CastlingRights {
    /**
     * X X X X K Q k q
     * Pulled from FEN notation. Top 4 bits are wasted, unfortunately
     */
    data: u8,
}

impl FromStr for CastlingRights {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 4 {
            return Err("Invalid input string!");
        }
        
        let mut cr: CastlingRights = CastlingRights{data: 0};
        if s.eq("-") {
            return Ok(cr);
        }

        if s.contains("K") {
            cr.data |= KINGSIDES[Color::White as usize];
        }
        if s.contains("Q") {
            cr.data |= QUEENSIDES[Color::White as usize];
        }
        if s.contains("k") {
            cr.data |= KINGSIDES[Color::Black as usize];
        }
        if s.contains("q") {
            cr.data |= QUEENSIDES[Color::Black as usize];
        }
        return Ok(cr);
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

        return write!(f, "{}", s);
    }
}

impl Default for CastlingRights {
    fn default() -> Self {
        CastlingRights{
            data: 0b00001111
        }
    }
}

const KINGSIDES: [u8; 2] = [0b00001000, 0b00000010];
const QUEENSIDES: [u8; 2] = [0b00000100, 0b00000001];

impl CastlingRights {

    fn kingside(&self, c: Color) -> bool {
        return (self.data & KINGSIDES[c as usize]) != 0;
    }

    fn queenside(&self, c: Color) -> bool {
        return (self.data & QUEENSIDES[c as usize]) != 0;
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::castling::CastlingRights;
    use crate::color::Color;

    struct Testcase {
        input: &'static str,
        black_kingside: bool,
        black_queenside: bool,
        white_kingside: bool,
        white_queenside: bool,
    }

    #[test]
    fn test_from_str() {
        let testcases = [
            Testcase{
                black_kingside: false,
                black_queenside: false,
                white_kingside: true,
                white_queenside: true,
                input: "KQ",
            },
            Testcase{
                black_kingside: true,
                black_queenside: true,
                white_kingside: true,
                white_queenside: true,
                input: "KQkq",
            }
        ];

        for tc in testcases {
            let res = CastlingRights::from_str(tc.input);
            assert!(res.is_ok());
            let cr = res.unwrap();            
            assert_eq!(tc.black_kingside, cr.kingside(Color::Black));
            assert_eq!(tc.black_queenside, cr.queenside(Color::Black));
            assert_eq!(tc.white_kingside, cr.kingside(Color::White));
            assert_eq!(tc.white_queenside, cr.queenside(Color::White));
        }

    }
}