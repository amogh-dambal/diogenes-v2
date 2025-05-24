use std::{fmt::Display, str::FromStr};

use bitbybit::bitfield;

use crate::error::DiogenesError;

/// A structure which encodes information about which side
/// can castle in which direction.
///
/// A [`CastlingRights`] struct is backed by a single [`u8`]. Each
/// of the four lower bits encodes a particular color's castling
/// ability for a direction, i.e.
/// X X X X K Q k q
/// ^M            ^L
///
/// where M is the most significant bit and L is the least. As per
/// chess engine/FEN standardization, Black is represented using lowercase
/// letters and White using uppercase.
/// The top four bits are unused.
#[bitfield(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CastlingRights {
    #[bits(4..=7)]
    _unused: u4,
    #[bit(3, rw)]
    white_kingside: bool,
    #[bit(2, rw)]
    white_queenside: bool,
    #[bit(1, rw)]
    black_kingside: bool,
    #[bit(0, rw)]
    black_queenside: bool,
}

/// By default, we assume that all sides can castle anywhere.
impl Default for CastlingRights {
    fn default() -> Self {
        Self::new_with_raw_value(0b00001111)
    }
}

impl FromStr for CastlingRights {
    type Err = DiogenesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 4 {
            return Err(DiogenesError::InvalidFenError {
                fen: s.to_string(),
                reason: format!(
                    "castling rights field has length {} which exceeds max value of 4",
                    s.len()
                ),
            });
        }

        // TODO: This should handle bad FEN strings like QQq
        match s {
            "-" => Ok(CastlingRights::ZERO),
            data => {
                let cr = data
                    .chars()
                    .try_fold(CastlingRights::ZERO, |cr, ch| match ch {
                        'K' => Ok(cr.with_white_kingside(true)),
                        'Q' => Ok(cr.with_white_queenside(true)),
                        'k' => Ok(cr.with_black_kingside(true)),
                        'q' => Ok(cr.with_black_queenside(true)),
                        c => Err(DiogenesError::InvalidFenError {
                            fen: s.to_string(),
                            reason: format!("invalid character {c} in castling rights FEN",),
                        }),
                    })?;
                Ok(cr)
            }
        }
    }
}

impl Display for CastlingRights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.raw_value == 0 {
            return write!(f, "-");
        }

        let mut s = String::with_capacity(4);
        s.push_str(self.white_kingside().then_some("K").unwrap_or(""));
        s.push_str(self.white_queenside().then_some("Q").unwrap_or(""));
        s.push_str(self.black_kingside().then_some("k").unwrap_or(""));
        s.push_str(self.black_queenside().then_some("q").unwrap_or(""));

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::rstest;

    use crate::castling::CastlingRights;

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
        assert_eq!(bk, cr.black_kingside());
        assert_eq!(bq, cr.black_queenside());
        assert_eq!(wk, cr.white_kingside());
        assert_eq!(wq, cr.white_queenside());
    }
}
