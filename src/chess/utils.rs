use std::ops::Not;
use num::FromPrimitive;
use super::bitboard::Bitboard;
use Dir::*;

#[derive(Clone, Copy, FromPrimitive, Debug)]
pub enum Color {
    White,
    Black,
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}

/// All chess piece types
#[derive(Clone, Copy, FromPrimitive, Debug)]
pub enum Piece {
    Pawn, Knight, Bishop, Rook, Queen, King,
}
pub struct CPiece(pub Piece, pub Color);

/// All eight cardinal directions
#[derive(Clone, Copy, FromPrimitive, Debug)]
pub enum Dir {
    Nort, Noea, East, Soea, Sout, Sowe, West, Nowe,
}

impl Dir {
    pub fn neg(&self) -> bool {
        match *self {
            West | Sout | Sowe | Soea => true,
            _ => false,
        }
    }

    pub fn pos(&self) -> bool {
        match *self {
            West | Sout | Sowe | Soea => false,
            _ => true,
        }
    }
}

/// All squares on a chess board
#[derive(Clone, Copy, FromPrimitive, Debug)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    /// Returns a bitboard with a one set on this square and
    /// zeroes everywhere else
    pub fn as_bitboard(&self) -> Bitboard {
       Bitboard(1) << *self as i32
    }

    /// Returns `Some(s)` if there exists a square `s` `steps` steps
    /// away from this square in direction `dir`, otherwise `None`.
    /// Moving east nine is equivalent to moving north one and moving
    /// west 9 is equivalent to moving south one, and so on so that
    /// east and west rap around until they can't anymore.
    pub fn translate(&self, dir: Dir, steps: i32) -> Option<Square> {
        let amount = match dir {
            Nort => 8, Noea => 9, East => 1, Soea => -7,
            Sout => -8, Sowe => -9, West => -1, Nowe => 7,
        };
        FromPrimitive::from_i32(*self as i32 + amount * steps)
    }
}