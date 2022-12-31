use crate::board::{Board, Color, Piece, Bitboard, bitboard, Dir, Square};

impl Board {
    /// Returns a bitboard marking the squares pawns of color `c` can be 
    /// single pushed to under pseudo-legal move generation
    fn pawn_push_targets(&self, c: Color) -> Bitboard {
        ((self.piece_bb(Some(c), Piece::Pawn) << 8) >> ((c as i32) << 4)) & self.empty_bb
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
        let piece_bb = self.piece_bb(Some(c), Piece::Pawn);
        match c {
            Color::White => Bitboard::sout_one(self.empty_bb) & piece_bb,
            Color::Black => Bitboard::nort_one(self.empty_bb) & piece_bb,
        }
    }


    /// Returns a bitboard marking the squares of pawns of color `c` that can be
    /// double pushed under pseudo-legal move generation
    fn pawn_can_dpush(&self, c: Color) -> Bitboard {
        let piece_bb = self.piece_bb(Some(c), Piece::Pawn);
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
        let piece_bb = self.piece_bb(Some(c), Piece::Pawn);
        match c {
            Color::White => Bitboard::noea_one(piece_bb),
            Color::Black => Bitboard::soea_one(piece_bb),
        }
    }

    /// Returns a bitboard marking the squares pawns of color `c` can attack
    /// to the west under pseudo-legal move generation
    fn pawn_west_attacks(&self, c: Color) -> Bitboard {
        let piece_bb = self.piece_bb(Some(c), Piece::Pawn);
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
            Color::White => self.piece_bb(Some(Color::White), Piece::Pawn) 
                & self.pawn_west_attacks(Color::Black),
            Color::Black => self.piece_bb(Some(Color::Black), Piece::Pawn) 
                & self.pawn_east_attacks(Color::White),
        }
    }

    /// Returns a bitboard marking the pawns of color `c` that
    /// can capture another pawn to the west under pseudo-legal move generation
    fn pawn_can_capture_pawn_west(&self, c: Color) -> Bitboard {
        match c {
            Color::White => self.piece_bb(Some(Color::White), Piece::Pawn)
                & self.pawn_east_attacks(Color::Black),
            Color::Black => self.piece_bb(Some(Color::Black), Piece::Pawn)
                & self.pawn_east_attacks(Color::White),
        }
    }

    /// Returns a bitboard marking the pawns of color `c` that
    /// can capture another pawn in any direction under pseudo-legal move generation
    fn pawn_can_capture_pawn(&self, c: Color) -> Bitboard {
        self.piece_bb(Some(c), Piece::Pawn) & self.pawn_attacks(c.opp())
    }

    /// Returns a bitboard marking ray attacks in direction `d` from
    /// square `s`. Ray attacks flow in direction `d`, but stop when
    /// a piece blocks the ray
    fn ray_attacks(&self, d: Dir, s: Square, occupied_bb: Option<Bitboard>) -> Bitboard {
        let occupied_bb = match occupied_bb {
            Some(b) => b,
            None => self.occupied_bb,
        };
        let mut attacks = bitboard::RAY_ATTACKS[d as usize][s as usize];
        let blocking = attacks & self.occupied_bb;
        let blocker = if d.pos() { blocking.bit_scan() } else { blocking.bit_scan_reverse() };
        if let Some(blocker) = blocker {
            attacks ^= bitboard::RAY_ATTACKS[d as usize][blocker as usize];
        }
        attacks
    }

    /// Returns a bitboard marking diagonal attacks
    /// (positive slope) from square `s`
    fn diag_attacks(&self, s: Square, occupied_bb: Option<Bitboard>) -> Bitboard {
        self.ray_attacks(Dir::Noea, s, occupied_bb) | self.ray_attacks(Dir::Sowe, s, occupied_bb)
    }

    /// Returns a bitboard marking antidiagonal attacks
    /// (negative slope) from square `s`
    fn anti_diag_attacks(&self, s: Square, occupied_bb: Option<Bitboard>) -> Bitboard {
        self.ray_attacks(Dir::Nowe, s, occupied_bb) | self.ray_attacks(Dir::Soea, s, occupied_bb)
    }
    
    /// Returns a bitboard marking file attacks
    /// (same number) from square `s`
    fn file_attacks(&self, s: Square, occupied_bb: Option<Bitboard>) -> Bitboard {
        self.ray_attacks(Dir::Nort, s, occupied_bb) | self.ray_attacks(Dir::Sout, s, occupied_bb)
    }

    /// Returns a bitboard marking rank attacks
    /// (same letter) from square `s`
    fn rank_attacks(&self, s: Square, occupied_bb: Option<Bitboard>) -> Bitboard {
        self.ray_attacks(Dir::East, s, occupied_bb) | self.ray_attacks(Dir::West, s, occupied_bb)
    }

    /// Returns a bitboard marking bishop attacks
    /// from square `s`
    fn bishop_attacks(&self, s: Square, occupied_bb: Option<Bitboard>) -> Bitboard {
        self.diag_attacks(s, occupied_bb) | self.anti_diag_attacks(s, occupied_bb)
    }

    /// Returns a bitboard marking rook attacks
    /// from square `s`
    fn rook_attacks(&self, s: Square, occupied_bb: Option<Bitboard>) -> Bitboard {
        self.file_attacks(s, occupied_bb) | self.rank_attacks(s, occupied_bb)
    }

    /// Returns a bitboard marking queen attacks
    /// from square `s`
    fn queen_attacks(&self, s: Square, occupied_bb: Option<Bitboard>) -> Bitboard {
        self.rook_attacks(s, occupied_bb) | self.bishop_attacks(s, occupied_bb)
    }

    /// Returns a bitboard marking squares with pieces present that
    /// attack square `s`
    fn attacks_to(&self, s: Square) -> Bitboard {
        bitboard::PAWN_ATTACKS[Color::White as usize][s as usize] & self.piece_bb(Some(Color::Black), Piece::Pawn) |
        bitboard::PAWN_ATTACKS[Color::Black as usize][s as usize] & self.piece_bb(Some(Color::White), Piece::Pawn) |
        bitboard::KNIGHT_ATTACKS[s as usize] & self.piece_bb(None, Piece::Knight) |
        bitboard::KING_ATTACKS[s as usize] & self.piece_bb(None, Piece::King) |
        self.bishop_attacks(s, None) & (self.piece_bb(None, Piece::Bishop) | self.piece_bb(None, Piece::Queen)) |
        self.rook_attacks(s, None) & (self.piece_bb(None, Piece::Rook) | self.piece_bb(None, Piece::Queen))
    }

    /// Returns true if square `s` is attacked by side `by_side`, otherwise false
    fn attacked(&self, s: Square, by_side: Color) -> bool {
        let pawns = self.piece_bb(Some(by_side), Piece::Pawn);
        if bitboard::PAWN_ATTACKS[by_side.opp() as usize][s as usize] & pawns != Bitboard(0) {
            return true;
        }
        let knights = self.piece_bb(Some(by_side), Piece::Knight);
        if bitboard::KNIGHT_ATTACKS[s as usize] & knights != Bitboard(0) {
            return true;
        }
        let king = self.piece_bb(Some(by_side), Piece::King);
        if bitboard::KING_ATTACKS[s as usize] & king != Bitboard(0) {
            return true;
        }
        let bishops_queen = self.piece_bb(Some(by_side), Piece::Bishop)
            | self.piece_bb(Some(by_side), Piece::Queen);
        if self.bishop_attacks(s, None) & bishops_queen != Bitboard(0) {
            return true;
        }

        let rooks_queen = self.piece_bb(Some(by_side), Piece::Rook)
            | self.piece_bb(Some(by_side), Piece::Queen);
        if self.rook_attacks(s, None) & rooks_queen != Bitboard(0) {
            return true;
        }
        false
    }

    /// Returns a bitboard marking the squares a rook on `rook_square` attacks
    /// with one piece allowed to be xrayed through in each ray direction. `blockers`
    /// specifies the set of squares in which a piece may block a ray.
    /// This function is useful for finding pins.
    fn xray_rook_attacks(&self, blockers: Bitboard, rook_square: Square) -> Bitboard {
        let attacks = self.rook_attacks(rook_square, None);
        // Pieces in the way of the ray attack
        let actual_blockers = attacks & blockers;
        // Symmetric difference between the original attack ray with all the blockers
        // and the ray if you take away the first blocker
        attacks ^ self.rook_attacks(rook_square, Some(actual_blockers ^ self.occupied_bb))
    }

    /// Returns a bitboard marking the squares a bishop on `bishop_square` attacks
    /// with one piece allowed to be xrayed through in each ray direction. `blockers`
    /// specifies the set of squares in which a piece may block a ray.
    /// This function is useful for finding pins. 
    fn xray_bishop_attacks(&self, blockers: Bitboard, bishop_square: Square) -> Bitboard {
        let attacks = self.bishop_attacks(bishop_square, None);
        // Pieces in the way of the ray attack
        let actual_blockers = attacks & blockers;
        // Symmetric difference between the original attack ray with all the blockers
        // and the ray if you take away the first blocker
        attacks ^ self.bishop_attacks(bishop_square, Some(actual_blockers ^ self.occupied_bb))
    }
}