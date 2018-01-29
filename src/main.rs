#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![feature(custom_attribute)]

mod board;
mod piece;
mod position;
mod moves;
mod utils;
mod move_search;

use board::DEFAULT_CONFIGURATION;
use board::ChessBoard;

use piece::Owner::*;
use move_search::SearchNode;

// use board::Board;

fn main() {
    // Allowing the panic because if it doesn't build from the default configuration, we're megafucked.
    let board = ChessBoard::from_ascii(DEFAULT_CONFIGURATION).unwrap();

    

    // let mut search = SearchNode::new(board, White);
    // search.generate_to_depth(5);


    let children = board.generate_moves(&White);

    for child in children {
        println!("{}", child);
    }

    let board = ChessBoard::from_ascii("
        RNBQKBNR
        PPPPPPPP
        xxbbxxxx
        xxxqQxxr
        xxxxxpxx
        xxxnkxxx
        xxxnxxxx
        xxxxxxxr",
    ).unwrap();

    let children = board.generate_moves(&White);

    for child in children {
        println!("{}", child);
    }
}
