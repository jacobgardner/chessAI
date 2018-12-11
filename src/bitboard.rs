#![allow(dead_code)]

use std::num::Wrapping;

pub const ROW_8: BitBoard = BitBoard::new(0xff00_0000_0000_0000);
pub const ROW_7: BitBoard = BitBoard::new(0x00ff_0000_0000_0000);
pub const ROW_6: BitBoard = BitBoard::new(0x0000_ff00_0000_0000);
pub const ROW_5: BitBoard = BitBoard::new(0x0000_00ff_0000_0000);
pub const ROW_4: BitBoard = BitBoard::new(0x0000_0000_ff00_0000);
pub const ROW_3: BitBoard = BitBoard::new(0x0000_0000_00ff_0000);
pub const ROW_2: BitBoard = BitBoard::new(0x0000_0000_0000_ff00);
pub const ROW_1: BitBoard = BitBoard::new(0x0000_0000_0000_00ff);

pub const WHITE_SQUARES: BitBoard =
    BitBoard::new(0b0101_0101_1010_1010_0101_0101_1010_1010_0101_0101_1010_1010_0101_0101_1010_1010);
pub const BLACK_SQUARES: BitBoard =
    BitBoard::new(0b1010_1010_0101_0101_1010_1010_0101_0101_1010_1010_0101_0101_1010_1010_0101_0101);

const HORIZONTAL_K1: BitBoard = BitBoard::new(0x5555_5555_5555_5555);
const HORIZONTAL_K2: BitBoard = BitBoard::new(0x3333_3333_3333_3333);
const HORIZONTAL_K4: BitBoard = BitBoard::new(0x0f0f_0f0f_0f0f_0f0f);

const DIAGONAL_K1: BitBoard = BitBoard::new(0x5500_5500_5500_5500);
const DIAGONAL_K2: BitBoard = BitBoard::new(0x3333_0000_3333_0000);
const DIAGONAL_K4: BitBoard = BitBoard::new(0x0f0f_0f0f_0000_0000);

const ANTIDIAGONAL_K1: BitBoard = BitBoard::new(0xaa00_aa00_aa00_aa00);
const ANTIDIAGONAL_K2: BitBoard = BitBoard::new(0xcccc_0000_cccc_0000);
const ANTIDIAGONAL_K4: BitBoard = BitBoard::new(0xf0f0_f0f0_0f0f_0f0f);

const ROTATE_45CW_K1: BitBoard = BitBoard::new(0xaaaa_aaaa_aaaa_aaaa);
const ROTATE_45CW_K2: BitBoard = BitBoard::new(0xcccc_cccc_cccc_cccc);
const ROTATE_45CW_K4: BitBoard = BitBoard::new(0xf0f0_f0f0_f0f0_f0f0);

const ROTATE_45CCW_K1: BitBoard = BitBoard::new(0x5555_5555_5555_5555);
const ROTATE_45CCW_K2: BitBoard = BitBoard::new(0x3333_3333_3333_3333);
const ROTATE_45CCW_K4: BitBoard = BitBoard::new(0x0f0f_0f0f_00f0_f0f0);

#[cfg_attr(rustfmt, rustfmt_skip)]
pub enum RankFile {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

#[derive(PartialEq, Debug)]
pub struct BitBoard {
    board: u64,
}

// pub trait BitBoard {
//     fn flip_vertical(self) -> Self;
//     fn flip_horizontal(self) -> Self;
//     fn flip_diagonal(self) -> Self;
//     fn flip_antidiagonal(self) -> Self;

//     fn rotate_180(self) -> Self;
//     fn rotate_90cw(self) -> Self;
//     fn rotate_90ccw(self) -> Self;

//     fn rotate_45cw(self) -> Self;
//     fn rotate_45ccw(self) -> Self;

//     fn to_bitboard(self) -> String;
//     fn to_rotatedbitboard(self) -> String;
// }

impl BitBoard {
    #[inline(always)]
    pub fn new(board: u64) -> Self {
        BitBoard { board }
    }

    // TODO: We probably want to inline all of these

    fn flip_vertical(self) -> Self {
        BitBoard::new(self.board.swap_bytes())
    }

    fn flip_horizontal(self) -> Self {
        let mut board = self.board;
        board = (Wrapping((board >> 1) & HORIZONTAL_K1.board)
            + Wrapping(2) * Wrapping(board & HORIZONTAL_K1.board))
        .0;
        board = (Wrapping((board >> 2) & HORIZONTAL_K2.board)
            + Wrapping(4) * Wrapping(board & HORIZONTAL_K2.board))
        .0;
        board = (Wrapping((board >> 4) & HORIZONTAL_K4.board)
            + Wrapping(16) * Wrapping(board & HORIZONTAL_K4.board))
        .0;

        BitBoard::new(board)

        // self ^= k4 & (self ^ self.rotate_left(8));
        // self ^= k2 & (self ^ self.rotate_left(4));
        // self ^= k1 & (self ^ self.rotate_left(2));

        // self.rotate_right(7)
    }

