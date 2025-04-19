use bitfield_struct::bitfield;

use crate::{piece::Piece, square::Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MoveType {
    Quiet,
    DoublePawnPush,
    KingsideCastle,
    QueensideCastle,
    Capture,
    EnPassant,
    KnightPromotion,
    BishopPromotion,
    RookPromotion,
    QueenPromotion,
    KnightPromotionCapture,
    BishopPromotionCapture,
    RookPromotionCapture,
    QueenPromotionCapture,
}

impl MoveType {
    const fn from_bits(val: usize) -> Self {
        match val {
            0 => Self::Quiet,
            1 => Self::DoublePawnPush,
            2 => Self::KingsideCastle,
            3 => Self::QueensideCastle,
            4 => Self::Capture,
            5 => Self::EnPassant,
            8 => Self::KnightPromotion,
            9 => Self::BishopPromotion,
            10 => Self::RookPromotion,
            11 => Self::QueenPromotion,
            12 => Self::KnightPromotionCapture,
            13 => Self::BishopPromotionCapture,
            14 => Self::RookPromotionCapture,
            15 => Self::QueenPromotionCapture,
        }
    }
}

/// A bitfield encoding representing a single move
/// TODO: Use `zerocopy` or some bitfield equivalent
/// to encode this better.
#[bitfield(u32)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Move {
    #[bits(6)]
    from_square: Square,

    #[bits(6)]
    to_square: Square,

    #[bits(2, default = MoveType::Quiet)]
    move_type: MoveType,

    #[bits(4)]
    from_piece: Piece,

    #[bits(4)]
    captured_piece: Option<Piece>,

    #[bits(8)]
    _unused: u8,
}

impl Move {
    /// Returns [`true`] if the move is a quiet move.
    ///
    /// See <TODO: LINK>
    pub fn is_quiet(&self) -> bool {
        matches!(self.move_type(), MoveType::Quiet)
    }

    /// Returns [`true`] if the move is a capture move.
    pub fn is_capture(&self) -> bool {
        matches!(self.move_type(), MoveType::Capture)
    }

    /// Returns [`true`] if the move is a promotion.
    pub fn is_promo(&self) -> bool {
        matches!(
            self.move_type(),
            MoveType::BishopPromotion
                | MoveType::KnightPromotion
                | MoveType::QueenPromotion
                | MoveType::RookPromotion
        )
    }
}
