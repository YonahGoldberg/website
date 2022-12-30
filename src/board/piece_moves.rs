use crate::board::{Board, Color, Piece, Bitboard, bitboard, Dir, Square};

use super::bitboard::RAY_ATTACKS;

impl Board {
    /// Returns a bitboard marking the squares pawns of color `c` can be 
    /// single pushed to under pseudo-legal move generation
    fn pawn_push_targets(&self, c: Color) -> Bitboard {
        ((self.piece_bb(c, Piece::Pawn) << 8) >> ((c as i32) << 4)) & self.empty_bb
    }
    /// Returns a bitboard marking the squares pawns of color `c` can be
    /// double pushed to under pseudo-legal move generation
    fn pawn_dpush_targets(&self, c: Color) -> Bitboard {
        let push_targets = self.pawn_push_targets(c);
        match c {
            Color::White => Bitboard::nort_one(push_targets) & self.empty_bb & bitboard::RANK4,
            Color::Black => Bitboard::sout_one(push_targets) & self.empty_bb & bitboard::RANK5,
        }
    }

    /// Returns a bitboard marking the squares of pawns of color `c` that can be
    /// single pushed under pseudo-legal move generation
    fn pawn_can_push(&self, c: Color) -> Bitboard {
        let piece_bb = self.piece_bb(c, Piece::Pawn);
        match c {
            Color::White => Bitboard::sout_one(self.empty_bb) & piece_bb,
            Color::Black => Bitboard::nort_one(self.empty_bb) & piece_bb,
        }
    }


    /// Returns a bitboard marking the squares of pawns of color `c` that can be
    /// double pushed under pseudo-legal move generation
    fn pawn_can_dpush(&self, c: Color) -> Bitboard {
        let piece_bb = self.piece_bb(c, Piece::Pawn);
        match c {
            Color::White => {
                let empty_rank3 = Bitboard::sout_one(self.empty_bb & bitboard::RANK4) & self.empty_bb;
                Bitboard::sout_one(empty_rank3) & piece_bb
            }
            Color::Black => {
                let empty_rank6 = Bitboard::nort_one(self.empty_bb & bitboard::RANK5) & self.empty_bb;
                Bitboard::sout_one(empty_rank6) & piece_bb
            }
        }
    }

    /// Returns a bitboard marking the squares pawns of color `c` can attack
    /// to the east under pseudo-legal move generation
    fn pawn_east_attacks(&self, c: Color) -> Bitboard {
        let piece_bb = self.piece_bb(c, Piece::Pawn);
        match c {
            Color::White => Bitboard::noea_one(piece_bb),
            Color::Black => Bitboard::soea_one(piece_bb),
        }
    }

    /// Returns a bitboard marking the squares pawns of color `c` can attack
    /// to the west under pseudo-legal move generation
    fn pawn_west_attacks(&self, c: Color) -> Bitboard {
        let piece_bb = self.piece_bb(c, Piece::Pawn);
        match c {
            Color::White => Bitboard::nowe_one(piece_bb),
            Color::Black => Bitboard::sowe_one(piece_bb),
        }
    }

    /// Returns a bitboard marking the squares pawns of color `c` can attack
    /// under pseudo-legal move generation
    fn pawn_attacks(&self, c: Color) -> Bitboard {
        self.pawn_west_attacks(c) | self.pawn_east_attacks(c)
    }

    /// Returns a bitboard marking the squares in which 2 pawns of color `c` can attack
    /// under pseudo-legal move generation
    fn pawn_dbl_attacks(&self, c: Color) -> Bitboard {
        self.pawn_west_attacks(c) & self.pawn_east_attacks(c)
    }

    /// Returns a bitboard marking the squares in which a single pawn of color `c` attacks
    /// under pseudo-legal move generation
    fn pawn_single_attacks(&self, c: Color) -> Bitboard {
        self.pawn_west_attacks(c) ^ self.pawn_east_attacks(c)
    }

    /// Returns a bitboard marking safe pawn squares. A safe pawn square
    /// for the player playing color `c` are the squares in which they have
    /// more pawns attacking than their opponent
    fn pawn_safe_sqares(&self, c: Color) -> Bitboard {
        self.pawn_dbl_attacks(c) | 
        !self.pawn_attacks(c.opp()) | 
        (self.pawn_single_attacks(c) & !self.pawn_dbl_attacks(c.opp()))
    }

    /// Returns a bitboard marking the pawns of color `c` that
    /// can capture another pawn to the east under pseudo-legal move generation
    fn pawn_can_capture_pawn_east(&self, c: Color) -> Bitboard {
        match c {
            Color::White => self.piece_bb(Color::White, Piece::Pawn) 
                & self.pawn_west_attacks(Color::Black),
            Color::Black => self.piece_bb(Color::Black, Piece::Pawn) 
                & self.pawn_east_attacks(Color::White),
        }
    }

    /// Returns a bitboard marking the pawns of color `c` that
    /// can capture another pawn to the west under pseudo-legal move generation
    fn pawn_can_capture_pawn_west(&self, c: Color) -> Bitboard {
        match c {
            Color::White => self.piece_bb(Color::White, Piece::Pawn)
                & self.pawn_east_attacks(Color::Black),
            Color::Black => self.piece_bb(Color::Black, Piece::Pawn)
                & self.pawn_east_attacks(Color::White),
        }
    }

    /// Returns a bitboard marking the pawns of color `c` that
    /// can capture another pawn in any direction under pseudo-legal move generation
    fn pawn_can_capture_pawn(&self, c: Color) -> Bitboard {
        self.piece_bb(c, Piece::Pawn) & self.pawn_attacks(c.opp())
    }

    fn ray_attacks(&self, d: Dir, s: Square) -> Bitboard {
        let mut attacks = bitboard::RAY_ATTACKS[d as usize][s as usize];
        let blocking = attacks & self.occupied_bb;
        let blocker = if d.pos() { blocking.bit_scan() } else { blocking.bit_scan_reverse() };
        if let Some(blocker) = blocker {
            attacks ^= RAY_ATTACKS[d as usize][blocker as usize];
        }
        attacks
    }

    fn diag_attacks(&self, s: Square) -> Bitboard {
        self.ray_attacks(Dir::Noea, s) | self.ray_attacks(Dir::Sowe, s)
    }

    fn anti_diag_attacks(&self, s: Square) -> Bitboard {
        self.ray_attacks(Dir::Nowe, s) | self.ray_attacks(Dir::Soea, s)
    }

    fn file_attacks(&self, s: Square) -> Bitboard {
        self.ray_attacks(Dir::Nort, s) | self.ray_attacks(Dir::Sout, s)
    }

    fn rank_attacks(&self, s: Square) -> Bitboard {
        self.ray_attacks(Dir::East, s) | self.ray_attacks(Dir::West, s)
    }

    fn bishop_attacks(&self, s: Square) -> Bitboard {
        self.diag_attacks(s) | self.anti_diag_attacks(s)
    }

    fn rook_attacks(&self, s: Square) -> Bitboard {
        self.file_attacks(s) | self.rank_attacks(s)
    }

    fn queen_attacks(&self, s: Square) -> Bitboard {
        self.rook_attacks(s) | self.bishop_attacks(s)
    }
}