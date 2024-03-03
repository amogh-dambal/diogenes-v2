use std::ops::AddAssign;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter, FromPrimitive, ToPrimitive)]
pub enum File {
    A, B, C, D, E, F, G, H
}

impl AddAssign<u32> for File {
    fn add_assign(&mut self, rhs: u32) {
        let val = self.to_u32().expect("Invalid add!") + rhs;
        let file = File::from_u32(val).expect("Invalid file!");
        
        *self = file;
    }
}

#[derive(Clone, Copy, Debug, EnumIter, FromPrimitive, ToPrimitive)]
pub enum Rank {
    ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT
}

pub const A_FILE: u64 =          0x0101010101010101;
pub const H_FILE: u64 =          0x8080808080808080;
pub const FIRST_RANK: u64 =      0x00000000000000ff;
pub const EIGHTH_RANK: u64 =     0xff00000000000000;
pub const A1_H8_DIAGONAL: u64 =  0x8040201008040201;
pub const H1_A8_DIAGONAL: u64 =  0x0102040810204080;
pub const LIGHT_SQUARES: u64 =   0x55aa55aa55aa55aa;
pub const DARK_SQUARES: u64 =    0xaa55aa55aa55aa55;
pub const ALL_SQUARES: u64 =     LIGHT_SQUARES | DARK_SQUARES;
pub const NO_SQUARES: u64 =      !ALL_SQUARES;

pub const NOT_A_FILE: u64 =      !A_FILE;
pub const NOT_H_FILE: u64 =      !H_FILE;