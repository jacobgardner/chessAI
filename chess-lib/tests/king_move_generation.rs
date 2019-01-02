extern crate chess_lib;

use snapshot::snapshot;
use chess_lib::{chess::Player, fixtures::*, test_moves::generate_moves_for_board};

#[snapshot]
fn test_generate_white_king_moves() -> Vec<String> {
    generate_moves_for_board(WHITE_KING_TEST, Player::White, None)
}

#[snapshot]
fn test_generate_black_king_moves() -> Vec<String> {
    generate_moves_for_board(BLACK_KING_TEST, Player::Black, None)
}
