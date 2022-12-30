mod bitboard;
mod piece_moves;
// mod piece_moves;
use bitboard::Bitboard;

/// The colors of pieces
#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opp(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

/// All chess piece types
#[derive(Clone, Copy)]
pub enum Piece {
    Pawn, Knight, Bishop, Rook, Queen, King,
}

/// All eight cardinal directions
#[derive(Clone, Copy)]
pub enum Dir {
    Nort, Noea, East, Soea, Sout, Sowe, West, Nowe,
}

impl Dir {
    pub fn neg(&self) -> bool {
        match *self {
            Dir::West | Dir::Sout | Dir::Sowe | Dir::Soea => true,
            _ => false,
        }
    }

    pub fn pos(&self) -> bool {
        match *self {
            Dir::West | Dir::Sout | Dir::Sowe | Dir::Soea => false,
            _ => true,
        }
    }
}

#[derive(Clone, Copy)]
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
    /// piece `p` of color `c`.
    pub fn piece_bb(&self, c: Color, p: Piece) -> Bitboard {
        Bitboard(self.piece_bb[p as usize].0 & self.piece_bb[6 + c as usize].0)
    }
}