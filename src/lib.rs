//! # An implementation of chess with move generation using bitboards
//! A lot of the information needed to construct this implementation
//! was obtained from the official chess programming wiki: 
//! https://www.chessprogramming.org/Bitboards

mod constants;
mod pawn_moves;
mod shifts;
/// The colors of pieces
#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    fn num(&self) -> usize {
        *self as usize
    }

    fn opp(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

/// All chess piece types
#[derive(Clone, Copy)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    fn num(&self) -> usize {
        *self as usize
    }
}

/// A bitboard is represented as a 64 bit unsigned integer, 1 bit
/// per square for the standard 8x8 chess board. A 1 bit set indicates
/// the presense (or lack thereof) of some property for that square.
/// Properties include a particular piece being present, that square being
/// a potential target, etc... These bitboard are implmented in 
/// Little Endian Rank-File (LERF) order, meaning towards higher
/// valued bits we traverse first across a rank (the numbers), then up files (the letters).
/// 
/// To read more on bitboard representations, you can visit: 
/// https://www.chessprogramming.org/Square_Mapping_Considerations 
type Bitboard = u64;

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
                Board::PAWNS_START,
                Board::KNIGHTS_START,
                Board::BISHOPS_START,
                Board::ROOKS_START,
                Board::QUEENS_START,
                Board::KINGS_START,
                Board::WHITE_START,
                Board::BLACK_START,
            ],
            empty_bb: Board::EMPTY_START,
            occupied_bb: Board::OCCUPIED_START,
        }
    }

    pub fn print_bb(b: Bitboard) {
        for i in (0..8).rev() {
            let rank = (b >> (i * 8)) & 0xff;
            for j in 0..8 {
                let square = (rank >> j) & 1;
                if square == 1 {
                    print!("1 ");
                } else {
                    print!(". ");
                }
            }
            println!("");
        }
        println!("");
    }
}
