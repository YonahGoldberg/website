use super::Square;
use num::FromPrimitive;

pub const QUIET: u16 = 0;
pub const PAWN_DPUSH: u16 = 1;
pub const KING_CASTLE: u16 = 2;
pub const QUEEN_CASTLE: u16 = 3;
pub const CAPTURE: u16 = 4;
pub const EP_CAPTURE: u16 = 5;
pub const KNIGHT_PROMO: u16 = 8;
pub const BISHOP_PROMO: u16 = 9;
pub const ROOK_PROMO: u16 = 10;
pub const QUEEN_PROMO: u16 = 11;
pub const KNIGHT_PROMO_CAPTURE: u16 = 12;
pub const BISHOP_PROMO_CAPTURE: u16 = 13;
pub const ROOK_PROMO_CAPTURE: u16 = 14;
pub const QUEEN_PROMO_CAPTURE: u16 = 15;
pub struct Cmove(u16);

impl Cmove {
    pub fn new(from: Square, to: Square, flags: u16) -> Cmove {
        Cmove(((flags & 0xf) << 12) | ((from as u16 & 0x3f) << 6) | (to as u16 & 0x3f))
    }

    pub fn get_from(&self) -> Square {
        // Index can't be more than 63
        // Square::from_u16((self.0 >> 6) & 0x3f).unwrap()
        FromPrimitive::from_u16((self.0 >> 6) & 0x3f).unwrap()
    }

    pub fn get_to(&self) -> Square {
        // Index can't be more than 63
        FromPrimitive::from_u16(self.0 & 0x3f).unwrap()
    }
}
