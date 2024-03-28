/// The main `Board` struct, which contains 11 bitboards, a fifty move rule
/// counter, and castling rights
///
/// # Fields
///
/// * `piece_bb` - contains bitboards marking the presense of
/// pawns, knights, bishops, rooks, queens, and kings respectively,
/// regardless of color. The 7th and 8th boards in the array mark the presense
/// of white and black pieces respectively, which can be intersected with
/// the previous indexed boards to obtain the location of only white or only
/// black pieces.
/// * `empty_bb` - marks the absense of pieces
/// * `occupied_bb` - makrs the presence of pieces
/// * `en_passant_bb` - marks the pawns that can be captured by en passant
/// (they just double pushed)
/// * `fifty_move_rule_counter` - number of plies so far. After 100 plies with no
/// pawn move or capture, the game is an automatic draw
/// * `castling_rights` - starting from LSB, marks whether castling is possible on
/// white king-side, white queen-side, black king-side, black queen-side
use super::bitboard::{self, Bitboard};
use super::cmove::{self, CMove};
use super::tables;
use super::utils::{CPiece, Color, Dir, Piece, Square};
use num_traits::FromPrimitive;
use Color::*;
use Dir::*;
use Piece::*;

struct CreateBoardError;

pub struct Board {
    piece_bb: [Bitboard; 8],
    empty_bb: Bitboard,
    occupied_bb: Bitboard,
    en_passant_bb: Bitboard,
    fifty_move_rule_counter: u8,
    castling_rights: u8,
}
// Constants for masking out castling rights
const WKING_SIDE_MASK: u8 = 1;
const WQUEEN_SIDE_MASK: u8 = 2;
const BKING_SIDE_MASK: u8 = 4;
const BQUEEN_SIDE_MASK: u8 = 8;

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
            en_passant_bb: Bitboard(0),
            fifty_move_rule_counter: 0,
            castling_rights: 0,
        }
    }

    pub fn from_piece_list(piece_list: &Vec<Option<CPiece>>) -> Result<Self, CreateBoardError> {
        if piece_list.len() != 64 {
            return Err(CreateBoardError);
        }

        let mut piece_bb: [Bitboard; 8] = [Bitboard(0); 8];
        let mut occupied_bb = Bitboard(0);

        for i in 0..63 {
            if let Some(CPiece(piece, color)) = piece_list[i] {
                let square_bb = Bitboard(1 << i);
                piece_bb[piece as usize] |= square_bb;
                piece_bb[6 + color as usize] |= square_bb;
                occupied_bb |= square_bb;
            }
        }

        let empty_bb = !occupied_bb;

        Ok(Board {
            piece_bb,
            empty_bb,
            occupied_bb,
            en_passant_bb: Bitboard(0),
            fifty_move_rule_counter: 0,
            castling_rights: 0,
        })
    }

    pub fn to_piece_list(&self) -> Vec<Option<CPiece>> {
        (0..63)
            .map(|num| FromPrimitive::from_i32(num).unwrap())
            .map(|s| self.piece_on_square(s))
            .collect()
    }

    // pub fn from_fen(fen: String) -> Result<Self, CreateBoardError> {
    //     let parts = fen.split(" ").collect::<Vec>();
    //     let pieces = parts[0];
    // }

    /// Returns the appropriate piece bitboard for
    /// piece `p` intersected with the piece bitboard
    /// for the color `c`, if `c` is not `None`
    fn piece_bb(&self, c: Option<Color>, p: Piece) -> Bitboard {
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
    fn pawns_can_push(&self, c: Color) -> Bitboard {
        let piece_bb = self.piece_bb(Some(c), Pawn);
        match c {
            White => Bitboard::sout_one(self.empty_bb) & piece_bb,
            Black => Bitboard::nort_one(self.empty_bb) & piece_bb,
        }
    }

    /// Returns a bitboard marking the squares of pawns of color `c` that can be
    /// double pushed under pseudo-legal move generation
    fn pawns_can_dpush(&self, c: Color) -> Bitboard {
        let piece_bb = self.piece_bb(Some(c), Pawn);
        match c {
            White => {
                let empty_rank3 =
                    Bitboard::sout_one(self.empty_bb & bitboard::RANK4) & self.empty_bb;
                Bitboard::sout_one(empty_rank3) & piece_bb
            }
            Black => {
                let empty_rank6 =
                    Bitboard::nort_one(self.empty_bb & bitboard::RANK5) & self.empty_bb;
                Bitboard::sout_one(empty_rank6) & piece_bb
            }
        }
    }

    /// Returns a bitboard marking the squares pawns of color `c` can attack
    /// to the east under pseudo-legal move generation
    fn pawn_east_attack_squares(&self, c: Color) -> Bitboard {
        let piece_bb = self.piece_bb(Some(c), Pawn);
        match c {
            White => Bitboard::noea_one(piece_bb),
            Black => Bitboard::soea_one(piece_bb),
        }
    }

    /// Returns a bitboard marking the squares pawns of color `c` can attack
    /// to the west under pseudo-legal move generation
    fn pawn_west_attack_squares(&self, c: Color) -> Bitboard {
        let piece_bb = self.piece_bb(Some(c), Pawn);
        match c {
            White => Bitboard::nowe_one(piece_bb),
            Black => Bitboard::sowe_one(piece_bb),
        }
    }

    /// Returns a bitboard marking the squares pawns of color `c` can attack
    /// under pseudo-legal move generation
    fn pawn_attack_squares(&self, c: Color) -> Bitboard {
        self.pawn_west_attack_squares(c) | self.pawn_east_attack_squares(c)
    }

    /// Returns a bitboard marking the squares in which 2 pawns of color `c` can attack
    /// under pseudo-legal move generation
    fn pawn_dbl_attack_squares(&self, c: Color) -> Bitboard {
        self.pawn_west_attack_squares(c) & self.pawn_east_attack_squares(c)
    }

    /// Returns a bitboard marking the squares in which a single pawn of color `c` attacks
    /// under pseudo-legal move generation
    fn pawn_single_attack_squares(&self, c: Color) -> Bitboard {
        self.pawn_west_attack_squares(c) ^ self.pawn_east_attack_squares(c)
    }

    /// Returns a bitboard marking safe pawn squares. A safe pawn square
    /// for the player playing color `c` are the squares in which they have
    /// more pawns attacking than their oponent
    fn pawn_safe_sqares(&self, c: Color) -> Bitboard {
        self.pawn_dbl_attack_squares(c)
            | !self.pawn_attack_squares(!c)
            | (self.pawn_single_attack_squares(c) & !self.pawn_dbl_attack_squares(!c))
    }

    /// Returns a bitboard marking the pawns of color `c` that
    /// can capture another pawn to the east under pseudo-legal move generation
    fn pawns_can_capture_pawn_east(&self, c: Color) -> Bitboard {
        match c {
            White => self.piece_bb(Some(White), Pawn) & self.pawn_west_attack_squares(Black),
            Black => self.piece_bb(Some(Black), Pawn) & self.pawn_east_attack_squares(White),
        }
    }

    /// Returns a bitboard marking the pawns of color `c` that
    /// can capture another pawn to the west under pseudo-legal move generation
    fn pawns_can_capture_pawn_west(&self, c: Color) -> Bitboard {
        match c {
            White => self.piece_bb(Some(White), Pawn) & self.pawn_east_attack_squares(Black),
            Black => self.piece_bb(Some(Black), Pawn) & self.pawn_east_attack_squares(White),
        }
    }

    /// Returns a bitboard marking the pawns of color `c` that
    /// can capture another pawn in any direction under pseudo-legal move generation
    fn pawns_can_capture_pawn(&self, c: Color) -> Bitboard {
        self.piece_bb(Some(c), Pawn) & self.pawn_attack_squares(!c)
    }

    /// Returns a bitboard marking ray attacks in direction `d` from
    /// square `s`. Ray attacks flow in direction `d`, but stop when
    /// a piece blocks the ray. The attack set includes the stopping piece.
    fn ray_attacks(&self, d: Dir, s: Square, occupied_bb: Option<Bitboard>) -> Bitboard {
        let occupied_bb = match occupied_bb {
            Some(b) => b,
            None => self.occupied_bb,
        };
        let mut attacks = tables::RAY_ATTACKS[d as usize][s as usize];
        let blocking = attacks & occupied_bb;
        let blocker = if d.pos() {
            blocking.bit_scan()
        } else {
            blocking.bit_scan_reverse()
        };
        if let Some(blocker) = blocker {
            attacks ^= tables::RAY_ATTACKS[d as usize][blocker as usize];
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
    /// from square `s` under pseudo-legal move generation
    fn bishop_attacks(&self, s: Square, occupied_bb: Option<Bitboard>) -> Bitboard {
        self.diag_attacks(s, occupied_bb) | self.anti_diag_attacks(s, occupied_bb)
    }

    /// Returns a bitboard marking rook attacks
    /// from square `s` under pseudo-legal move generation
    fn rook_attacks(&self, s: Square, occupied_bb: Option<Bitboard>) -> Bitboard {
        self.file_attacks(s, occupied_bb) | self.rank_attacks(s, occupied_bb)
    }

    /// Returns a bitboard marking queen attacks
    /// from square `s` under pseudo-legal move generation
    fn queen_attacks(&self, s: Square, occupied_bb: Option<Bitboard>) -> Bitboard {
        self.rook_attacks(s, occupied_bb) | self.bishop_attacks(s, occupied_bb)
    }

    /// Returns a bitboard marking pawn attacks
    /// from square `s` of a pawn of color `c` under pseudo-legal move generation
    fn pawn_attacks(s: Square, c: Color) -> Bitboard {
        tables::PAWN_ATTACKS[c as usize][s as usize]
    }

    /// Returns a bitboard marking knight attacks
    /// from square `s` under pseudo-legal move generation
    fn knight_attacks(s: Square) -> Bitboard {
        tables::KNIGHT_ATTACKS[s as usize]
    }

    /// Returns a bitboard marking king attacks
    /// from square `s` under pseudo-legal move generation
    fn king_attacks(s: Square) -> Bitboard {
        tables::KING_ATTACKS[s as usize]
    }

    /// Returns a bitboard marking squares with pieces present that
    /// attack square `s` under pseudo-legal move generation
    fn attacks_to(&self, s: Square, by_color: Color) -> Bitboard {
        self.color_bb(by_color)
            & (Board::pawn_attacks(s, !by_color) & self.piece_bb(None, Pawn)
                | Board::knight_attacks(s) & self.piece_bb(None, Knight)
                | Board::king_attacks(s) & self.piece_bb(None, King)
                | self.bishop_attacks(s, None)
                    & (self.piece_bb(None, Bishop) | self.piece_bb(None, Queen))
                | self.rook_attacks(s, None)
                    & (self.piece_bb(None, Rook) | self.piece_bb(None, Queen)))
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
        tables::IN_BETWEEN[from as usize][to as usize]
    }

    /// Returns a bitboard marking the pins on color `on_color` with king on
    /// square `king_square`
    fn pins(&self, on_color: Color, king_square: Square) -> Bitboard {
        let op_rq = self.piece_bb(Some(!on_color), Rook) | self.piece_bb(Some(!on_color), Queen);
        let mut pinned: Bitboard = Bitboard(0);
        // xray rook attacks from our king, past our pieces as blockers,
        // to oponent's pieces
        let pinners = self.xray_rook_attacks(self.color_bb(on_color), king_square) & op_rq;
        // for each pinner
        for sq in pinners {
            // The pinned pieces are between the pinners and the king square
            pinned |= Board::in_between(sq, king_square) & self.color_bb(on_color);
        }

        // Same thing but for bishop rays
        let op_bq = self.piece_bb(Some(!on_color), Bishop) | self.piece_bb(Some(!on_color), Queen);
        let mut pinned: Bitboard = Bitboard(0);
        // xray rook attacks from our king, past our pieces as blockers,
        // to oponent's pieces
        let pinners = self.xray_bishop_attacks(self.color_bb(on_color), king_square) & op_bq;
        // for each pinner
        for sq in pinners {
            // The pinned pieces are between the pinners and the king square
            pinned |= Board::in_between(sq, king_square) & self.color_bb(on_color);
        }

        pinned
    }

    /// Makes the move `m`, updating this board's internal state
    /// This function assumes `m` is a valid move
    pub fn make_move_mut(&mut self, m: &CMove) {
        use Square::*;
        let promo_piece = m.is_promo();
        let from = m.get_from();
        // a one on the from square, else zeroes
        let from_bb = from.as_bitboard();
        // a one on the to square, else zeroes
        let to_bb = m.get_to().as_bitboard();
        // ones on the from and to squares, else zeroes
        let from_to_bb = from_bb ^ to_bb;
        // Assuming this is a valid move and there is a piece on the square
        let CPiece(piece, color) = self.piece_on_square(m.get_from()).unwrap();

        if m.is_king_castle() {
            self.piece_bb[piece as usize] ^= from_to_bb;
            let (rook_from_to_bb, castle_mask) = if let White = color {
                (Bitboard(1 << 7) | Bitboard(1 << 5), 12)
            } else {
                (Bitboard(1 << 63) | Bitboard(1 << 61), 3)
            };
            self.piece_bb[Rook as usize] ^= rook_from_to_bb;
            self.castling_rights &= castle_mask;
            return;
        } else if m.is_queen_castle() {
            self.piece_bb[piece as usize] ^= from_to_bb;
            let (rook_from_to_bb, castle_mask) = if let White = color {
                (Bitboard(1) | Bitboard(1 >> 3), 12)
            } else {
                (Bitboard(56) | Bitboard(1 << 53), 3)
            };
            self.piece_bb[Rook as usize] ^= rook_from_to_bb;
            self.castling_rights &= castle_mask;
            return;
        }

        if m.is_capture() {
            self.fifty_move_rule_counter = 0;
            // If captured piece is different than piece, this is correct,
            // otherwise the to square will be set to 0 instead of 1
            self.piece_bb[piece as usize] ^= from_to_bb;
            // Update from piece's color bit
            self.piece_bb[6 + color as usize] ^= from_to_bb;

            let CPiece(captured_piece, captured_color) = self.piece_on_square(m.get_to()).unwrap();
            // If captured piece is different than piece, we update
            // captured piece bitboard normally, otherwise we flip the bit that was incorrect
            self.piece_bb[captured_piece as usize] ^= to_bb;

            // Promote the piece
            if let Some(promo_piece) = promo_piece {
                self.piece_bb[piece as usize] ^= to_bb;
                self.piece_bb[promo_piece as usize] ^= to_bb;
            }

            // update captured color bitboard
            self.piece_bb[6 + captured_color as usize] ^= to_bb;
            // occupied bitboard has new empty square
            self.occupied_bb ^= from_bb;
            // empty bitboard has new empty square
            self.empty_bb ^= from_bb;
        } else {
            // This is a promotion
            if let Some(promo_piece) = promo_piece {
                // Update prev piece bitboard
                self.piece_bb[piece as usize] ^= from_bb;
                // Updated promo piece bitboard
                self.piece_bb[promo_piece as usize] ^= to_bb;
            } else {
                // update piece bitboard
                self.piece_bb[piece as usize] ^= from_to_bb;
            }

            if let Pawn = piece {
                self.fifty_move_rule_counter = 0;
            } else {
                self.fifty_move_rule_counter += 1;
            }
            // update color bitboard
            self.piece_bb[6 + color as usize] ^= from_to_bb;
            // occupied bitboard has new empty square
            self.occupied_bb ^= from_bb;
            // empty bitboard has new empty square
            self.empty_bb ^= from_bb;
        }

        if m.is_pawn_dpush() {
            self.en_passant_bb = m.get_to().as_bitboard();
        } else {
            self.en_passant_bb = Bitboard(0);
        }
        // update castling rights if king moved
        match (piece, color) {
            (King, White) => self.castling_rights &= 12,
            (King, Black) => self.castling_rights &= 3,
            _ => (),
        };
        // update castling rights if rook moved
        match (piece, from) {
            (Rook, A1) => self.castling_rights &= 13,
            (Rook, H1) => self.castling_rights &= 15,
            (Rook, A8) => self.castling_rights &= 7,
            (Rook, H8) => self.castling_rights &= 11,
            _ => (),
        }
    }

    /// Returns `Some(p)` if there exists a piece `p` on square `s`,
    /// otherwise None
    fn piece_on_square(&self, s: Square) -> Option<CPiece> {
        let bb = s.as_bitboard();

        let c = if (bb & self.color_bb(White)).occupied() {
            White
        } else if (bb & self.color_bb(Black)).occupied() {
            Black
        } else {
            return None;
        };

        for i in 0..6 {
            if (self.piece_bb[i] & bb).occupied() {
                let p = FromPrimitive::from_usize(i).unwrap();
                return Some(CPiece(p, c));
            }
        }

        // Shouldn't get here
        panic!();
    }

    /// Generates a list of moves for color `for_color`
    /// Given the current board state
    pub fn generate_moves(&self, for_color: Color) -> Vec<CMove> {
        if self.fifty_move_rule_counter >= 50 {
            return vec![];
        }

        let king_bb = self.piece_bb(Some(for_color), King);
        let king_square: Square = king_bb.bit_scan().unwrap();
        let attacks_to_king = self.attacks_to(king_square, !for_color);
        let checked = attacks_to_king.occupied();
        let not_pinned = !self.pins(for_color, king_square);

        if checked {
            self.out_of_check_moves(king_square, attacks_to_king, for_color, not_pinned)
        } else {
            (0..6)
                .map(|i| FromPrimitive::from_i32(i).unwrap())
                .flat_map(|piece| self.generate_piece_moves(piece, for_color, not_pinned))
                .chain(self.castle_moves(for_color))
                .collect()
        }
    }

    fn out_of_check_moves(
        &self,
        king_square: Square,
        attacks_to_king: Bitboard,
        for_color: Color,
        not_pinned: Bitboard,
    ) -> Vec<CMove> {
        let king_attacks = Board::king_attacks(king_square);
        // Can't move king to square with our own piece
        let not_to_own_piece = king_attacks & !self.color_bb(for_color);
        // Iterator over all king moves
        let king_moves = not_to_own_piece.filter_map(|to| {
            // Can't move to a square op attacks
            if self.attacks_to(to, !for_color).occupied() {
                None
            } else if self.piece_on_square(to).is_some() {
                Some(CMove::new(king_square, to, cmove::CAPTURE))
            } else {
                Some(CMove::new(king_square, to, cmove::QUIET))
            }
        });

        // only king moves can get out of double check
        if attacks_to_king.count() > 1 {
            king_moves.collect()
        } else {
            // Only one attacker
            let attacker = attacks_to_king.bit_scan().unwrap();
            // The pieces that can capture attacker (can't be pinned)
            let can_capture = self.attacks_to(attacker, for_color) & not_pinned;
            let capture_moves = can_capture.map(|from| CMove::new(from, attacker, cmove::CAPTURE));

            // If the attack was the result of a dpush, we can en passant
            let dpush_king_attack = attacks_to_king & self.en_passant_bb;
            // No en passant capture possable
            if dpush_king_attack.empty() {
                king_moves.chain(capture_moves).collect()
            } else {
                let pawns_not_pinned = self.piece_bb(Some(for_color), Pawn) & not_pinned;
                let ep_moves = Self::ep_moves(for_color, pawns_not_pinned, dpush_king_attack);
                king_moves.chain(capture_moves).chain(ep_moves).collect()
            }
        }
    }

    fn generate_piece_moves(
        &self,
        for_piece: Piece,
        for_color: Color,
        not_pinned: Bitboard,
    ) -> Vec<CMove> {
        if let Pawn = for_piece {
            self.generate_pawn_moves(for_color, not_pinned)
        } else {
            let piece_bb = self.piece_bb(Some(for_color), for_piece);

            piece_bb
                .flat_map(|from| {
                    let can_attack = match for_piece {
                        Knight => Board::knight_attacks(from),
                        Bishop => self.bishop_attacks(from, None),
                        Rook => self.rook_attacks(from, None),
                        Queen => self.queen_attacks(from, None),
                        King => Board::king_attacks(from),
                        Pawn => panic!(), // Can't happen
                    } & !self.color_bb(for_color); // Can't move to square with own piece

                    can_attack.map(move |to| {
                        let to_square_bb = to.as_bitboard();
                        let flag = if (to_square_bb & self.occupied_bb).occupied() {
                            cmove::CAPTURE
                        } else {
                            cmove::QUIET
                        };
                        CMove::new(from, to, flag)
                    })
                })
                .collect()
        }
    }

    fn generate_pawn_moves(&self, for_color: Color, not_pinned: Bitboard) -> Vec<CMove> {
        let op_occupied = self.color_bb(!for_color);
        let pawn_bb = self.piece_bb(Some(for_color), Pawn) & not_pinned;
        let can_push = self.pawns_can_push(for_color);
        let can_dpush = self.pawns_can_dpush(for_color);

        // For every pawn
        let regular_moves = pawn_bb.flat_map(|from| {
            let mut moves = vec![];
            let can_attack = Board::pawn_attacks(from, for_color) & op_occupied;
            let this_pawn_bb = from.as_bitboard();

            // If this pawn can be single pushed
            if (can_push & this_pawn_bb).occupied() {
                let to_dir = match for_color {
                    White => Nort,
                    Black => Sout,
                };
                // We can unwrap since we know this pawn can be pushed
                let to = from.translate(to_dir, 1).unwrap();
                moves.push(CMove::new(from, to, cmove::QUIET));
            }

            // If this pawn can be double pushed
            if (can_dpush & this_pawn_bb).occupied() {
                let to_dir = match for_color {
                    White => Nort,
                    Black => Sout,
                };
                // We can unwrap since we know this pawn can be pushed
                let to = from.translate(to_dir, 2).unwrap();
                moves.push(CMove::new(from, to, cmove::PAWN_DPUSH));
            }

            // For every piece this pawn attacks
            can_attack.for_each(|to| moves.push(CMove::new(from, to, cmove::CAPTURE)));
            moves
        });

        let ep_moves = Self::ep_moves(for_color, pawn_bb, self.en_passant_bb);
        regular_moves.chain(ep_moves).collect()
    }

    fn ep_moves(for_color: Color, with_pawns: Bitboard, pawn_dpushed: Bitboard) -> Vec<CMove> {
        let mut moves = vec![];
        // If our pawn lies to the east of the dpushed pawn, we en passant west
        let ep_capture_west_pawn = Bitboard::east_one(pawn_dpushed) & with_pawns;
        // If our pawn lies to the west of the dpushed pawn, we en passant west
        let ep_capture_east_pawn = Bitboard::west_one(pawn_dpushed) & with_pawns;
        // We can en passant west
        if ep_capture_west_pawn.occupied() {
            let from = ep_capture_west_pawn.bit_scan().unwrap();
            let to = match for_color {
                White => from.translate(Nowe, 1),
                Black => from.translate(Sowe, 1),
            }
            .unwrap();
            moves.push(CMove::new(from, to, cmove::EP_CAPTURE));
        }
        // We can en passant east
        if ep_capture_east_pawn.occupied() {
            let from = ep_capture_west_pawn.bit_scan().unwrap();
            let to = match for_color {
                White => from.translate(Noea, 1),
                Black => from.translate(Soea, 1),
            }
            .unwrap();
            moves.push(CMove::new(from, to, cmove::EP_CAPTURE));
        }
        moves
    }

    fn castle_moves(&self, for_color: Color) -> Vec<CMove> {
        use Square::*;
        let mut moves = vec![];
        match for_color {
            White => {
                // can king-side castle
                if self.castling_rights & WKING_SIDE_MASK > 0 {
                    if (self.attacks_to(F1, Black) | self.attacks_to(G1, Black)).empty()
                        && (self.occupied_bb & Bitboard(1 << 6 | 1 << 5)).empty()
                    {
                        moves.push(CMove::new(E1, G1, cmove::KING_CASTLE));
                    }
                }
                if self.castling_rights & BKING_SIDE_MASK > 0 {
                    if (self.attacks_to(C1, Black) | self.attacks_to(D1, Black)).empty()
                        && (self.occupied_bb & Bitboard(1 << 3 | 1 << 4)).empty()
                    {
                        moves.push(CMove::new(E1, C1, cmove::QUEEN_CASTLE));
                    }
                }
            }
            Black => {
                // can king-side castle
                if self.castling_rights & BKING_SIDE_MASK > 0 {
                    if (self.attacks_to(F8, White) | self.attacks_to(G8, White)).empty()
                        && (self.occupied_bb & Bitboard(1 << 62 | 1 << 61)).empty()
                    {
                        moves.push(CMove::new(E8, G8, cmove::KING_CASTLE));
                    }
                }
                if self.castling_rights & BKING_SIDE_MASK > 0 {
                    if (self.attacks_to(C1, Black) | self.attacks_to(D1, Black)).empty()
                        && (self.occupied_bb & Bitboard(1 << 58 | 1 << 59)).empty()
                    {
                        moves.push(CMove::new(E8, C8, cmove::KING_CASTLE));
                    }
                }
            }
        }
        moves
    }
}
