pub mod bitboard;
mod cmove;
use bitboard::Bitboard;
use num::FromPrimitive;
use cmove::Cmove;

/// The colors of pieces
use Color::*;
#[derive(Clone, Copy, FromPrimitive)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn op(&self) -> Color {
        match self {
            White => Black,
            Black => White,
        }
    }
}

/// All chess piece types
use Piece::*;
#[derive(Clone, Copy, FromPrimitive)]
pub enum Piece {
    Pawn, Knight, Bishop, Rook, Queen, King,
}

/// All eight cardinal directions
use Dir::*;
#[derive(Clone, Copy, FromPrimitive)]
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
use Square::*;
#[derive(Clone, Copy, FromPrimitive)]
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
    fn as_bitboard(&self) -> Bitboard {
        Bitboard(1) << *self as i32
    }
}

/// The main `Board` struct, which contains 10 bitboards
/// The `piece_bb` array first contains bitboards marking the presense of
/// pawns, knights, bishops, rooks, queens, and kings respectively, 
/// regardless of color. The 7th and 8th boards in the array mark the presense
/// of white and black pieces respectively, which can be intersected with
/// the previous indexed boards to obtain the location of only white or only
/// black pieces.
/// 
/// The `empty_bb` and `occupied_bb` boards mark the absense of and the presense 
/// of pieces, respectively.
pub struct Board {
    piece_bb: [Bitboard; 8],
    empty_bb: Bitboard,
    occupied_bb: Bitboard,
}

impl Board {
    /// Creates a new Bitboard struct with beginning piece
    /// placements for each bitboard
    pub fn new() -> Board {
        Board {
            piece_bb: [
                bitboard::PAWN_START,
                bitboard::KNIGHT_START,
                bitboard::BISHOP_START,
                bitboard::ROOK_START,
                bitboard::QUEEN_START,
                bitboard::KING_START,
                bitboard::WHITE_START,
                bitboard::BLACK_START,
            ],
            empty_bb: bitboard::EMPTY_START,
            occupied_bb: bitboard::OCCUPIED_START,
        }
    }

    /// Returns the appropriate piece bitboard for
    /// piece `p` intersected with the piece bitboard
    /// for the color `c`, if `c` is not `None`
    pub fn piece_bb(&self, c: Option<Color>, p: Piece) -> Bitboard {
        let intersection = match c {
            Some(c) => self.piece_bb[6 + c as usize],
            None => Bitboard(!0),
        };
        self.piece_bb[p as usize] & intersection
    }

    /// Returns a bitboard of all the pieces of color `c`
    fn color_bb(&self, c: Color) -> Bitboard {
        self.piece_bb[6 + c as usize] & self.occupied_bb
    }

    /// Returns a bitboard marking the squares pawns of color `c` can be 
    /// single pushed to under pseudo-legal move generation
    fn pawn_push_targets(&self, c: Color) -> Bitboard {
        ((self.piece_bb(Some(c), Pawn) << 8) >> ((c as i32) << 4)) & self.empty_bb
    }
    /// Returns a bitboard marking the squares pawns of color `c` can be
    /// double pushed to under pseudo-legal move generation
    fn pawn_dpush_targets(&self, c: Color) -> Bitboard {
        let push_targets = self.pawn_push_targets(c);
        match c {
            White => Bitboard::nort_one(push_targets) & self.empty_bb & bitboard::RANK4,
            Black => Bitboard::sout_one(push_targets) & self.empty_bb & bitboard::RANK5,
        }
    }

    /// Returns a bitboard marking the squares of pawns of color `c` that can be
    /// single pushed under pseudo-legal move generation
    fn pawn_can_push(&self, c: Color) -> Bitboard {
        let piece_bb = self.piece_bb(Some(c), Pawn);
        match c {
            White => Bitboard::sout_one(self.empty_bb) & piece_bb,
            Black => Bitboard::nort_one(self.empty_bb) & piece_bb,
        }
    }


