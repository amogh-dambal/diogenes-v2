use num_derive::{FromPrimitive, ToPrimitive};

use crate::board::{ALL_SQUARES, NOT_A_FILE, NOT_AB_FILE, NOT_GH_FILE, NOT_H_FILE};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Ray(RayDirection),
    Knight(KnightDirection),
}

impl Direction {
    pub const BISHOP_DIRS: [RayDirection; 4] = [
        RayDirection::NE,
        RayDirection::NW,
        RayDirection::SE,
        RayDirection::SW,
    ];
    pub const ROOK_DIRS: [RayDirection; 4] = [
        RayDirection::N,
        RayDirection::S,
        RayDirection::E,
        RayDirection::W,
    ];

    pub(crate) fn shift(&self) -> isize {
        match self {
            Direction::Ray(ray_dir) => match ray_dir {
                RayDirection::N => 8,
                RayDirection::S => -8,
                RayDirection::NE => 9,
                RayDirection::SW => -9,
                RayDirection::E => 1,
                RayDirection::W => -1,
                RayDirection::NW => 7,
                RayDirection::SE => -7,
            },
            Direction::Knight(knight_dir) => match knight_dir {
                KnightDirection::NNE => 17,
                KnightDirection::SSW => -17,
                KnightDirection::NEE => 10,
                KnightDirection::SWW => -10,
                KnightDirection::SEE => -6,
                KnightDirection::NWW => 6,
                KnightDirection::SSE => -15,
                KnightDirection::NNW => 15,
            },
        }
    }

    pub(crate) fn mask(&self) -> u64 {
        match self {
            Direction::Ray(ray_dir) => match ray_dir {
                RayDirection::E | RayDirection::NE | RayDirection::SE => NOT_A_FILE,
                RayDirection::W | RayDirection::NW | RayDirection::SW => NOT_H_FILE,
                _ => ALL_SQUARES,
            },
            Direction::Knight(knight_dir) => match knight_dir {
                KnightDirection::NNE | KnightDirection::SSE => NOT_A_FILE,
                KnightDirection::NNW | KnightDirection::SSW => NOT_H_FILE,
                KnightDirection::NEE | KnightDirection::SEE => NOT_AB_FILE,
                KnightDirection::NWW | KnightDirection::SWW => NOT_GH_FILE,
            },
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    strum::EnumIter,
    FromPrimitive,
    ToPrimitive,
)]
pub enum RayDirection {
    N = 8,
    NE = 9,
    E = 1,
    SE = -7,
    S = -8,
    SW = -9,
    W = -1,
    NW = 7,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    strum::EnumIter,
    FromPrimitive,
    ToPrimitive,
)]
pub enum KnightDirection {
    NNE = 17,
    NEE = 10,
    SEE = -6,
    SSE = -15,
    SSW = -17,
    SWW = -10,
    NWW = 6,
    NNW = 15,
}
