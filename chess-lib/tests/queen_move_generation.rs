extern crate lib;

use snapshot::snapshot;

use lib::chess::Player;
use lib::fixtures::*;
use lib::test_moves::generate_moves_for_board;

#[snapshot]
fn test_generate_white_queen_moves() -> Vec<String> {
    generate_moves_for_board(WHITE_QUEEN_TEST, Player::White, Some(WHITE_EN_PASSANT))
}

#[snapshot]
fn test_generate_black_queen_moves() -> Vec<String> {
    generate_moves_for_board(BLACK_QUEEN_TEST, Player::Black, Some(BLACK_EN_PASSANT))
}
