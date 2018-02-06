#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![feature(custom_attribute)]

mod board;
mod piece;
mod position;
mod moves;
mod utils;
mod move_search;
mod alpha_beta;

use board::DEFAULT_CONFIGURATION;
use board::ChessBoard;

use alpha_beta::{AlphaBeta, CanGenerateMoves, Mode};

use piece::Owner::*;
// use move_search::SearchNode;

// impl<'a> MoveIterator<ChessBoard> for board::move_pieces::MoveIterator<'a> {

// }

// impl<'a> CanGenerateMoves<board::move_pieces::MoveIterator<'a>> for ChessBoard {
//     fn generate_moves(&'a self) -> board::move_pieces::MoveIterator<'a> {
//         // self.generate_moves()
//         ChessBoard::generate_moves(self)
//     }
// }

// impl<'a> MoveIterator for board::move_pieces::MoveIterator<'a> {

// }

// impl <'a> CanGenerateMoves<'a> for ChessBoard {
//     type Item = ChessBoard;
//     type MoveIter = board::move_pieces::MoveIterator<'a>;

//     fn generate_moves(&'a self) -> Self::MoveIter {
//         self.generate_moves()
//     }
// }

impl <'a> CanGenerateMoves<'a> for ChessBoard {
    type ScoreType = f32;
    type Item = ChessBoard;
    type MoveIter = board::move_pieces::MoveIterator<'a>;

    fn generate_moves(&'a self) -> Self::MoveIter {
        self.generate_moves()
    }

    fn score(&self) -> Self::ScoreType {
        0f32
    }
}

// use board::Board;

fn main() {
    // Allowing the panic because if it doesn't build from the default configuration, we're megafucked.
    let board = ChessBoard::from_ascii(DEFAULT_CONFIGURATION, White).unwrap();

    let mut search = AlphaBeta::new(board, Mode::Maximizer);
    // let mut search = AlphaBeta { state: board };

    // let mut search = SearchNode::new(board, White);
    // search.generate_to_depth(4);


    // let children = board.generate_moves(&White);

    // for child in children {
    //     println!("{}", child);
    // }

    // let board = ChessBoard::from_ascii("
    //     RNBQKBNR
    //     PPPPPPPP
    //     xxbbxxxx
    //     xxxqQxxr
    //     xxxxxpxx
    //     xxxnkxxx
    //     xxxnxxxx
    //     xxxxxxxr",
    // ).unwrap();

    // let children = board.generate_moves(&White);

    // for child in children {
    //     println!("{}", child);
    // }
}
