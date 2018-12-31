extern crate lib;

use snapshot::snapshot;

use lib::chess::Player;
use lib::fixtures::*;
use lib::test_moves::generate_moves_for_board;

#[snapshot]
fn test_generate_white_king_moves() -> Vec<String> {
    generate_moves_for_board(WHITE_KING_TEST, Player::White, None)
}

#[snapshot]
fn test_generate_black_king_moves() -> Vec<String> {
    generate_moves_for_board(BLACK_KING_TEST, Player::Black, None)
}
