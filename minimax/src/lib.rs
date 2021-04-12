use std::fmt::Display;

mod tictactoe;

// Ultimate goal here:
//  - I have a function that can take in a generic object of some sort Generic or Trait
//  - That object has an iterator that can make more of itself
//  - I can safely pass a value from the iterator back into the root function.

// This seems to indicate to me two approaches:
//  (A) Trait that create an iterator that iterates over itself
//  (B) Generic Struct where the generic implements a trait that does the same...

pub trait MinimaxNode {
    type It: Iterator<Item = Self>;
    type Heuristic: PartialOrd;

    fn iter(&self) -> Self::It;
    fn heuristic(&self) -> Self::Heuristic;
}

pub fn minimax<H: Display, T: MinimaxNode<Heuristic = H> + Display>(
    root_node: T,
    max_depth: usize,
    is_maximizing_player: bool,
) {
    if max_depth == 0 {
        return;
    }

    for node in root_node.iter() {
        minimax(node, max_depth - 1, !is_maximizing_player);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tictactoe::{TicTacToe, TicTacToeMoveGenerator};

    impl MinimaxNode for TicTacToe {
        type It = TicTacToeMoveGenerator;
        type Heuristic = f64;

        fn iter(&self) -> Self::It {
            self.generate_moves()
        }

        fn heuristic(&self) -> Self::Heuristic {
            self.score_board() 
        }
    }

    #[test]
    fn tic_tac_toe() {
        let board = TicTacToe::new();

        minimax(board, 8, true);


        println!("The only winning move is not to play.");
    }
}
