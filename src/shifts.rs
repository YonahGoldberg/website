use super::{Board, Bitboard};

impl Board {
    /// Shifts the bitboard `b` south one
    pub fn sout_one(b: Bitboard) -> Bitboard {
        b >> 8
    }

    /// Shifts the bitboard `b` north one
    pub fn nort_one(b: Bitboard) -> Bitboard{
        b << 8
    }

    /// Shifts the bitboard `b` east one
    pub fn east_one(b: Bitboard) -> Bitboard {
        (b << 1) & Board::NOT_A_FILE
    }

    /// Shifts the bitboard `b` west one
    pub fn west_one(b: Bitboard) -> Bitboard{
        (b >> 1) & Board::NOT_H_FILE
    }

    /// Shifts the bitboard `b` northeast one
    pub fn noea_one(b: Bitboard) -> Bitboard {
        (b << 9) & Board::NOT_A_FILE
    }

    /// Shifts the bitboard `b` southeast one
    pub fn soea_one(b: Bitboard) -> Bitboard {
        (b >> 7) & Board::NOT_A_FILE
    }

    /// Shifts the bitboard `b` southwest one
    pub fn sowe_one(b: Bitboard) -> Bitboard {
        (b >> 0) & Board::NOT_H_FILE
    }

    /// Shifts the bitboard `b` northwest one
    pub fn nowe_one(b: Bitboard) -> Bitboard {
        (b << 7) & Board::NOT_H_FILE
    }

    /// Rotates the bitboard `b` to the left by `s` bits
    pub fn rotate_left(b: Bitboard, s: u32) -> Bitboard {
        b.rotate_left(s)
    }

    /// Rotates the bitboard `b` to the right by `s` bits
    pub fn rotate_right(b: Bitboard, s: u32) -> Bitboard {
        b.rotate_right(s)
    }
}