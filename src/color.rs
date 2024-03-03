use num_derive::FromPrimitive;
use num_derive::ToPrimitive;

use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, Default, PartialEq, FromPrimitive, ToPrimitive, EnumIter)]
pub enum Color {
    #[default]
    White,
    Black,
    None,
}

pub const NUM_COLORS: i8 = 2;