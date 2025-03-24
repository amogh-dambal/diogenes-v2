use std::{fmt::Debug, num::ParseIntError};

use thiserror::Error;

/// TODO: Add better, more descriptive errors
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DiogenesError {
    #[error("{0:?} is not a valid File")]
    InvalidFileError(String),
    #[error("{0:?} is not a valid Rank")]
    InvalidRankError(String),
    #[error("{fen:?} is not a valid FEN string: {reason:?}")]
    InvalidFenError{
        fen: String,
        reason: String,
    },
    #[error("{0} is not a valid square")]
    InvalidSquareError(String),
    #[error("{0} is not a valid piece")]
    InvalidPieceError(String),
    #[error("{0} does not represent a valid color, must be either 0 (White) or 1 (Black)")]
    InvalidColor(String),
}

impl From<ParseIntError> for DiogenesError {
    fn from(err: ParseIntError) -> Self {
        DiogenesError::InvalidFenError { fen: String::from(""), reason: err.to_string() }
    }
}

pub type DiogenesResult<T> = anyhow::Result<T, DiogenesError>;