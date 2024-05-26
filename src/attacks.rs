use num_traits::{FromPrimitive, ToPrimitive};
use strum::IntoEnumIterator;

use crate::bitboard::Bitboard;
use crate::direction::{KnightDirection, RayDirection, BISHOP_DIRS, ROOK_DIRS};
use crate::square::Square;

#[derive(Debug, PartialEq, Eq)]
pub struct Attacks {
    white_pawn: [Bitboard; 64],
    black_pawn: [Bitboard; 64],
    king: [Bitboard; 64],
    knight: [Bitboard; 64],
    rays: [[Bitboard; 64]; 8],
}

impl Attacks {
    pub fn new() -> Attacks {
        let mut a = Attacks {
            white_pawn: [Bitboard::default(); 64],
            black_pawn: [Bitboard::default(); 64],
            king: [Bitboard::default(); 64],
            knight: [Bitboard::default(); 64],
            rays: [[Bitboard::default(); 64]; 8],
        };
        
        for (sq, _) in Square::iter().take(63).enumerate() {
            let bb = Bitboard::new(1 << sq);
            
            for (dir_idx, dir) in RayDirection::iter().enumerate() {
                a.rays[dir_idx][sq] = bb.fill_all(&dir) & !bb;
                a.king[sq] |= bb.fill_one(&dir) & !bb;
            }
            
            for kdir in KnightDirection::iter() {
                a.knight[sq] = bb.fill_all(&kdir) & !bb;
            }

            a.white_pawn[sq] |= bb.fill_one(&RayDirection::NE);
            a.white_pawn[sq] |= bb.fill_one(&RayDirection::NW);

            a.black_pawn[sq] |= bb.fill_one(&RayDirection::SE);
            a.black_pawn[sq] |= bb.fill_one(&RayDirection::SW);
        }
        
        return a;
    }

    pub fn king(&self, sq: &Square, danger_squares: Bitboard) -> Bitboard {
        let sq_idx = sq.to_usize().expect("Invalid square");
        return self.king[sq_idx] & !danger_squares;
    }

    pub fn knight(&self, sq: Square, blockers: Bitboard) -> Bitboard {
        let sq_idx = sq.to_usize().expect("Invalid bitboard");
        return self.knight[sq_idx] & !blockers;
    }

    pub fn bishop(&self, sq: Square, blockers: Bitboard) -> Bitboard {
        return self.sliding_piece_attacks(sq, blockers, &BISHOP_DIRS);
    }

    pub fn rook(&self, sq: Square, blockers: Bitboard) -> Bitboard {
        return self.sliding_piece_attacks(sq, blockers, &ROOK_DIRS);
    }

    fn sliding_piece_attacks(&self, sq: Square, blockers: Bitboard, dirs: &[RayDirection; 4]) -> Bitboard {
        let mut attacks = Bitboard::default();
        for (dir_idx, dir) in dirs.iter().enumerate() {
            let ray = self.rays[dir_idx][sq.to_usize().expect("Invalid square!")];

            let pos: i32;
            match dir {
                &RayDirection::E | &RayDirection::N | &RayDirection::NE | &RayDirection::NW => {
                    pos = (ray & blockers).bitscan_reverse();    
                },
                &RayDirection::W | &RayDirection::S | &RayDirection::SE | &RayDirection::SW => {
                    pos = (ray & blockers).bitscan_forward(); 
                }
            }
            
            let block = Square::from_i32(pos).expect("Invalid square!");
            let unreachable = self.rays[dir_idx][block.to_usize().expect("Invalid square!")];

            attacks |= ray & !unreachable;
        }
        return attacks;
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
    fn test_knight() {

    }

    #[test]
    fn test_pawn() {

    }

    #[test]
    fn test_bishop() {

    }

    #[test]
    fn test_rook() {

    }

    #[test]
    fn test_queen() {
        
    }
}