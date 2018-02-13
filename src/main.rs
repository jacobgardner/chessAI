#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![feature(custom_attribute)]

mod board;
mod piece;
mod position;
mod moves;
mod utils;
pub mod search;

use search::NodeRole::*;
use board::DEFAULT_CONFIGURATION;
use board::ChessBoard;
use board::move_pieces::WrappedBoard;
// use board::state::BoardState;
use std::rc::Rc;


// use search::alpha_beta::{AlphaBeta, CanGenerateMoves, Mode};
use search::minimax::SearchNode;
use search::Searchable;
use search::Score;

use piece::Owner::*;

type ScoreType = f64;

impl Score for ScoreType {
    fn min_default() -> Self {
        std::f64::MAX
    }

    fn max_default() -> Self {
        std::f64::MIN
    }
}

impl Searchable<WrappedBoard, ScoreType> for WrappedBoard {
    fn score(&self) -> ScoreType  {
        0f64
    }

    fn generate_moves(&self) -> Box<Iterator<Item=Self>> {
        let iterator = self.move_iterator();
        Box::new(iterator)
        // unimplemented!();
    }
}

fn main() {
    // Allowing the panic because if it doesn't build from the default configuration, we're megafucked.
    let board = ChessBoard::from_ascii(DEFAULT_CONFIGURATION, White).unwrap();

    let mut search_node = SearchNode::new(WrappedBoard(Rc::new(board)));
    let (score, best_move) = search_node.search(4, &Maximizer);


    println!("{}", score);
    println!("{}", best_move.unwrap().0);

    // let mut search = AlphaBeta::new(board, Mode::Maximizer);
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
