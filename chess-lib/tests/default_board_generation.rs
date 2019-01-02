extern crate chess_lib;

use chess_lib::{
    chess::{Player, DEFAULT_BOARD},
    test_moves::generate_moves_for_board,
};
use snapshot::snapshot;

#[snapshot]
fn test_generate_white_default_moves() -> Vec<String> {
    generate_moves_for_board(DEFAULT_BOARD, Player::White, None)
}

#[snapshot]
fn test_generate_black_default_moves() -> Vec<String> {
    generate_moves_for_board(DEFAULT_BOARD, Player::Black, None)
}
