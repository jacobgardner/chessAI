#![cfg(test)]
extern crate lib;

#[macro_use]
extern crate criterion;

use lib::chess::{Board, Player};
use lib::fixtures::*;
use lib::test_moves::generate_moves_for_board;

use criterion::Criterion;

fn pawn_generation(c: &mut Criterion) {
    c.bench_function("white pawn generation", |b| {
        b.iter(|| generate_moves_for_board(WHITE_PAWN_TEST, Player::White, Some(WHITE_EN_PASSANT)))
    });

    c.bench_function("black pawn generation", |b| {
        b.iter(|| generate_moves_for_board(BLACK_PAWN_TEST, Player::Black, Some(BLACK_EN_PASSANT)))
    });
}

criterion_group!(benches, pawn_generation);
criterion_main!(benches);
