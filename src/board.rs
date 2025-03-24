#![allow(clippy::upper_case_acronyms)]
use std::fmt::Debug;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::{
    error::{DiogenesError, DiogenesResult},
    square::Square,
};

#[derive(
    Clone,
    Copy,
    Debug,
    num_derive::FromPrimitive,
    num_derive::ToPrimitive,
    strum::EnumIter,
    strum::Display,
    strum::EnumString,
)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Clone, Copy, Debug, strum::EnumIter, FromPrimitive, ToPrimitive)]
pub enum Rank {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
}

pub fn try_index(file: File, rank: Rank) -> DiogenesResult<usize> {
    let f: usize = file
        .to_usize()
        .ok_or(DiogenesError::InvalidFileError(format!("{file:#?}")))?;
    let r: usize = rank
        .to_usize()
        .ok_or(DiogenesError::InvalidRankError(format!("{file:#?}")))?;

    Ok((8 * r) + f)
}

pub fn index<T>(file: T, rank: T) -> usize
where
    T: Into<usize>,
{
    let f: usize = file.into();
    let r: usize = rank.into();
    (8 * r) + f
}

pub fn try_square(file: File, rank: Rank) -> Option<Square> {
    Square::from_usize(try_index(file, rank).ok()?)
}

pub fn square<T>(file: T, rank: T) -> Square
where
    T: Into<usize>,
{
    Square::from_usize(index(file.into(), rank.into())).unwrap()
}

// Bitboard constants representing specific parts of the chess board
// These constants are defined using LERF (little-endian rank-file) mapping
pub const A_FILE: u64 = 0x0101010101010101;
pub const AB_FILE: u64 = 0x0303030303030303;
pub const GH_FILE: u64 = 0xc0c0c0c0c0c0c0c0;
pub const H_FILE: u64 = 0x8080808080808080;
pub const FIRST_RANK: u64 = 0x00000000000000ff;
pub const EIGHTH_RANK: u64 = 0xff00000000000000;
pub const A1_H8_DIAGONAL: u64 = 0x8040201008040201;
pub const H1_A8_DIAGONAL: u64 = 0x0102040810204080;
pub const LIGHT_SQUARES: u64 = 0x55aa55aa55aa55aa;
pub const DARK_SQUARES: u64 = 0xaa55aa55aa55aa55;
pub const ALL_SQUARES: u64 = LIGHT_SQUARES | DARK_SQUARES;
pub const NO_SQUARES: u64 = !ALL_SQUARES;

pub const NOT_A_FILE: u64 = !A_FILE;
pub const NOT_H_FILE: u64 = !H_FILE;
pub const NOT_AB_FILE: u64 = !AB_FILE;
pub const NOT_GH_FILE: u64 = !GH_FILE;
