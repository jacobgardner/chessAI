#![cfg(test)]
extern crate lib;

#[macro_use]
extern crate criterion;

use lib::chess::{Board, Player};
use lib::fixtures::*;

use criterion::Criterion;

fn pawn_generation(c: &mut Criterion) {
    c.bench_function("white pawn generation", |b| {
        b.iter(|| {
            let mut boards = vec![];

            let mut board = Board::from(WHITE_PAWN_TEST).unwrap();
            board.prev_move = Some(WHITE_EN_PASSANT);
            boards.push(format!("{}", board).to_owned());

            let mut generator = board.generate_moves(Player::White);

            loop {
                let new_board = match generator.next() {
                    Some(board) => board,
                    None => break,
                };

                boards.push(format!("{}", new_board).to_owned());
            }
            boards
        })
    });

    c.bench_function("black pawn generation", |b| {
        b.iter(|| {
            let mut boards = vec![];

            let mut board = Board::from(BLACK_PAWN_TEST).unwrap();
            board.prev_move = Some(BLACK_EN_PASSANT);
            boards.push(format!("{}", board).to_owned());

            let mut generator = board.generate_moves(Player::Black);

            loop {
                let new_board = match generator.next() {
                    Some(board) => board,
                    None => break,
                };

                boards.push(format!("{}", new_board).to_owned());
            }

            boards
        })
    });
}

criterion_group!(benches, pawn_generation);
criterion_main!(benches);