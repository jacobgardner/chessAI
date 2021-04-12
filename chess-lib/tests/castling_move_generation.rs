extern crate chess_lib;

use snapshot::snapshot;

use chess_lib::{chess::Player, fixtures::*, test_moves::generate_moves_for_board};

#[snapshot]
fn test_generate_castling_unobstructed_w() -> Vec<String> {
    generate_moves_for_board(CASTLING_TEST_UNOBSTRUCTED, Player::White, None)
}

#[snapshot]
fn test_generate_castling_unobstructed_b() -> Vec<String> {
    generate_moves_for_board(CASTLING_TEST_UNOBSTRUCTED, Player::Black, None)
}

#[snapshot]
fn test_generate_castling_obstructed_w() -> Vec<String> {
    generate_moves_for_board(CASTLING_TEST_OBSTRUCTED, Player::White, None)
}

#[snapshot]
fn test_generate_castling_obstructed_b() -> Vec<String> {
    generate_moves_for_board(CASTLING_TEST_OBSTRUCTED, Player::Black, None)
}

#[snapshot]
fn test_generate_castling_from_check_w() -> Vec<String> {
    generate_moves_for_board(CASTLING_TEST_FROM_CHECK, Player::White, None)
}

#[snapshot]
fn test_generate_castling_from_check_b() -> Vec<String> {
    generate_moves_for_board(CASTLING_TEST_FROM_CHECK, Player::Black, None)
}

#[snapshot]
fn test_generate_castling_through_check_w() -> Vec<String> {
    generate_moves_for_board(CASTLING_TEST_THROUGH_CHECK, Player::White, None)
}

#[snapshot]
fn test_generate_castling_through_check_b() -> Vec<String> {
    generate_moves_for_board(CASTLING_TEST_THROUGH_CHECK, Player::Black, None)
}
