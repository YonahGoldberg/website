//! An implementation of chess with move generation using bitboards
//! A lot of the information needed to construct this implementation
//! was obtained from the official chess programming wiki: 
//! <https://www.chessprogramming.org/Bitboards>

mod board;
mod bitboard;
mod tables;
mod cmove;
mod engine;
mod utils;

extern crate num;
#[macro_use]
extern crate num_derive;