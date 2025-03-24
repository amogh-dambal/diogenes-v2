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
    /// Create a new eagerly-initialized [`Attacks`] which contains all attack sets
    /// for each piece on each square.
    ///
    /// This precomputation enables O(1) attack set lookup during move generation.
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

    /// TODO: Implement pawn attacks (have to take en-passant into play).
    pub fn pawns(&self, _sq: Square, _blockers: Bitboard) -> Bitboard {
        todo!("write logic to generate pawn attacks")
    }

    /// Returns a [`Bitboard`] representing all squares attacked by a king located
    /// on `sq` given a set of "danger" squares (i.e squares to which a king could
    /// not move without moving into check, which is illegal).
    pub fn king(&self, sq: Square, danger_squares: Bitboard) -> Bitboard {
        self.king[sq] & !danger_squares
    }

    /// Returns a [`Bitboard`] of all squares attacked by a knight on
    /// a square given "blockers" (i.e. other pieces which may already
    /// exist on the board).
    pub fn knight(&self, sq: Square, blockers: Bitboard) -> Bitboard {
        self.knight[sq] & !blockers
    }

    /// Returns a [`Bitboard`] of all squares attacked by a bishop on
    /// a square given "blockers" (i.e. other pieces which may already
    /// exist on the board).
    pub fn bishop(&self, sq: Square, blockers: Bitboard) -> Bitboard {
        self.sliding_piece_attacks(sq, blockers, &BISHOP_DIRS)
    }

    /// Returns a [`Bitboard`] of all squares attacked by a rook on
    /// a square given "blockers" (i.e. other pieces which may already
    /// exist on the board).
    pub fn rook(&self, sq: Square, blockers: Bitboard) -> Bitboard {
        self.sliding_piece_attacks(sq, blockers, &ROOK_DIRS)
    }

    /// Returns a [`Bitboard`] of all squares attacked by a queen on
    /// a square given "blockers" (i.e. other pieces which may already
    /// exist on the board).
    pub fn queen(&self, sq: Square, blockers: Bitboard) -> Bitboard {
        self.rook(sq, blockers) | self.bishop(sq, blockers)
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
    use rstest::rstest;

    use crate::{bitboard::Bitboard, square::Square};

    use super::Attacks;

    #[test]
    fn test_new() {
        let a1 = Attacks::new();
        let a2 = Attacks::new();
        assert_eq!(a1, a2);
    }

    #[rstest]
    #[case::king_no_danger(Square::E1, Bitboard::default(), Bitboard::new(0b0011100000101000))]
    #[case::king_all_danger(Square::E8, Bitboard::default(), Bitboard::new(0b0011100000101000).flip_vertical())]
    #[case::king_mixed(
        Square::G1,
        Bitboard::new(0b1110000000000000),
        Bitboard::new(0b10100000)
    )]
    fn test_king(#[case] square: Square, #[case] danger: Bitboard, #[case] expected: Bitboard) {
        let at = Attacks::new();
        let actual = at.king(square, danger);
        assert_eq!(
            expected, actual,
            "expected\n{expected:?} but got\n{actual:?}"
        );
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