    /// Returns a bitboard marking the squares of pawns of color `c` that can be
    /// double pushed under pseudo-legal move generation
    fn pawn_can_dpush(&self, c: Color) -> Bitboard {
        let piece_bb = self.piece_bb(Some(c), Pawn);
        match c {
            White => {
                let empty_rank3 = Bitboard::sout_one(self.empty_bb & bitboard::RANK4) & self.empty_bb;
                Bitboard::sout_one(empty_rank3) & piece_bb
            }
            Black => {
                let empty_rank6 = Bitboard::nort_one(self.empty_bb & bitboard::RANK5) & self.empty_bb;
                Bitboard::sout_one(empty_rank6) & piece_bb
            }
        }
    }

    /// Returns a bitboard marking the squares pawns of color `c` can attack
    /// to the east under pseudo-legal move generation
    fn pawn_east_attacks(&self, c: Color) -> Bitboard {
        let piece_bb = self.piece_bb(Some(c), Pawn);
        match c {
            White => Bitboard::noea_one(piece_bb),
            Black => Bitboard::soea_one(piece_bb),
        }
    }

    /// Returns a bitboard marking the squares pawns of color `c` can attack
    /// to the west under pseudo-legal move generation
    fn pawn_west_attacks(&self, c: Color) -> Bitboard {
        let piece_bb = self.piece_bb(Some(c), Pawn);
        match c {
            White => Bitboard::nowe_one(piece_bb),
            Black => Bitboard::sowe_one(piece_bb),
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
    /// more pawns attacking than their oponent
    fn pawn_safe_sqares(&self, c: Color) -> Bitboard {
        self.pawn_dbl_attacks(c) | 
        !self.pawn_attacks(c.op()) | 
        (self.pawn_single_attacks(c) & !self.pawn_dbl_attacks(c.op()))
    }

    /// Returns a bitboard marking the pawns of color `c` that
    /// can capture another pawn to the east under pseudo-legal move generation
    fn pawn_can_capture_pawn_east(&self, c: Color) -> Bitboard {
        match c {
            White => self.piece_bb(Some(White), Pawn) 
                & self.pawn_west_attacks(Black),
            Black => self.piece_bb(Some(Black), Pawn) 
                & self.pawn_east_attacks(White),
        }
    }

    /// Returns a bitboard marking the pawns of color `c` that
    /// can capture another pawn to the west under pseudo-legal move generation
    fn pawn_can_capture_pawn_west(&self, c: Color) -> Bitboard {
        match c {
            White => self.piece_bb(Some(White), Pawn)
                & self.pawn_east_attacks(Black),
            Black => self.piece_bb(Some(Black), Pawn)
                & self.pawn_east_attacks(White),
        }
    }

    /// Returns a bitboard marking the pawns of color `c` that
    /// can capture another pawn in any direction under pseudo-legal move generation
    fn pawn_can_capture_pawn(&self, c: Color) -> Bitboard {
        self.piece_bb(Some(c), Pawn) & self.pawn_attacks(c.op())
    }

    /// Returns a bitboard marking ray attacks in direction `d` from
    /// square `s`. Ray attacks flow in direction `d`, but stop when
    /// a piece blocks the ray
    pub fn ray_attacks(&self, d: Dir, s: Square, occupied_bb: Option<Bitboard>) -> Bitboard {
        let occupied_bb = match occupied_bb {
            Some(b) => b,
            None => self.occupied_bb,
        };
        let mut attacks = bitboard::RAY_ATTACKS[d as usize][s as usize];
        let blocking = attacks & occupied_bb;
        let blocker = if d.pos() { blocking.bit_scan() } else { blocking.bit_scan_reverse() };
        if let Some(blocker) = blocker {
            attacks ^= bitboard::RAY_ATTACKS[d as usize][blocker as usize];
        }
        attacks
    }

    /// Returns a bitboard marking diagonal attacks
    /// (positive slope) from square `s`
    fn diag_attacks(&self, s: Square, occupied_bb: Option<Bitboard>) -> Bitboard {
        self.ray_attacks(Noea, s, occupied_bb) | self.ray_attacks(Sowe, s, occupied_bb)
    }

    /// Returns a bitboard marking antidiagonal attacks
    /// (negative slope) from square `s`
    fn anti_diag_attacks(&self, s: Square, occupied_bb: Option<Bitboard>) -> Bitboard {
        self.ray_attacks(Nowe, s, occupied_bb) | self.ray_attacks(Soea, s, occupied_bb)
    }
    
    /// Returns a bitboard marking file attacks
    /// (same number) from square `s`
    fn file_attacks(&self, s: Square, occupied_bb: Option<Bitboard>) -> Bitboard {
        self.ray_attacks(Nort, s, occupied_bb) | self.ray_attacks(Sout, s, occupied_bb)
    }

    /// Returns a bitboard marking rank attacks
    /// (same letter) from square `s`
    fn rank_attacks(&self, s: Square, occupied_bb: Option<Bitboard>) -> Bitboard {
        self.ray_attacks(East, s, occupied_bb) | self.ray_attacks(West, s, occupied_bb)
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
        bitboard::PAWN_ATTACKS[White as usize][s as usize] & self.piece_bb(Some(Black), Pawn) |
        bitboard::PAWN_ATTACKS[Black as usize][s as usize] & self.piece_bb(Some(White), Pawn) |
        bitboard::KNIGHT_ATTACKS[s as usize] & self.piece_bb(None, Knight) |
        bitboard::KING_ATTACKS[s as usize] & self.piece_bb(None, King) |
        self.bishop_attacks(s, None) & (self.piece_bb(None, Bishop) | self.piece_bb(None, Queen)) |
        self.rook_attacks(s, None) & (self.piece_bb(None, Rook) | self.piece_bb(None, Queen))
    }

    /// Returns true if square `s` is attacked by side `by_side`, otherwise false
    fn attacked(&self, s: Square, by_side: Color) -> bool {
        let pawns = self.piece_bb(Some(by_side), Pawn);
        if bitboard::PAWN_ATTACKS[by_side.op() as usize][s as usize] & pawns != Bitboard(0) {
            return true;
        }
        let knights = self.piece_bb(Some(by_side), Knight);
        if bitboard::KNIGHT_ATTACKS[s as usize] & knights != Bitboard(0) {
            return true;
        }
        let king = self.piece_bb(Some(by_side), King);
        if bitboard::KING_ATTACKS[s as usize] & king != Bitboard(0) {
            return true;
        }
        let bishops_queen = self.piece_bb(Some(by_side), Bishop)
            | self.piece_bb(Some(by_side), Queen);
        if self.bishop_attacks(s, None) & bishops_queen != Bitboard(0) {
            return true;
        }

        let rooks_queen = self.piece_bb(Some(by_side), Rook)
            | self.piece_bb(Some(by_side), Queen);
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

    /// Returns a bitboard marking the squares in between `from` and `to` along
    /// a ray in one of the eight cardinal directions. Returns an empty bitboard
    /// if `from` and `to` are not along a cardinal direction.
    fn in_between(from: Square, to: Square) -> Bitboard {
        bitboard::IN_BETWEEN[from as usize][to as usize]
    }

    /// Returns a bitboard marking the pins on color `on_color` with king on
    /// square `king_square`
    fn pins(&self, on_color: Color, king_square: Square) -> Bitboard {
        let op_rq = self.piece_bb(Some(on_color.op()), Rook) 
            | self.piece_bb(Some(on_color.op()), Queen);
        let mut pinned: Bitboard = Bitboard(0);
        // xray rook attacks from our king, past our pieces as blockers, 
        // to oponent's pieces
        let mut pinners = self.xray_rook_attacks(
            self.color_bb(on_color), 
            king_square
        ) & op_rq;
        // for each pinner
        while pinners > Bitboard(0) {
            // OK to unwrap because by loop guard there is a pinner
            let sq: Square = FromPrimitive::from_u32(pinners.bit_scan().unwrap()).unwrap();
            // The pinned pieces are between the pinners and the king square
            pinned |= Board::in_between(sq, king_square) & self.color_bb(on_color);
            // Mask out prev pinner
            pinners &= pinners - Bitboard(1);
        }

        // Same thing but for bishop rays
        let op_bq = self.piece_bb(Some(on_color.op()), Bishop) 
            | self.piece_bb(Some(on_color.op()), Queen);
        let mut pinned: Bitboard = Bitboard(0);
        // xray rook attacks from our king, past our pieces as blockers, 
        // to oponent's pieces
        let mut pinners = self.xray_bishop_attacks(
            self.color_bb(on_color), 
            king_square
        ) & op_bq;
        // for each pinner
        while pinners > Bitboard(0) {
            // OK to unwrap because by loop guard there is a pinner
            let sq: Square = FromPrimitive::from_u32(pinners.bit_scan().unwrap()).unwrap();
            // The pinned pieces are between the pinners and the king square
            pinned |= Board::in_between(sq, king_square) & self.color_bb(on_color);
            // Mask out prev pinner
            pinners &= pinners - Bitboard(1);
        }

        pinned
    }

    /// Makes the move `m`, updating this board's internal state
    /// This function assumes `m` is a valid move
    pub fn make_move_mut(&mut self, m: Cmove) {
        // a one on the from square, else zeroes
        let from_bb = m.get_from().as_bitboard();
        // a one on the to square, else zeroes
        let to_bb = m.get_to().as_bitboard();
        // ones on the from and to squares, else zeroes
        let from_to_bb = from_bb ^ to_bb;
        // Assuming this is a valid move and there is a piece on the square
        let (piece, color) = self.piece_on_square(m.get_from()).unwrap();

        if m.is_capture() {
            // If captured piece is different than piece, this is correct,
            // otherwise the to square will be set to 0 instead of 1
            self.piece_bb[piece as usize] ^= from_to_bb;
            // Update from piece's color bit
            self.piece_bb[6 + color as usize] ^= from_to_bb;

            let (captured_piece, captured_color) = self.piece_on_square(m.get_to()).unwrap();
            // If captured piece is different than piece, we update
            // captured piece bitboard normally, otherwise we flip the bit that was incorrect
            self.piece_bb[captured_piece as usize] ^= to_bb;
            // update captured color bitboard
            self.piece_bb[6 + captured_color as usize] ^= to_bb;
            // occupied bitboard has new empty square
            self.occupied_bb ^= from_bb;
            // empty bitboard has new empty square
            self.empty_bb ^= from_bb;
        } else {
            // update piece bitboard
            self.piece_bb[piece as usize] ^= from_to_bb;
            // update color bitboard
            self.piece_bb[6 + color as usize] ^= from_to_bb;
            // occupied bitboard has new empty square
            self.occupied_bb ^= from_bb;
            // empty bitboard has new empty square
            self.empty_bb ^= from_bb; 
        }
    }

    /// Returns `Some(p)` if there exists a piece `p` on square `s`,
    /// otherwise None
    pub fn piece_on_square(&self, s: Square) -> Option<(Piece, Color)> {
        let bb = s.as_bitboard();

        let c = if bb & self.color_bb(White) > Bitboard(0) { 
            White 
        } else if bb & self.color_bb(Black) > Bitboard(0) { 
            Black 
        } else { 
            return None;
        };

        for i in 0..6 {
            if self.piece_bb[i] & bb > Bitboard(0) {
                let p = FromPrimitive::from_usize(i).unwrap();
                return Some((p, c));
            }
        }

        // Shouldn't get here
        panic!();
    }
}