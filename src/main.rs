#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![feature(custom_attribute)]

mod board;
mod piece;
mod position;
mod moves;
mod utils;

use board::DEFAULT_CONFIGURATION;
use board::ChessBoard;

use piece::Owner::*;

// use board::Board;

fn main() {
    // Allowing the panic because if it doesn't build from the default configuration, we're megafucked.
    let board = ChessBoard::from_ascii(DEFAULT_CONFIGURATION).unwrap();

    let children = board.generate_moves(&White);

    println!("{:?}", children);
}
