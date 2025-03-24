use std::collections::HashSet;
use std::fmt::{Binary, Debug, Display, Write};
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr, ShrAssign
};

use num_derive::{FromPrimitive, ToPrimitive};
use strum::IntoEnumIterator;

use crate::board::{self, File, Rank};
use crate::direction::Direction;

const DEBRUIJN_LOOKUP: [i32; 64] = [
    0, 47, 1, 56, 48, 27, 2, 60, 57, 49, 41, 37, 28, 16, 3, 61, 54, 58, 35, 52, 50, 42, 21, 44, 38,
    32, 29, 23, 17, 11, 4, 62, 46, 55, 26, 59, 40, 36, 15, 53, 34, 51, 20, 43, 31, 22, 10, 45, 25,
    39, 14, 33, 19, 30, 9, 24, 13, 18, 8, 12, 7, 6, 5, 63,
];
const DEBRUIJN_MAGIC_VAL: u64 = 0x03f79d71b4cb0a89;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
pub struct Bitboard(u64);

impl Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let set_bits: HashSet<usize> = HashSet::from_iter(self.serialize());
        for rank in (0..=7).rev() {
            for file in 0..= 7 {
                let i = board::index(file as usize, rank as usize);
                if set_bits.contains(&i) {
                    write!(f, "X ")?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        
        }
        writeln!(f)
    }
}

/// Binary operation trait implementations
/// for our Bitboard to allow us to overload
/// shifts, AND/OR, etc
impl Binary for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAnd<u64> for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: u64) -> Self::Output {
        Self(self.0 & rhs)
    }
}

impl BitAndAssign<usize> for Bitboard {
    fn bitand_assign(&mut self, rhs: usize) {
        self.0 &= rhs as u64;
    }
}

impl BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitOrAssign<usize> for Bitboard {
    fn bitor_assign(&mut self, rhs: usize) {
        self.0 |= rhs as u64;
    }
}

impl BitOrAssign<u64> for Bitboard {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs;
    }
}

impl BitXor for Bitboard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign<usize> for Bitboard {
    fn bitxor_assign(&mut self, rhs: usize) {
        self.0 ^= rhs as u64;
    }
}

impl Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl Shl<isize> for Bitboard {
    type Output = Self;
    fn shl(self, rhs: isize) -> Self::Output {
        if rhs > 0 {
            Self(self.0 << rhs)
        } else {
            Self(self.0 >> -rhs)
        }
    }
}

impl ShlAssign<isize> for Bitboard {
    fn shl_assign(&mut self, rhs: isize) {
        if rhs > 0 {
            self.0 <<= rhs;
        } else {
            self.0 >>= -rhs;
        }
    }
}

impl Shr<isize> for Bitboard {
    type Output = Self;
    fn shr(self, rhs: isize) -> Self::Output {
        if rhs > 0 {
            Self(self.0 >> rhs)
        } else {
            Self(self.0 << -rhs)
        }
    }
}

impl ShrAssign<isize> for Bitboard {
    fn shr_assign(&mut self, rhs: isize) {
        if rhs > 0 {
            self.0 >>= rhs;
        } else {
            self.0 <<= -rhs;
        }
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for rank in Rank::iter().rev() {
            for file in File::iter() {
                let idx = board::try_index(file, rank).expect("invalid file + rank for index! todo");
                let k = 1 << idx;

                if self.0 & k != 0 {
                    write!(s, "{}", 1)?;
                } else {
                    write!(s, ".")?;
                }
            }
            writeln!(s)?;
        }
        write!(f, "{}", s)
    }
}

impl Bitboard {
    /// Creates a new bitboard from the provided [`u64`].
    pub fn new(data: u64) -> Bitboard {
        Bitboard(data)
    }

    /// Returns the boolean representation of this bitboard.
    pub fn bool(&self) -> bool {
        self.0 != 0
    }

    /// Generic fill algorithms that use Dumb7 fill
    /// to fill a bitboard in a specific direction
    /// with a specific propagator. These are used to compute various attack
    /// sets.
    /// TODO: If we want to optimize performance, we can
    /// unroll this loop.
    pub fn fill_all(&self, dir: &impl Direction) -> Bitboard {
        let mut flood = Bitboard(self.0);
        for _ in 0..=7 {
            flood |= flood.fill_one(dir);
        }

        flood
    }

    /// Generic fill algorithm which fills a bitboard in a given direction.
    pub fn fill_one(&self, dir: &impl Direction) -> Bitboard {
        let mut result = Bitboard(self.0);
        let shift = dir.get_shift();
        let mask = dir.get_wraparound_mask();
        result |= (result << shift) & mask;
        
        result
    }

    // Utility functions to compute information about bitboards

    /// Flip a bitboard vertically about the center ranks.
    /// Rank 1 is mapped to rank 8 and vice versa.
    pub fn flip_vertical(&mut self) -> Bitboard {
        let k1: u64 = 0x00FF00FF00FF00FF;
        let k2: u64 = 0x0000FFFF0000FFFF;
        let mut x: u64 = self.0;
        x = ((x >> 8) & k1) | ((x & k1) << 8);
        x = ((x >> 16) & k2) | ((x & k2) << 16);
        x = x.rotate_left(32);

        Bitboard(x)
    }

    /// Compute the number of set bits in the bitboard.
    ///
    /// Uses Brian Kernighan's loop-based algorithm, which
    /// is more performant for sparsely populated bitboards.
    pub fn popcount(&self) -> i32 {
        let mut pc: i32 = 0;
        let mut val = self.0;
        while val != 0 {
            pc += 1;
            val &= val - 1;
        }
        pc
    }