    fn flip_diagonal(self) -> Self {
        let mut board = self.board;
        let mut temp = DIAGONAL_K4.board & (board ^ (board << 28));
        board ^= temp ^ (temp >> 28);
        temp = DIAGONAL_K2.board & (board ^ (board << 14));
        board ^= temp ^ (temp >> 14);
        temp = DIAGONAL_K1.board & (board ^ (board << 7));
        board ^= temp ^ (temp >> 7);

        BitBoard::new(board)
    }

    fn flip_antidiagonal(self) -> Self {
        let mut board = self.board;
        let mut temp = board ^ (board << 36);
        board ^= board ^ ANTIDIAGONAL_K4.board & (temp ^ (board >> 36));
        temp = ANTIDIAGONAL_K2.board & (board ^ (board << 18));
        board ^= temp ^ (temp >> 18);
        temp = ANTIDIAGONAL_K1.board & (board ^ (board << 9));
        board ^= temp ^ (temp >> 9);

        BitBoard::new(board)
    }

    fn rotate_180(self) -> Self {
        self.flip_vertical().flip_horizontal()
    }

    fn rotate_90cw(self) -> Self {
        self.flip_diagonal().flip_vertical()
    }

    fn rotate_90ccw(self) -> Self {
        self.flip_vertical().flip_diagonal()
    }

    fn rotate_45cw(mut self) -> Self {
        let mut board = self.board;

        board ^= ROTATE_45CW_K1.board & (board ^ board.rotate_right(8));
        board ^= ROTATE_45CW_K2.board & (board ^ board.rotate_right(16));
        board ^= ROTATE_45CW_K4.board & (board ^ board.rotate_right(32));

        BitBoard::new(board)
    }

    fn rotate_45ccw(mut self) -> Self {
        let mut board = self.board;
        board ^= ROTATE_45CCW_K1.board & (board ^ board.rotate_right(8));
        board ^= ROTATE_45CCW_K2.board & (board ^ board.rotate_right(16));
        board ^= ROTATE_45CCW_K4.board & (board ^ board.rotate_right(32));

        // self
        BitBoard::new(board)
    }

    fn to_bitboard(mut self) -> String {
        let mut bits = String::with_capacity(64 + 8);

        let mut board = self.board;

        for _ in 0..8 {
            bits += &to_bitstring(board, 8);
            bits += "\n";
            board <<= 8;
        }

        bits
    }

    fn to_rotatedbitboard(self) -> String {
        let mut bits = String::new();

        // 1, 2, 3, 4, 5, 6, 7, 8, 7, 6, 5, 4, 3, 2, 1

        let ranges: Vec<(u64, u64)> = vec![(1, 0), (3, 1), (6, 3)];
        let mut board = self.board;

        for range in ranges {
            bits += &to_bitstring(bitrange(range.0, range.1) & board, range.0 - range.1);
            bits += "\n";
        }

        // println!("{:064b}", bitrange(RankFile::B1 as u64, 0));
        // println!("{:064b}", bitrange(RankFile::D1 as u64, RankFile::B1 as u64));

        // bits += &to_bitstring(1 << RankFile::A1 as u64 & self, 1);
        // bits += "\n";
        // bits += &to_bitstring(1 << RankFile::C1 as u64 & self, 2);
        // bits += "\n";
        // // String::from("")

        bits
    }
}

fn bitrange(start: u64, end: u64) -> u64 {
    ((1 << start as u64) - 1) ^ ((1 << end as u64) - 1)
}

fn to_bitstring(bits: u64, padding: u64) -> String {
    // format!("{:0padding$b}\n", (bits & ROW_8) >> (64 - 8), padding = padding).chars().rev().collect::<String>()
    format!(
        "{:0padding$b}",
        (bits & ROW_8.board) >> (64 - 8),
        padding = padding as usize
    )
    .chars()
    .rev()
    .collect::<String>()
}

// TODO: More, better tests
#[test]
pub fn test_flip_vertical() {
    assert_eq!(ROW_1.flip_vertical(), ROW_8);
    assert_eq!(ROW_8.flip_vertical(), ROW_1);

    assert_eq!(WHITE_SQUARES.flip_vertical(), BLACK_SQUARES);
    assert_eq!(BLACK_SQUARES.flip_vertical(), WHITE_SQUARES);
}

#[test]
pub fn test_flip_horizontal() {
    assert_eq!(WHITE_SQUARES.flip_horizontal(), BLACK_SQUARES);
    assert_eq!(BLACK_SQUARES.flip_horizontal(), WHITE_SQUARES);
}

#[test]
pub fn test_flip_diagonal() {
    assert_eq!(WHITE_SQUARES.flip_diagonal(), WHITE_SQUARES);
    assert_eq!(BLACK_SQUARES.flip_diagonal(), BLACK_SQUARES);
}

#[test]
pub fn test_rotate45cw() {
    // assert_eq!(1u64.rotate_45cw(), 0b0);
}
