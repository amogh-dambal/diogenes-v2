use super::types::Color;

#[derive(Debug)]
pub struct CastlingRights {
    /**
     * X X X X K Q k q
     * Pulled from FEN notation. Top 4 bits are wasted, unfortunately
     */
    data: u8,
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
    #![allow(arithmetic_overflow)]
    pub fn from_str(s: &str) -> CastlingRights {
        if s.len() > 4 {
            panic!("Invalid input string!")
        }
        
        let mut cr: CastlingRights = CastlingRights{data: 0};
        if s.eq("-") {
            return cr;
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
            cr.data |= QUEENSIDES[Color::White as usize];
        }
        
        return cr;
    }

    fn kingside(&self, c: Color) -> bool {
        return (self.data & KINGSIDES[c as usize]) != 0;
    }

    fn queenside(&self, c: Color) -> bool {
        return (self.data & QUEENSIDES[c as usize]) != 0;
    }
}

#[cfg(test)]
mod tests {
    use super::CastlingRights;
    use super::Color;

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
            }
        ];

        for tc in testcases {
            let cr: CastlingRights = CastlingRights::from_str(tc.input);
            assert_eq!(tc.black_kingside, cr.kingside(Color::Black));
            assert_eq!(tc.black_queenside, cr.queenside(Color::Black));
            assert_eq!(tc.white_kingside, cr.kingside(Color::White));
            assert_eq!(tc.white_queenside, cr.queenside(Color::White));
        }

    }
}