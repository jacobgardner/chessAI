extern crate chess_lib;

use chess_lib::{chess::Player, fixtures::*, test_moves::generate_moves_for_board};
use snapshot::snapshot;

#[snapshot]
fn test_generate_white_knight_moves() -> Vec<String> {
    generate_moves_for_board(WHITE_KNIGHT_TEST, Player::White, None)
}

#[snapshot]
fn test_generate_black_knight_moves() -> Vec<String> {
    generate_moves_for_board(BLACK_KNIGHT_TEST, Player::Black, None)
}
