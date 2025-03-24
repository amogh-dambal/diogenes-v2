use crate::error::DiogenesError;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIter,
    strum::EnumString,
    strum::IntoStaticStr,
)]
#[strum(
    parse_err_fn = parse_err_fn,
    parse_err_ty = DiogenesError,
)]
pub enum Color {
    #[strum(serialize = "w")]
    White,
    #[strum(serialize = "b")]
    Black,
}

fn parse_err_fn(s: &str) -> DiogenesError {
    DiogenesError::InvalidColor(s.to_string())
}