    /// Returns the index of the LS1B (least significant 1 bit)
    ///
    /// Uses a DeBruijn lookup table to compute the index of
    /// the least significant 1 bit set in the given bitboard.
    /// Assume that indices increase going from right to left.
    pub fn bitscan_forward(&self) -> i32 {
        let val: u64 = self.0;
        if val == 0 {
            return -1;
        }

        // Need the wrapped version of the function because the algorithm
        // relies on overflow behavior which is disallowed by default.
        let key: u64 = u64::wrapping_mul(val ^ (val - 1), DEBRUIJN_MAGIC_VAL) >> 58;
        DEBRUIJN_LOOKUP[key as usize]
    }

    /// Returns the index of the MS1B (most significant 1 bit)
    ///
    /// Uses a DeBruijn lookup table to compute the index of
    /// the most significant 1 bit set in the given bitboard.
    /// Assume that indices increase going from right to left.
    pub fn bitscan_reverse(&self) -> i32 {
        let mut val: u64 = self.0;
        if val == 0 {
            return -1;
        }

        val |= val >> 1;
        val |= val >> 2;
        val |= val >> 4;
        val |= val >> 8;
        val |= val >> 16;
        val |= val >> 32;
        let key: u64 = u64::wrapping_mul(val, DEBRUIJN_MAGIC_VAL) >> 58;
        DEBRUIJN_LOOKUP[key as usize]
    }

    /// Convert the set-centric bitboard representation to
    /// a list of indices where bits are set.
    pub fn serialize(&self) -> Vec<usize> {
        let mut indices: Vec<usize> = Vec::new();
        let mut bb: Bitboard = Bitboard(self.0);
        while bb.0 != 0 {
            indices.push(bb.bitscan_forward() as usize);
            bb.0 &= bb.0 - 1;
        }

        indices
    }
}

#[cfg(test)]
mod tests {
    use crate::{board, direction::RayDirection};

    use super::Bitboard;
    use rand::Rng;

    #[test]
    fn test_bool() {
        let set_bb = Bitboard(0b10001);
        assert!(set_bb.bool());

        let unset_bb = Bitboard(0);
        assert!(!unset_bb.bool());

        let default_bb = Bitboard::default();
        assert!(!default_bb.bool());
    }

    #[test]
    fn test_fill() {
        let x = Bitboard(0xFF00);
        let x_south = Bitboard(0xFFFF);
        assert_eq!(x_south, x.fill_all(&RayDirection::S));

        let x_north = Bitboard(0xFFFFFFFFFFFFFF00);
        assert_eq!(x_north, x.fill_all(&RayDirection::N));

        let y = Bitboard(0b1).fill_all(&RayDirection::N);
        let y_east = Bitboard(board::ALL_SQUARES);
        assert_eq!(y_east, y.fill_all(&RayDirection::E));
        assert_eq!(y, y.fill_all(&RayDirection::W));
    }

    #[test]
    fn test_bitboard_ops() {
        let x: u64 = 0b1010001;
        let y: u64 = 0b0010101;

        // Standard operations
        assert_eq!(x & y, (Bitboard(x) & Bitboard(y)).0);
        assert_eq!(x | y, (Bitboard(x) | Bitboard(y)).0);
        assert_eq!(x ^ y, (Bitboard(x) ^ Bitboard(y)).0);
        assert_eq!(!x, (!Bitboard(x)).0);

        // OpAssign operations
        let mut p: u64 = 0b101101100;
        let q: u64 = 0b011001010;
        let mut bp = Bitboard(p);
        let bq = Bitboard(q);

        p |= q;
        bp |= bq;
        assert_eq!(p, bp.0);

        let shift = rand::thread_rng().gen_range(1..32);
        assert_eq!(x << shift, (Bitboard(x) << shift).0);
        assert_eq!(x >> shift, (Bitboard(x) >> shift).0);
    }

    #[test]
    fn test_popcount() {
        let mut bb: Bitboard = Bitboard(0);
        for i in 0..=63 {
            bb.0 |= 1 << i;
            assert_eq!(i + 1, bb.popcount());
        }
    }

    #[test]
    fn test_bitscans() {
        let mut bb: Bitboard = Bitboard(0);
        let first = rand::thread_rng().gen_range(0..=63);
        let second = rand::thread_rng().gen_range(0..=63);
        bb.0 |= 1 << first;
        bb.0 |= 1 << second;

        assert_eq!(bb.bitscan_forward(), first.min(second));
        assert_eq!(bb.bitscan_reverse(), first.max(second));
    }

    #[test]
    fn test_serialize() {
        let mut sample_indices: Vec<usize> = rand::seq::index::sample(
            &mut rand::thread_rng(),
            63,
            rand::thread_rng().gen_range(0..=63),
        )
        .into_vec();

        let mut bb: Bitboard = Bitboard(0);
        for idx in &sample_indices {
            let i: u64 = *idx as u64;
            bb.0 |= 1 << i;
        }
        let mut indices: Vec<usize> = bb.serialize();

        // Allows us to more easily compare two vectors
        indices.sort();
        sample_indices.sort();

        let matching = sample_indices
            .iter()
            .zip(indices)
            .filter(|&(a, b)| *a == b)
            .count();

        assert_eq!(matching, sample_indices.len());
    }
}

