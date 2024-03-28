use super::utils::{
    Piece::{self, *},
    Square,
};
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
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CMove(u16);

impl CMove {
    pub fn new(from: Square, to: Square, flags: u16) -> CMove {
        CMove(((flags & 0xf) << 12) | ((from as u16) << 6) | (to as u16))
    }

    pub fn get_from(&self) -> Square {
        // Index can't be more than 63
        FromPrimitive::from_u16((self.0 >> 6) & 0x3f).unwrap()
    }

    pub fn get_to(&self) -> Square {
        // Index can't be more than 63
        FromPrimitive::from_u16(self.0 & 0x3f).unwrap()
    }

    pub fn get_flags(&self) -> u16 {
        self.0 >> 12
    }

    pub fn set_from(&mut self, from: Square) {
        self.0 = (self.0 & 0xff3f) | ((from as u16) << 6);
    }

    pub fn set_to(&mut self, to: Square) {
        self.0 = (self.0 & 0xffc0) | (to as u16);
    }

    pub fn is_capture(&self) -> bool {
        self.0 & CAPTURE != 0
    }

    pub fn is_ep_capture(&self) -> bool {
        self.0 == EP_CAPTURE
    }

    pub fn is_pawn_dpush(&self) -> bool {
        self.0 == PAWN_DPUSH
    }

    pub fn is_promo(&self) -> Option<Piece> {
        if self.0 & 8 > 0 {
            // Lowest 2 bits
            Some(match self.0 & 3 {
                0 => Knight,
                1 => Bishop,
                2 => Rook,
                3 => Queen,
                _ => panic!(),
            })
        } else {
            None
        }
    }

    pub fn is_king_castle(&self) -> bool {
        self.0 == KING_CASTLE
    }

    pub fn is_queen_castle(&self) -> bool {
        self.0 == QUEEN_CASTLE
    }
}
