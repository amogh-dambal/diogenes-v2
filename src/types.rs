use num_derive::FromPrimitive;
use num_derive::ToPrimitive;

pub enum Direction {
    N = 8,
    NE = 9,
    E = 1,
    SE = -7,
    S = -8,
    SW = -9,
    W = -1,
    NW = 7
}

pub enum File {
    A, B, C, D, E, F, G, H
}

pub enum Rank {
    ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT
}

#[derive(Debug, PartialEq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    None,
}

const A_FILE: u64 =          0x0101010101010101;
const H_FILE: u64 =          0x8080808080808080;
const FIRST_RANK: u64 =      0x00000000000000ff;
const EIGHTH_RANK: u64 =     0xff00000000000000;
const A1_H8_DIAGONAL: u64 =  0x8040201008040201;
const H1_A8_DIAGONAL: u64 =  0x0102040810204080;
const LIGHT_SQUARES: u64 =   0x55aa55aa55aa55aa;
const DARK_SQUARES: u64 =    0xaa55aa55aa55aa55;
const ALL_SQUARES: u64 =     LIGHT_SQUARES | DARK_SQUARES;
const NO_SQUARES: u64 =      !ALL_SQUARES;

const NOT_A_FILE: u64 =      !A_FILE;
const NOT_H_FILE: u64 =      !H_FILE;


#[derive(Debug, Default, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Color {
    #[default]
    White,
    Black,
    None,
}

pub const NUM_COLORS: i8 = 2;