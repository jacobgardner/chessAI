extern crate lib;

use snapshot::snapshot;

use lib::chess::{Player, DEFAULT_BOARD};
use lib::fixtures::*;
use lib::test_moves::generate_moves_for_board;

#[snapshot]
fn test_generate_white_default_moves() -> Vec<String> {
    generate_moves_for_board(DEFAULT_BOARD, Player::White, None)
}

#[snapshot]
fn test_generate_black_default_moves() -> Vec<String> {
    generate_moves_for_board(DEFAULT_BOARD, Player::Black, None)
}

