use std::ops::{Index, IndexMut};

use num_traits::{FromPrimitive, ToPrimitive};
use strum::IntoEnumIterator;

use crate::bitboard::Bitboard;
use crate::direction::{BISHOP_DIRS, KnightDirection, ROOK_DIRS, RayDirection};
use crate::square::Square;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttackSet([Bitboard; 64]);

impl Default for AttackSet {
    fn default() -> Self {
        Self([Bitboard::default(); 64])
    }
}

impl Index<&Square> for AttackSet {
    type Output = Bitboard;

    fn index(&self, sq: &Square) -> &Self::Output {
        let idx = sq.to_usize().unwrap();
        &self.0[idx]
    }
}

impl IndexMut<&Square> for AttackSet {
    fn index_mut(&mut self, sq: &Square) -> &mut Self::Output {
        let idx = sq.to_usize().unwrap();
        &mut self.0[idx]
    }
}

impl Index<Square> for AttackSet {
    type Output = Bitboard;

    fn index(&self, sq: Square) -> &Self::Output {
        let idx = sq.to_usize().unwrap();
        &self.0[idx]
    }
}

impl IndexMut<Square> for AttackSet {
    fn index_mut(&mut self, sq: Square) -> &mut Self::Output {
        let idx = sq.to_usize().unwrap();
        &mut self.0[idx]
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Attacks {
    white_pawn: AttackSet,
    black_pawn: AttackSet,
    king: AttackSet,
    knight: AttackSet,
    rays: [AttackSet; 8],
}

impl Attacks {
    pub fn new() -> Self {
        let mut attacks = Attacks {
            white_pawn: AttackSet::default(),
            black_pawn: AttackSet::default(),
            king: AttackSet::default(),
            knight: AttackSet::default(),
            rays: [AttackSet::default(); 8],
        };

        for sq in Square::iter().take(63) {
            let bb = sq.bitboard();

            for (dir_idx, dir) in RayDirection::iter().enumerate() {
                attacks.rays[dir_idx][sq] = bb.fill_all(&dir) & !bb;
                attacks.king[sq] |= bb.fill_one(&dir) & !bb;
            }

            for kdir in KnightDirection::iter() {
                attacks.knight[sq] = bb.fill_all(&kdir) & !bb;
            }

            attacks.white_pawn[sq] |= bb.fill_one(&RayDirection::NE);
            attacks.white_pawn[sq] |= bb.fill_one(&RayDirection::NW);

            attacks.black_pawn[sq] |= bb.fill_one(&RayDirection::SE);
            attacks.black_pawn[sq] |= bb.fill_one(&RayDirection::SW);
        }

        attacks
    }

    pub fn king(&self, sq: Square, danger_squares: Bitboard) -> Bitboard {
        self.king[sq] & !danger_squares
    }

    pub fn knight(&self, sq: Square, blockers: Bitboard) -> Bitboard {
        self.knight[sq] & !blockers
    }

    pub fn bishop(&self, sq: Square, blockers: Bitboard) -> Bitboard {
        self.sliding_piece_attacks(sq, blockers, &BISHOP_DIRS)
    }

    pub fn rook(&self, sq: Square, blockers: Bitboard) -> Bitboard {
        self.sliding_piece_attacks(sq, blockers, &ROOK_DIRS)
    }

    fn sliding_piece_attacks(
        &self,
        sq: Square,
        blockers: Bitboard,
        dirs: &[RayDirection; 4],
    ) -> Bitboard {
        let mut attacks = Bitboard::default();
        for (dir_idx, dir) in dirs.iter().enumerate() {
            let ray = self.rays[dir_idx][sq];

            let pos: i32 = match dir {
                &RayDirection::E | &RayDirection::N | &RayDirection::NE | &RayDirection::NW => {
                    (ray & blockers).bitscan_reverse()
                }
                &RayDirection::W | &RayDirection::S | &RayDirection::SE | &RayDirection::SW => {
                    (ray & blockers).bitscan_forward()
                }
            };

            let block = Square::from_i32(pos).expect("Invalid square!");
            let unreachable = self.rays[dir_idx][block];

            attacks |= ray & !unreachable;
        }
        attacks
    }
}

// TODO: Add unit tests for attack generations.
#[cfg(test)]
mod tests {
    use super::Attacks;

    #[test]
    fn test_new() {
        let a1 = Attacks::new();
        let a2 = Attacks::new();
        assert_eq!(a1, a2);
    }

    #[test]
    fn test_king() {
        let at = Attacks::new();
    }

    #[test]
    fn test_knight() {}

    #[test]
    fn test_pawn() {}

    #[test]
    fn test_bishop() {}

    #[test]
    fn test_rook() {}

    #[test]
    fn test_queen() {}
}

