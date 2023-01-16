//! An implementation of chess with move generation using bitboards
//! A lot of the information needed to construct this implementation
//! was obtained from the official chess programming wiki: 
//! <https://www.chessprogramming.org/Bitboards>

#[macro_use]
extern crate num_derive;
extern crate num;
pub mod board;
pub mod engine;
