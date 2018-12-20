extern crate lib;

use snapshot::snapshot;

use lib::chess::Player;
use lib::fixtures::*;
use lib::test_moves::generate_moves_for_board;

#[snapshot]
fn test_generate_white_bishop_moves() -> Vec<String> {
    generate_moves_for_board(WHITE_BISHOP_TEST, Player::White, None)
}

#[snapshot]
fn test_generate_black_bishop_moves() -> Vec<String> {
    generate_moves_for_board(BLACK_BISHOP_TEST, Player::Black, None)
}
