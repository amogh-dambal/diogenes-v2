use std::{fmt::Display, str::FromStr};

use bitfield_struct::bitfield;

use crate::error::DiogenesError;

/// A bitfield structure which encodes information about which side
/// can castle in which direction.
#[bitfield(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CastlingRights {
    /// Set to true if Black can castle queenside
    #[bits(default = true)]
    black_queenside: bool,

    /// Set to true if Black can castle kingside
    #[bits(default = true)]
    black_kingside: bool,

    /// Set to true if White can castle queenside
    #[bits(default = true)]
    white_queenside: bool,

    /// Set to true if White can castle kingside
    #[bits(default = true)]
    white_kingside: bool,

    /// Unused padding bits
    #[bits(4)]
    __: usize,
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

        match s {
            "-" => Ok(Self::from_bits(0)),
            data => {
                let castling_rights =
                    data.chars()
                        .try_fold(CastlingRights::from_bits(0), |cr, ch| match ch {
                            'K' => Ok(cr.with_white_kingside(true)),
                            'Q' => Ok(cr.with_white_queenside(true)),
                            'k' => Ok(cr.with_black_kingside(true)),
                            'q' => Ok(cr.with_black_queenside(true)),
                            ch => Err(DiogenesError::InvalidFenError {
                                fen: s.to_string(),
                                reason: format!(
                                    "invalid char: {ch:?} in FEN string for castling rights"
                                ),
                            }),
                        })?;

                Ok(castling_rights)
            }
        }
    }
}

impl Display for CastlingRights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 0 {
            return write!(f, "-");
        }

        let mut s = String::new();
        if self.white_kingside() {
            s += "K";
        }

        if self.white_queenside() {
            s += "Q";
        }

        if self.black_kingside() {
            s += "k";
        }

        if self.black_queenside() {
            s += "q";
        }

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
        assert_eq!(wq, cr.white_queenside())
    }
}
