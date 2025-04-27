use arbitrary_int::{Number, u2, u4, u6};
use bitbybit::bitfield;
use num_traits::FromPrimitive;

use crate::{piece::Piece, square::Square};

/// A single move encoded into a [`u32`] using the following standard encodings scheme:
/// ```
/// // 00000000 0000 0000 0000 000000 000000
/// // EMPTYBTS CAPC FRPC SPFG TO--SQ FROMSQ
/// ```
///
/// where the bits are read from MSB->LSB.
///
/// The first and second six bits encode the index of the source
/// and destination square of the move, respectively. Since there
/// are 64 possible indices, 6 bits are sufficient.
///
/// The next four bits are a set of bitflags which encode various "special" moves.
/// The first two bits each encode one of four unique moves:
/// 00: Quiet move
/// 01: Double pawn push
/// 10: Kingside castle
/// 11: Queenside castle
///
/// If the third bit is set, the move is a capture. The only sub-case for captures is _en passant_, which
/// is represented using:
/// 0101: En passant
///
/// If the fourth bit is set, the move is a promotion. If the promotion bit is set,
/// then the lower two bits encode the type of the promoted piece. A capture bit can _also_ be set
/// to encode a capture promotion (i.e. a pawn captures a piece on the eighth rank to promote).
/// 00: Knight promotion
/// 01: Bishop promotion
/// 10: Rook promotion
/// 11: Queen promotion
///
/// This gives a full special move table of:
/// 0000: Quiet move
/// 0001: Double pawn push
/// 0010: Kingside castle
/// 0011: Queenside castle
/// 0100: Standard capture
/// 0101: En passant
/// 0110: INVALID
/// 0111: INVALID
/// 1000: Knight promotion
/// 1001: Bishop promotion
/// 1010: Rook promotion
/// 1011: Queen promotion
/// 1100: Knight promotion with capture
/// 1101: Bishop promotion with capture
/// 1110: Rook promotion with capture
/// 1111: Queen promotion with capture
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Move {
    #[bits(24..=31)]
    _unused: u8,
    #[bits(20..=23, rw)]
    captured_piece: u4,
    #[bits(16..=19, rw)]
    moved_piece: u4,
    #[bit(15, rw)]
    promotion: bool,
    #[bit(14, rw)]
    capture: bool,
    #[bits(12..=13, rw)]
    special: u2,
    #[bits(6..=11, rw)]
    to_sq: u6,
    #[bits(0..=5, rw)]
    from_sq: u6,
}

impl Move {
    pub fn new(val: u32) -> Self {
        Self::new_with_raw_value(val)
    }

    pub fn from(&self) -> Option<Square> {
        Square::from_u8(self.from_sq().as_u8())
    }

    pub fn to(&self) -> Option<Square> {
        Square::from_u8(self.to_sq().as_u8())
    }

    pub fn moved(&self) -> Option<Piece> {
        Piece::from_u8(self.moved_piece().as_u8())
    }

    pub fn captured(&self) -> Option<Piece> {
        Piece::from_u8(self.captured_piece().as_u8())
    }

    pub fn is_quiet(&self) -> bool {
        self.special().as_u8() == 0u8
    }
}
