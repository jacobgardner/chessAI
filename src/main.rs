extern crate num;
#[macro_use]
extern crate num_derive;

mod bitboard;
mod board;

use bitboard::BitBoard;
use board::Board;

pub const DEFAULT_BOARD: &str = "
rnbkqbnr
pppppppp
xxxxxxxx
xxxxxxxx
xxxxxxxx
xxxxxxxx
PPPPPPPP
RNBKQBNR
";

fn main() -> Result<(), ()> {
    let pieces: [u64; 6] = [1, 1 << 8, 1 << 12, 1 << 16, 1 << 25, 1 << 63];

    let board = Board {
        players: [
            pieces[0] | pieces[2] | pieces[4],
            pieces[1] | pieces[3] | pieces[5],
        ],
        pieces: pieces,
    };
    println!("{}", board);

    let board = Board::from(DEFAULT_BOARD)?;
    println!("{}", board);

    // println!("{:064b}", 0xf0f8af8fu64);
    // println!("{}", bitboard::BLACK_SQUARES.to_bitboard());
    // println!("{}", bitboard::WHITE_SQUARES.to_bitboard());
    // println!("{}", bitboard::ROW_1.to_bitboard());

    println!("{}", 0b00001111.rotate_45cw().to_rotatedbitboard());

    Ok(())
}
