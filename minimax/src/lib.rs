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
    type Heuristic: PartialOrd;

    fn iter(&self) -> Self::It;
    fn heuristic(&self) -> Self::Heuristic;
}

pub struct BestPath<B, S> {
    pub boards: Vec<B>,
    pub score: S,
}

pub fn minimax_path<H: PartialOrd, T: MinimaxNode<Heuristic = H>>(
    root_node: T,
    is_maximizing_player: bool,
    max_depth: usize,
) -> BestPath<T, H> {
    if max_depth == 0 {
        return BestPath {
            score: root_node.heuristic(),
            boards: vec![root_node],
        };
    }

    let player_operation = if is_maximizing_player {
        cmp::Ordering::Greater
    } else {
        cmp::Ordering::Less
    };

    let path = root_node
        .iter()
        .fold(None, |acc: Option<BestPath<T, H>>, node| {
            let path: BestPath<T, H> = minimax_path(node, !is_maximizing_player, max_depth - 1);

            if let Some(value) = acc {
                match path.score.partial_cmp(&value.score) {
                    Some(op) if op == player_operation => Some(path),
                    _ => Some(value),
                }
            } else {
                Some(path)
            }
        });

    if let Some(mut path) = path {
        // let mut boards = best_boards.unwrap();
        // boards.push(root_node);

        path.boards.push(root_node);

        path
    } else {
        BestPath {
            score: root_node.heuristic(),
            boards: vec![root_node],
        }
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
        let board = TicTacToe::with_size(3);

        let path = minimax_path(board, true, 16);

        println!("MINIMAX Value: {}", path.score);

        for b in path.boards.iter().rev() {
            println!("{}", b);
        }

        println!("The only winning move is not to play.");
    }
}
