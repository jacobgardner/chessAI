mod bitboard;
mod board;

use bitboard::BitBoard;
use board::Board;

const DEFAULT_BOARD: &str = "
rnbkqbnr
pppppppp
xxxxxxxx
xxxxxxxx
xxxxxxxx
xxxxxxxx
PPPPPPPP
RNBKQBNR
";

fn main() {


    let board = Board::from(DEFAULT_BOARD);
    println!("{}", board);

    // println!("{:064b}", 0xf0f8af8fu64);
    // println!("{}", bitboard::BLACK_SQUARES.to_bitboard());
    // println!("{}", bitboard::WHITE_SQUARES.to_bitboard());
    // println!("{}", bitboard::ROW_1.to_bitboard());

    println!("{}", 0b00001111.rotate_45cw().to_rotatedbitboard());
}
