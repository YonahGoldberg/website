use super::{Board, Bitboard, Color, Piece};

impl Board {
    /// Returns the appropriate piece bitboard for
    /// piece `p` of color `c`.
    fn piece_bb(&self, c: Color, p: Piece) -> Bitboard {
        self.piece_bb[p.num()] & self.piece_bb[6 + c.num() as usize]
    }

    /// Returns a bitboard marking the squares pawns of color `c` can be 
    /// single pushed to under pseudo-legal move generation
    fn pawn_push_targets(&self, c: Color) -> Bitboard {
        ((self.piece_bb(c, Piece::Pawn) << 8) >> (c.num() << 4)) & self.empty_bb
    }
    /// Returns a bitboard marking the squares pawns of color `c` can be
    /// double pushed to under pseudo-legal move generation
    fn pawn_dpush_targets(&self, c: Color) -> Bitboard {
        let push_targets = self.pawn_push_targets(c);
        match c {
            Color::White => Board::nort_one(push_targets) & self.empty_bb & Board::RANK4,
            Color::Black => Board::sout_one(push_targets) & self.empty_bb & Board::RANK5,
        }
    }

    /// Returns a bitboard marking the squares of pawns of color `c` that can be
    /// single pushed under pseudo-legal move generation
    fn pawn_can_push(&self, c: Color) -> Bitboard {
        let piece_bb = self.piece_bb(c, Piece::Pawn);
        match c {
            Color::White => Board::sout_one(self.empty_bb) & piece_bb,
            Color::Black => Board::nort_one(self.empty_bb) & piece_bb,
        }
    }


    /// Returns a bitboard marking the squares of pawns of color `c` that can be
    /// double pushed under pseudo-legal move generation
    fn pawn_can_dpush(&self, c: Color) -> Bitboard {
        let piece_bb = self.piece_bb(c, Piece::Pawn);
        match c {
            Color::White => {
                let empty_rank3 = Board::sout_one(self.empty_bb & Board::RANK4) & self.empty_bb;
                Board::sout_one(empty_rank3) & piece_bb
            }
            Color::Black => {
                let empty_rank6 = Board::nort_one(self.empty_bb & Board::RANK5) & self.empty_bb;
                Board::sout_one(empty_rank6) & piece_bb
            }
        }
    }

    /// Returns a bitboard marking the squares pawns of color `c` can attack
    /// to the east under pseudo-legal move generation
    fn pawn_east_attacks(&self, c: Color) -> Bitboard {
        let piece_bb = self.piece_bb(c, Piece::Pawn);
        match c {
            Color::White => Board::noea_one(piece_bb),
            Color::Black => Board::soea_one(piece_bb),
        }
    }

    /// Returns a bitboard marking the squares pawns of color `c` can attack
    /// to the west under pseudo-legal move generation
    fn pawn_west_attacks(&self, c: Color) -> Bitboard {
        let piece_bb = self.piece_bb(c, Piece::Pawn);
        match c {
            Color::White => Board::nowe_one(piece_bb),
            Color::Black => Board::sowe_one(piece_bb),
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
}