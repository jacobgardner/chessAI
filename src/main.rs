#![allow(dead_code)]
// #![cfg_attr(feature = "strict", allow(dead_code))]
// #![cfg_attr(feature = "strict", deny(missing_docs))]
#![cfg_attr(feature = "strict", deny(warnings))]

#[macro_use] extern crate uncover;
#[macro_use] extern crate num_derive;
#[macro_use] extern crate failure;

define_uncover_macros!(
    enable_if(cfg!(debug_assertions))
);

mod bitboard;
mod bitposition;
mod board;
mod rank_file;

use crate::bitboard::BitBoard;
use crate::board::Board;

fn main() -> Result<(), failure::Error> {
    let pieces: [BitBoard; 6] = [
        BitBoard::from(1),
        BitBoard::from(1 << 8),
        BitBoard::from(1 << 12),
        BitBoard::from(1 << 16),
        BitBoard::from(1 << 25),
        BitBoard::from(1 << 63),
    ];

    let board = Board {
        players: [
            pieces[0].join(pieces[2]).join(pieces[4]),
            pieces[1].join(pieces[3]).join(pieces[5]),
        ],
        pieces,
    };
    println!("{}", board);

    let board = Board::from(board::DEFAULT_BOARD)?;
    println!("{}", board);

    // println!("{:064b}", 0xf0f8af8fu64);
    // println!("{}", bitboard::BLACK_SQUARES.to_bitboard());
    // println!("{}", bitboard::WHITE_SQUARES.to_bitboard());
    // println!("{}", bitboard::FILE_1.to_bitboard());

    println!(
        "{}",
        BitBoard::from(0b0000_1111)
            .rotate_45cw()
            .to_bitboard()
    );

    Ok(())
}
