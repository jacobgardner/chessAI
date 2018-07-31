#![allow(dead_code)]

use std::num::Wrapping;

pub const ROW_8: u64 = 0xff00000000000000;
pub const ROW_7: u64 = 0x00ff000000000000;
pub const ROW_6: u64 = 0x0000ff0000000000;
pub const ROW_5: u64 = 0x000000ff00000000;
pub const ROW_4: u64 = 0x00000000ff000000;
pub const ROW_3: u64 = 0x0000000000ff0000;
pub const ROW_2: u64 = 0x000000000000ff00;
pub const ROW_1: u64 = 0x00000000000000ff;

pub const WHITE_SQUARES: u64 = 0b0101010110101010010101011010101001010101101010100101010110101010;
pub const BLACK_SQUARES: u64 = 0b1010101001010101101010100101010110101010010101011010101001010101;

const HORIZONTAL_K1: u64 = 0x5555555555555555;
const HORIZONTAL_K2: u64 = 0x3333333333333333;
const HORIZONTAL_K4: u64 = 0x0f0f0f0f0f0f0f0f;

const DIAGONAL_K1: u64 = 0x5500550055005500;
const DIAGONAL_K2: u64 = 0x3333000033330000;
const DIAGONAL_K4: u64 = 0x0f0f0f0f00000000;

const ANTIDIAGONAL_K1: u64 = 0xaa00aa00aa00aa00;
const ANTIDIAGONAL_K2: u64 = 0xcccc0000cccc0000;
const ANTIDIAGONAL_K4: u64 = 0xf0f0f0f00f0f0f0f;

const ROTATE_45CW_K1: u64 = 0xaaaaaaaaaaaaaaaa;
const ROTATE_45CW_K2: u64 = 0xcccccccccccccccc;
const ROTATE_45CW_K4: u64 = 0xf0f0f0f0f0f0f0f0;

const ROTATE_45CCW_K1: u64 = 0x5555555555555555;
const ROTATE_45CCW_K2: u64 = 0x3333333333333333;
const ROTATE_45CCW_K4: u64 = 0x0f0f0f0f00f0f0f0;

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

pub trait BitBoard {
    fn flip_vertical(self) -> Self;
    fn flip_horizontal(self) -> Self;
    fn flip_diagonal(self) -> Self;
    fn flip_antidiagonal(self) -> Self;

    fn rotate_180(self) -> Self;
    fn rotate_90cw(self) -> Self;
    fn rotate_90ccw(self) -> Self;

    fn rotate_45cw(self) -> Self;
    fn rotate_45ccw(self) -> Self;

    fn to_bitboard(self) -> String;
    fn to_rotatedbitboard(self) -> String;
}

impl BitBoard for u64 {
    fn flip_vertical(self) -> Self {
        self.swap_bytes()
    }

    fn flip_horizontal(mut self) -> Self {
        self = (Wrapping((self >> 1) & HORIZONTAL_K1)
            + Wrapping(2) * Wrapping(self & HORIZONTAL_K1))
            .0;
        self = (Wrapping((self >> 2) & HORIZONTAL_K2)
            + Wrapping(4) * Wrapping(self & HORIZONTAL_K2))
            .0;
        self = (Wrapping((self >> 4) & HORIZONTAL_K4)
            + Wrapping(16) * Wrapping(self & HORIZONTAL_K4))
            .0;
        self

        // self ^= k4 & (self ^ self.rotate_left(8));
        // self ^= k2 & (self ^ self.rotate_left(4));
        // self ^= k1 & (self ^ self.rotate_left(2));

        // self.rotate_right(7)
    }

    fn flip_diagonal(mut self) -> Self {
        let mut temp = DIAGONAL_K4 & (self ^ (self << 28));
        self ^= temp ^ (temp >> 28);
        temp = DIAGONAL_K2 & (self ^ (self << 14));
        self ^= temp ^ (temp >> 14);
        temp = DIAGONAL_K1 & (self ^ (self << 7));
        self ^= temp ^ (temp >> 7);

        self
    }

    fn flip_antidiagonal(mut self) -> Self {
        let mut temp = self ^ (self << 36);
        self ^= ANTIDIAGONAL_K4 & (temp ^ (self >> 36));
        temp = ANTIDIAGONAL_K2 & (self ^ (self << 18));
        self ^= temp ^ (temp >> 18);
        temp = ANTIDIAGONAL_K1 & (self ^ (self << 9));
        self ^= temp ^ (temp >> 9);

        self
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
        self ^= ROTATE_45CW_K1 & (self ^ self.rotate_right(8));
        self ^= ROTATE_45CW_K2 & (self ^ self.rotate_right(16));
        self ^= ROTATE_45CW_K4 & (self ^ self.rotate_right(32));

        self
    }

    fn rotate_45ccw(mut self) -> Self {
        self ^= ROTATE_45CCW_K1 & (self ^ self.rotate_right(8));
        self ^= ROTATE_45CCW_K2 & (self ^ self.rotate_right(16));
        self ^= ROTATE_45CCW_K4 & (self ^ self.rotate_right(32));

        self
    }

    fn to_bitboard(mut self) -> String {
        let mut bits = String::with_capacity(64 + 8);

        for _ in 0..8 {
            bits += &to_bitstring(self, 8);
            // bits += &format!("{:08b}", (self & ROW_8) >> (64 - 8)).chars().rev().collect::<String>();
            bits += "\n";
            self <<= 8;
        }

        // bits += ;

        bits
    }

    fn to_rotatedbitboard(self) -> String {
        let mut bits = String::new();

        // 1, 2, 3, 4, 5, 6, 7, 8, 7, 6, 5, 4, 3, 2, 1

        let ranges: Vec<(u64, u64)> = vec![(1, 0), (3, 1), (6, 3)];

        for range in ranges {
            bits += &to_bitstring(bitrange(range.0, range.1) & self, range.0 - range.1);
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
        (bits & ROW_8) >> (64 - 8),
        padding = padding as usize
    ).chars()
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
