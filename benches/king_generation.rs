#![cfg(test)]
extern crate lib;

#[macro_use]
extern crate criterion;

use lib::chess::Player;
use lib::fixtures::*;
use lib::test_moves::generate_moves_for_board;

use criterion::Criterion;

fn king_generation(c: &mut Criterion) {
    c.bench_function("white king generation", |b| {
        b.iter(|| generate_moves_for_board(WHITE_KING_TEST, Player::White, None))
    });

    c.bench_function("black king generation", |b| {
        b.iter(|| generate_moves_for_board(BLACK_KING_TEST, Player::Black, None))
    });
}

criterion_group!(benches, king_generation);
criterion_main!(benches);
