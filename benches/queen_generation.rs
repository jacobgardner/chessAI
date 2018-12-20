#![cfg(test)]
extern crate lib;

#[macro_use]
extern crate criterion;

use lib::chess::Player;
use lib::fixtures::*;
use lib::test_moves::generate_moves_for_board;

use criterion::Criterion;

fn queen_generation(c: &mut Criterion) {
    c.bench_function("white queen generation", |b| {
        b.iter(|| generate_moves_for_board(WHITE_QUEEN_TEST, Player::White, None))
    });

    c.bench_function("black queen generation", |b| {
        b.iter(|| generate_moves_for_board(BLACK_QUEEN_TEST, Player::Black, None))
    });
}

criterion_group!(benches, queen_generation);
criterion_main!(benches);
