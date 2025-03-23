use crate::square::Square;


/// A bitfield encoding representing a single move
/// TODO: Use `zerocopy` or some bitfield equivalent
/// to encode this better.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Move {
    /// 00000000 0000 0000 0000 000000 000000
    /// EMPTYBTS CAPC FRPC SPFG TO--SQ FROMSQ
    data: u32,
}

const FROM_SQUARE: u32 = 0b111111;
const TO_SQUARE: u32 = FROM_SQUARE << 6;

// Bitmasks for special flags
const SPECIAL_FLAGS: u32 = 0b1111 << 12;
const DOUBLE_PAWN_PUSH: u32 = 0b0001;
const KING_CASTLE: u32 = 0b0010;
const QUEEN_CASTLE: u32 = 0b0011;
const CAPTURE: u32 = 0b0100;
const PROMO: u32 = 0b1000;
const EN_PASSANT: u32 = 0b0101;
const KNIGHT_PROMO: u32 = 0b1000;
const BISHOP_PROMO: u32 = 0b1001;
const ROOK_PROMO: u32 = 0b1010;
const QUEEN_PROMO: u32 = 0b1011;

impl Move {
    pub fn from(&self) -> Square {
        let sq: Option<Square> = num_traits::FromPrimitive::from_u32(self.data & FROM_SQUARE);
        sq.expect("Invalid square!")
    }

    pub fn to(&self) -> Square {
        let sq: Option<Square> = num_traits::FromPrimitive::from_u32(self.data >> 6 & TO_SQUARE);
        sq.expect("Invalid square!")
    }

    pub fn is_capture(&self) -> bool {
        let flags: u32 = self.read_sp_flags();
        (flags & CAPTURE) != 0
    }

    pub fn is_promo(&self) -> bool {
        let flags: u32 = self.read_sp_flags();
        (flags & PROMO) != 0
    }

    fn read_sp_flags(&self) -> u32 {
        (self.data & SPECIAL_FLAGS) >> 12
    }

}