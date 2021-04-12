use std::{cmp, fmt::Display};

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
    type Heuristic: PartialOrd + Min + Max;

    fn iter(&self) -> Self::It;
    fn heuristic(&self) -> Self::Heuristic;
}

pub struct BestPath<B, S> {
    pub boards: Vec<B>,
    pub score: S,
}

/// This will blow the stack if we keep this recursive for chess
pub fn minimax<H: PartialOrd + Min + Max + Display, T: MinimaxNode<Heuristic = H> + Display>(
    root_node: T,
    max_depth: usize,
    is_maximizing_player: bool,
) -> BestPath<T, H> {
    if max_depth == 0 {
        return BestPath {
            score: root_node.heuristic(),
            boards: vec![root_node],
        };
    }

    if is_maximizing_player {
        let mut value = H::min();
        let mut has_children = false;

        let mut best_boards: Option<Vec<T>> = None;

        for node in root_node.iter() {
            has_children = true;
            let minimax_value = minimax(node, max_depth - 1, false);

            if let Some(cmp::Ordering::Greater) = minimax_value.score.partial_cmp(&value) {
                value = minimax_value.score;
                best_boards = Some(minimax_value.boards);
            }
        }

        if !has_children {
            return BestPath {
                score: root_node.heuristic(),
                boards: vec![root_node],
            };
        }

        let mut boards = best_boards.unwrap();
        boards.push(root_node);

        BestPath {
            score: value,
            boards,
        }
    } else {
        let mut value = H::max();
        let mut has_children = false;
        let mut best_boards: Option<Vec<T>> = None;

        for node in root_node.iter() {
            has_children = true;
            let minimax_value = minimax(node, max_depth - 1, true);

            if let Some(cmp::Ordering::Less) = minimax_value.score.partial_cmp(&value) {
                value = minimax_value.score;
                best_boards = Some(minimax_value.boards);
            }
        }

        if !has_children {
            return BestPath {
                score: root_node.heuristic(),
                boards: vec![root_node],
            };
        }

        let mut boards = best_boards.unwrap();
        boards.push(root_node);

        BestPath {
            score: value,
            boards,
        }
    }
}

pub trait Min {
    fn min() -> Self;
}

pub trait Max {
    fn max() -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tictactoe::{TicTacToe, TicTacToeMoveGenerator};

    impl Min for f64 {
        fn min() -> Self {
            f64::MIN
        }
    }

    impl Max for f64 {
        fn max() -> Self {
            f64::MAX
        }
    }

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
        let board = TicTacToe::with_size(4);

        let path = minimax(board, 15, true);

        println!("MINIMAX Value: {}", path.score);

        for b in path.boards.iter().rev() {
            println!("{}", b);
        }

        println!("The only winning move is not to play.");
    }
}
