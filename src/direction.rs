#![allow(clippy::upper_case_acronyms)]

use num_derive::{FromPrimitive, ToPrimitive};

use crate::board::{ALL_SQUARES, NOT_AB_FILE, NOT_A_FILE, NOT_GH_FILE, NOT_H_FILE};

pub trait Direction {
    fn get_shift(&self) -> isize;
    fn get_wraparound_mask(&self) -> u64;
}


#[derive(strum::EnumIter, FromPrimitive, ToPrimitive, PartialEq, Eq)]
pub enum RayDirection {
    N,  // +8
    NE, // +9
    E,  // +1
    SE, // -7
    S,  // -8
    SW, // -9
    W,  // -1
    NW, // +7
}

impl Direction for RayDirection {
    fn get_shift(&self) -> isize {
        match self {
            RayDirection::N => 8,
            RayDirection::S => -8,
            RayDirection::NE => 9,
            RayDirection::SW => -9,
            RayDirection::E => 1,
            RayDirection::W => -1,
            RayDirection::NW => 7,
            RayDirection::SE => -7,
        }
    }

    fn get_wraparound_mask(&self) -> u64 {
        match self {
            RayDirection::E | RayDirection::NE | RayDirection::SE => {
                NOT_A_FILE
            }
            RayDirection::W | RayDirection::NW | RayDirection::SW => {
                NOT_H_FILE
            }
            _ => {
                // Basically a no-op
                ALL_SQUARES
            }
        }
    }
}

#[derive(strum::EnumIter, FromPrimitive, ToPrimitive, PartialEq, Eq)]
pub enum KnightDirection {
    NNE, // +17
    NEE, // +10
    SEE, // -6
    SSE, // -15
    SSW, // -17
    SWW, // -10
    NWW, // +6
    NNW, // +15
}

impl Direction for KnightDirection {
    fn get_shift(&self) -> isize {
        match self {
            KnightDirection::NNE => 17,
            KnightDirection::SSW => -17,
            KnightDirection::NEE => 10,
            KnightDirection::SWW => -10,
            KnightDirection::SEE => 6,
            KnightDirection::NWW => -6,
            KnightDirection::SSE => 15,
            KnightDirection::NNW => -15,
        }
    }

    fn get_wraparound_mask(&self) -> u64 {
        match self {
            KnightDirection::NNE | KnightDirection::SSE => NOT_A_FILE,
            KnightDirection::NNW | KnightDirection::SSW => NOT_H_FILE,
            KnightDirection::NEE | KnightDirection::SEE => NOT_AB_FILE,
            KnightDirection::NWW | KnightDirection::SWW => NOT_GH_FILE,
        }
    }
}

pub const BISHOP_DIRS: [RayDirection; 4] = [RayDirection::NE, RayDirection::NW, RayDirection::SE, RayDirection::SW];
pub const ROOK_DIRS: [RayDirection; 4] = [RayDirection::N, RayDirection::S, RayDirection::E, RayDirection::W];