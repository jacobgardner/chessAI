#![cfg(test)]
extern crate lib;

#[macro_use]
extern crate criterion;

use lib::chess::{Board, Player};
use lib::fixtures::*;
use lib::test_moves::generate_moves_for_board;

use criterion::Criterion;

fn rook_generation(c: &mut Criterion) {
    c.bench_function("white rook generation", |b| {
        b.iter(|| generate_moves_for_board(WHITE_ROOK_TEST, Player::White, None))
    });

    c.bench_function("black rook generation", |b| {
        b.iter(|| generate_moves_for_board(BLACK_ROOK_TEST, Player::Black, None))
    });
}

criterion_group!(benches, rook_generation);
criterion_main!(benches);

