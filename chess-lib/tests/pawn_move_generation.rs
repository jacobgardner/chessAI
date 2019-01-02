extern crate chess_lib;

use snapshot::snapshot;
use chess_lib::{chess::Player, fixtures::*, test_moves::generate_moves_for_board};

#[snapshot]
fn test_generate_white_pawn_moves() -> Vec<String> {
    generate_moves_for_board(WHITE_PAWN_TEST, Player::White, Some(WHITE_EN_PASSANT))
}

#[snapshot]
fn test_generate_black_pawn_moves() -> Vec<String> {
    generate_moves_for_board(BLACK_PAWN_TEST, Player::Black, Some(BLACK_EN_PASSANT))
}
