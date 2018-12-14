#![cfg(test)]
extern crate lib;

use lib::chess::{Board, Player};
use lib::fixtures::*;
use snapshot::snapshot;

#[snapshot]
fn test_generate_white_pawn_moves() -> Vec<String> {
    let mut boards = vec![];

    let mut board = Board::from(WHITE_PAWN_TEST).unwrap();
    board.prev_move = Some(WHITE_EN_PASSANT);
    boards.push(format!("{}", board).to_owned());

    let mut generator = board.generate_moves(Player::White);

    loop {
        let new_board = match generator.next() {
            Some(board) => board,
            None => break,
        };

        boards.push(format!("{}", new_board).to_owned());
    }
    boards
}

#[snapshot]
fn test_generate_black_pawn_moves() -> Vec<String> {
    let mut boards = vec![];

    let mut board = Board::from(BLACK_PAWN_TEST).unwrap();
    board.prev_move = Some(BLACK_EN_PASSANT);
    boards.push(format!("{}", board).to_owned());

    let mut generator = board.generate_moves(Player::Black);

    loop {
        let new_board = match generator.next() {
            Some(board) => board,
            None => break,
        };

        boards.push(format!("{}", new_board).to_owned());
    }

    boards
}
