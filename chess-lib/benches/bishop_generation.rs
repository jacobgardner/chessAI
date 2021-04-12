#![cfg(test)]
extern crate lib;

#[macro_use]
extern crate criterion;

use lib::chess::Player;
use lib::fixtures::*;
use lib::test_moves::generate_moves_for_board;

use criterion::Criterion;

fn bishop_generation(c: &mut Criterion) {
    c.bench_function("white bishop generation", |b| {
        b.iter(|| generate_moves_for_board(WHITE_BISHOP_TEST, Player::White, None))
    });

    c.bench_function("black bishop generation", |b| {
        b.iter(|| generate_moves_for_board(BLACK_BISHOP_TEST, Player::Black, None))
    });
}

criterion_group!(benches, bishop_generation);
criterion_main!(benches);
