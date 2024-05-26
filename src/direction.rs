use num_derive::{FromPrimitive, ToPrimitive};
use strum_macros::EnumIter;

use crate::board::{ALL_SQUARES, NOT_AB_FILE, NOT_A_FILE, NOT_GH_FILE, NOT_H_FILE};

pub trait Direction {
    fn get_shift(&self) -> isize;
    fn get_wraparound_mask(&self) -> u64;
}


#[derive(EnumIter, FromPrimitive, ToPrimitive, PartialEq, Eq)]
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
            RayDirection::N => return 8,
            RayDirection::S => return -8,
            RayDirection::NE => return 9,
            RayDirection::SW => return -9,
            RayDirection::E => return 1,
            RayDirection::W => return -1,
            RayDirection::NW => return 7,
            RayDirection::SE => return -7,
        }
    }

    fn get_wraparound_mask(&self) -> u64 {
        match self {
            RayDirection::E | RayDirection::NE | RayDirection::SE => {
                return NOT_A_FILE;
            }
            RayDirection::W | RayDirection::NW | RayDirection::SW => {
                return NOT_H_FILE;
            }
            _ => {
                // Basically a no-op
                return ALL_SQUARES;
            }
        }
    }
}

#[derive(EnumIter, FromPrimitive, ToPrimitive, PartialEq, Eq)]
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