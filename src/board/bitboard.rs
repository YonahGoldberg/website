/// A bitboard is represented as a 64 bit unsigned integer, 1 bit
/// per square for the standard 8x8 chess board. A 1 bit set indicates
/// the presence (or lack thereof) of some property for that square.
/// Properties include a particular piece being present, that square being
/// a potential target, etc... These bitboard are implmented in 
/// Little Endian Rank-File (LERF) order, meaning towards higher
/// valued bits we traverse first across a rank (the numbers), then up files (the letters).
/// 
/// To read more on bitboard representations, you can visit: 
/// <https://www.chessprogramming.org/Square_Mapping_Considerations>

pub mod tables;
pub use tables::*;
use super::{Square, Piece};
use num::FromPrimitive;
use std::ops::{
    BitAnd, BitAndAssign, BitOr, 
    BitOrAssign, BitXor, BitXorAssign, 
    Shl, ShlAssign, Shr, ShrAssign, Not,
    Sub, Add, SubAssign, AddAssign, Index, IndexMut,
    Deref, DerefMut,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bitboard(pub u64);

/// Constants used to initialize bitboards
pub const PAWN_START: Bitboard = Bitboard(0x00_ff_00_00_00_00_ff_00);
pub const KNIGHT_START: Bitboard = Bitboard(0x42_00_00_00_00_00_00_42);
pub const BISHOP_START: Bitboard = Bitboard(0x24_00_00_00_00_00_00_24);
pub const ROOK_START: Bitboard = Bitboard(0x81_00_00_00_00_00_00_81);
pub const QUEEN_START: Bitboard = Bitboard(0x08_00_00_00_00_00_00_08);
pub const KING_START: Bitboard = Bitboard(0x10_00_00_00_00_00_00_10);
pub const WHITE_START: Bitboard = Bitboard(0x00_00_00_00_00_00_ff_ff);
pub const BLACK_START: Bitboard = Bitboard(0xff_ff_00_00_00_00_00_00);
pub const EMPTY_START: Bitboard = Bitboard(0x00_00_ff_ff_ff_ff_00_00);
pub const OCCUPIED_START: Bitboard = Bitboard(0xff_ff_00_00_00_00_ff_ff);

/// 1s everywhere except for the A file
pub const NOT_A_FILE: Bitboard = Bitboard(0xfe_fe_fe_fe_fe_fe_fe_fe);
/// 1s everywhere except for the H file
pub const NOT_H_FILE: Bitboard = Bitboard(0x7f_7f_7f_7f_7f_7f_7f_7f);

/// Rank masks
pub const RANK4: Bitboard = Bitboard(0x00_00_00_00_ff_00_00_00);
pub const RANK5: Bitboard = Bitboard(0x00_00_00_ff_00_00_00_00);

impl Bitboard {
    /// Shifts the bitboard `b` south one
    pub fn sout_one(b: Bitboard) -> Bitboard {
        b >> 8
    }

    /// Shifts the bitboard `b` north one
    pub fn nort_one(b: Bitboard) -> Bitboard {
        b << 8
    }

    /// Shifts the bitboard `b` east one
    pub fn east_one(b: Bitboard) -> Bitboard {
        (b << 1) & self::NOT_A_FILE
    }

    /// Shifts the bitboard `b` west one
    pub fn west_one(b: Bitboard) -> Bitboard {
        (b >> 1) & self::NOT_H_FILE
    }

    /// Shifts the bitboard `b` northeast one
    pub fn noea_one(b: Bitboard) -> Bitboard {
        (b << 9) & self::NOT_A_FILE
    }

    /// Shifts the bitboard `b` southeast one
    pub fn soea_one(b: Bitboard) -> Bitboard {
        (b >> 7) & self::NOT_A_FILE
    }

    /// Shifts the bitboard `b` southwest one
    pub fn sowe_one(b: Bitboard) -> Bitboard {
        (b >> 9) & self::NOT_H_FILE
    }

    /// Shifts the bitboard `b` northwest one
    pub fn nowe_one(b: Bitboard) -> Bitboard {
        (b << 7) & self::NOT_H_FILE
    }

    /// Rotates the bitboard `b` to the left by `s` bits
    pub fn rotate_left(Bitboard(b): Bitboard, s: u32) -> Bitboard {
        Bitboard(b.rotate_left(s))
    }

    /// Rotates the bitboard `b` to the right by `s` bits
    pub fn rotate_right(Bitboard(b): Bitboard, s: u32) -> Bitboard {
        Bitboard(b.rotate_right(s))
    }
    /// Returns the square the least significant 1 bit, or None
    /// if there is no 1 bit
    pub fn bit_scan(&self) -> Option<Square> {
        let trailing_zeros = self.0.trailing_zeros();
        if trailing_zeros == 64 { None } else { Some(FromPrimitive::from_u32(trailing_zeros).unwrap()) }
    }

    /// Returns the square of the most significant 1 bit, or None
    /// if there is no 1 bit
    pub fn bit_scan_reverse(&self) -> Option<Square> {
        let leading_zeros = self.0.leading_zeros();
        if leading_zeros == 64 { None } else { Some(FromPrimitive::from_u32(leading_zeros ^ 63).unwrap()) }
    }

    pub fn empty(&self) -> bool {
        self.0 == 0
    }

    pub fn occupied(&self) -> bool {
        self.0 != 0
    }
}

impl ToString for Bitboard {
    fn to_string(&self) -> String {
        let mut res = String::from("");
        for i in (0..8).rev() {
            let rank = (self.0 >> (i * 8)) & 0xff;
            for j in 0..8 {
                let square = (rank >> j) & 1;
                if square == 1 {
                    res += "1 ";
                } else {
                    res += ". ";
                }
            }
            res += "\n";
        }
        return res;
    }
}

/// Iterate over the squares set to one in the bitboard
impl Iterator for Bitboard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if *self == Bitboard(0) {
            return None;
        }
        let next_square: Square = self.bit_scan().unwrap();
        *self &= *self - Bitboard(1);
        Some(next_square)
    }
}

// Implementation of bitwise operations for bitboards
// Just use the operation on the inner u64
impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Self(self.0 & rhs.0)
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
        *self = Self(self.0 | rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 ^ rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl Shl<i32> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: i32) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl ShlAssign<i32> for Bitboard {
    fn shl_assign(&mut self, rhs: i32) {
        *self = Self(self.0 << rhs)
    }
}

impl Shr<i32> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: i32) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl ShrAssign<i32> for Bitboard {
    fn shr_assign(&mut self, rhs: i32) {
        *self = Self(self.0 >> rhs)
    }
}

impl Sub for Bitboard {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Bitboard {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Bitboard(self.0 - rhs.0);
    }
}

impl Add for Bitboard {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Bitboard {
    fn add_assign(&mut self, rhs: Self) {
        *self = Bitboard(self.0 - rhs.0);
    }
}
