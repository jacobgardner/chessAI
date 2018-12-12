use crate::bitposition::BitPosition;
use crate::rank_file::RankFile;

use std::num::Wrapping;

pub const ROW_8: BitBoard = BitBoard::new(0xff00_0000_0000_0000);
pub const ROW_7: BitBoard = BitBoard::new(0x00ff_0000_0000_0000);
pub const ROW_6: BitBoard = BitBoard::new(0x0000_ff00_0000_0000);
pub const ROW_5: BitBoard = BitBoard::new(0x0000_00ff_0000_0000);
pub const ROW_4: BitBoard = BitBoard::new(0x0000_0000_ff00_0000);
pub const ROW_3: BitBoard = BitBoard::new(0x0000_0000_00ff_0000);
pub const ROW_2: BitBoard = BitBoard::new(0x0000_0000_0000_ff00);
pub const ROW_1: BitBoard = BitBoard::new(0x0000_0000_0000_00ff);

pub const WHITE_SQUARES: BitBoard = BitBoard::new(
    0b0101_0101_1010_1010_0101_0101_1010_1010_0101_0101_1010_1010_0101_0101_1010_1010,
);
pub const BLACK_SQUARES: BitBoard = BitBoard::new(
    0b1010_1010_0101_0101_1010_1010_0101_0101_1010_1010_0101_0101_1010_1010_0101_0101,
);

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

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Rotated45BitBoard {
    board: u64,
}

impl From<u64> for Rotated45BitBoard {
    #[inline(always)]
    fn from(bits: u64) -> Self {
        Rotated45BitBoard { board: bits }
    }
}

impl Rotated45BitBoard {
    pub fn to_bitboard(self) -> String {
        let mut bits = String::new();

        // 1, 2, 3, 4, 5, 6, 7, 8, 7, 6, 5, 4, 3, 2, 1

        let ranges: Vec<(u64, u64)> = vec![(1, 0), (3, 1), (6, 3)];
        let board = self.board;

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

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct BitBoard {
    board: u64,
}

impl From<u64> for BitBoard {
    #[inline(always)]
    fn from(bits: u64) -> Self {
        BitBoard { board: bits }
    }
}

impl From<BitPosition> for BitBoard {
    #[inline(always)]
    fn from(position: BitPosition) -> Self {
        BitBoard {
            board: 1 << position.right_index,
        }
    }
}

impl From<RankFile> for BitBoard {
    #[inline(always)]
    fn from(rank_file: RankFile) -> Self {
        BitBoard::from(BitPosition::from(rank_file))
    }
}

// TODO: Check if inlining(always) is actually better than what the compiler does by default
impl BitBoard {
    #[inline(always)]
    pub const fn new(board: u64) -> Self {
        BitBoard { board }
    }

    #[inline(always)]
    pub fn empty() -> Self {
        covered_by!("BitBoard::empty");
        BitBoard { board: 0 }
    }

    #[inline(always)]
    pub fn join(self, rhs: BitBoard) -> Self {
        covered_by!("BitBoard::join");
        BitBoard::from(self.board | rhs.board)
    }

    #[inline(always)]
    pub fn intersect(self, rhs: BitBoard) -> Self {
        covered_by!("BitBoard::intersect");
        BitBoard::from(self.board & rhs.board)
    }

    #[inline(always)]
    pub fn inverse(self) -> Self {
        covered_by!("BitBoard::inverse");
        BitBoard::from(!self.board)
    }

    #[inline(always)]
    pub fn is_empty(self) -> bool {
        self.board == 0
    }

    #[inline(always)]
    pub fn first_bit_position(self) -> BitPosition {
        BitPosition::from(self.board.trailing_zeros())
    }

    pub fn flip_vertical(self) -> Self {
        covered_by!("BitBoard::flip_vertical");
        BitBoard::from(self.board.swap_bytes())
    }

    pub fn flip_horizontal(self) -> Self {
        covered_by!("BitBoard::flip_horizontal");
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

        BitBoard::from(board)

        // self ^= k4 & (self ^ self.rotate_left(8));
        // self ^= k2 & (self ^ self.rotate_left(4));
        // self ^= k1 & (self ^ self.rotate_left(2));

        // self.rotate_right(7)
    }

    pub fn flip_diagonal(self) -> Self {
        covered_by!("BitBoard::flip_diagonal");

        let mut board = self.board;
        let mut temp = DIAGONAL_K4.board & (board ^ (board << 28));
        board ^= temp ^ (temp >> 28);
        temp = DIAGONAL_K2.board & (board ^ (board << 14));
        board ^= temp ^ (temp >> 14);
        temp = DIAGONAL_K1.board & (board ^ (board << 7));
        board ^= temp ^ (temp >> 7);

        BitBoard::from(board)
    }

    pub fn flip_antidiagonal(self) -> Self {
        covered_by!("BitBoard::flip_antidiagonal");
        let mut board = self.board;
        let mut temp = board ^ (board << 36);

        board ^= ANTIDIAGONAL_K4.board & (temp ^ (board >> 36));
        temp = ANTIDIAGONAL_K2.board & (board ^ (board << 18));
        board ^= temp ^ (temp >> 18);
        temp = ANTIDIAGONAL_K1.board & (board ^ (board << 9));
        board ^= temp ^ (temp >> 9);

        BitBoard::from(board)
    }

    pub fn rotate_180(self) -> Self {
        covered_by!("BitBoard::rotate_180");
        self.flip_vertical().flip_horizontal()
    }

    pub fn rotate_90cw(self) -> Self {
        covered_by!("BitBoard::rotate_90cw");
        self.flip_diagonal().flip_vertical()
    }

    pub fn rotate_90ccw(self) -> Self {
        covered_by!("BitBoard::rotate_90ccw");
        self.flip_vertical().flip_diagonal()
    }

    pub fn rotate_45cw(self) -> Rotated45BitBoard {
        covered_by!("BitBoard::rotate_45cw");
        let mut board = self.board;

        board ^= ROTATE_45CW_K1.board & (board ^ board.rotate_right(8));
        board ^= ROTATE_45CW_K2.board & (board ^ board.rotate_right(16));
        board ^= ROTATE_45CW_K4.board & (board ^ board.rotate_right(32));

        Rotated45BitBoard::from(board)
    }

    pub fn rotate_45ccw(self) -> Rotated45BitBoard {
        let mut board = self.board;
        board ^= ROTATE_45CCW_K1.board & (board ^ board.rotate_right(8));
        board ^= ROTATE_45CCW_K2.board & (board ^ board.rotate_right(16));
        board ^= ROTATE_45CCW_K4.board & (board ^ board.rotate_right(32));

        // TODO: Do we need a 3rd type for 45ccw?
        Rotated45BitBoard::from(board)
    }

    pub fn to_bitboard(self) -> String {
        let mut bits = String::with_capacity(64 + 8);

        let mut board = self.board;

        for _ in 0..8 {
            bits += &to_bitstring(board, 8);
            bits += "\n";
            board <<= 8;
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        covers!("BitBoard::empty");
        assert_eq!(BitBoard::empty(), BitBoard::new(0));
        assert!(BitBoard::empty().is_empty());
    }

    #[test]
    fn test_join() {
        covers!("BitBoard::join");
        assert_eq!(
            BitBoard::from(RankFile::A1).join(BitBoard::from(RankFile::H8)),
            BitBoard::from(0x8000_0000_0000_0001)
        );

        assert_eq!(
            BitBoard::from(RankFile::A1)
                .join(RankFile::H8.into())
                .join(RankFile::A1.into()),
            BitBoard::from(0x8000_0000_0000_0001)
        );

        assert_eq!(
            WHITE_SQUARES.join(BLACK_SQUARES),
            BitBoard::empty().inverse()
        );
    }

    #[test]
    fn test_intersect() {
        covers!("BitBoard::intersect");

        assert_eq!(
            BitBoard::empty().intersect(WHITE_SQUARES),
            BitBoard::empty()
        );

        assert_eq!(
            BLACK_SQUARES.intersect(BitBoard::from(RankFile::A1)),
            BitBoard::from(RankFile::A1)
        );
    }

    #[test]
    fn test_inverse() {
        covers!("BitBoard::inverse");

        assert_eq!(WHITE_SQUARES.inverse(), BLACK_SQUARES);
        assert_eq!(
            BitBoard::from(1).inverse(),
            BitBoard::from(0xffff_ffff_ffff_fffe)
        );
    }

    #[test]
    fn test_flip_vertical() {
        covers!("BitBoard::flip_vertical");
        assert_eq!(ROW_1.flip_vertical(), ROW_8);
        assert_eq!(ROW_8.flip_vertical(), ROW_1);

        assert_eq!(WHITE_SQUARES.flip_vertical(), BLACK_SQUARES);
        assert_eq!(BLACK_SQUARES.flip_vertical(), WHITE_SQUARES);

        assert_eq!(
            BitBoard::from(RankFile::A1).flip_vertical(),
            BitBoard::from(RankFile::A8)
        );
        assert_eq!(
            BitBoard::from(RankFile::C3).flip_vertical(),
            BitBoard::from(RankFile::C6)
        );
        assert_eq!(
            BitBoard::from(RankFile::G2).flip_vertical(),
            BitBoard::from(RankFile::G7)
        );
        assert_eq!(
            BitBoard::from(RankFile::D4).flip_vertical(),
            BitBoard::from(RankFile::D5)
        );

        assert_eq!(
            BitBoard::from(RankFile::D5).flip_vertical(),
            BitBoard::from(RankFile::D4)
        );
        assert_eq!(
            BitBoard::from(RankFile::H8).flip_vertical(),
            BitBoard::from(RankFile::H1)
        );
    }

    #[test]
    fn test_flip_horizontal() {
        covers!("BitBoard::flip_horizontal");
        assert_eq!(WHITE_SQUARES.flip_horizontal(), BLACK_SQUARES);
        assert_eq!(BLACK_SQUARES.flip_horizontal(), WHITE_SQUARES);

        assert_eq!(
            BitBoard::from(RankFile::A1).flip_horizontal(),
            BitBoard::from(RankFile::H1)
        );
        assert_eq!(
            BitBoard::from(RankFile::C3).flip_horizontal(),
            BitBoard::from(RankFile::F3)
        );
        assert_eq!(
            BitBoard::from(RankFile::G2).flip_horizontal(),
            BitBoard::from(RankFile::B2)
        );
        assert_eq!(
            BitBoard::from(RankFile::D4).flip_horizontal(),
            BitBoard::from(RankFile::E4)
        );
    }

    #[test]
    fn test_flip_diagonal() {
        covers!("BitBoard::flip_diagonal");
        assert_eq!(WHITE_SQUARES.flip_diagonal(), WHITE_SQUARES);
        assert_eq!(BLACK_SQUARES.flip_diagonal(), BLACK_SQUARES);

        assert_eq!(
            BitBoard::from(RankFile::A1).flip_diagonal(),
            BitBoard::from(RankFile::A1)
        );
        assert_eq!(
            BitBoard::from(RankFile::C3).flip_diagonal(),
            BitBoard::from(RankFile::C3)
        );
        assert_eq!(
            BitBoard::from(RankFile::G2).flip_diagonal(),
            BitBoard::from(RankFile::B7)
        );
        assert_eq!(
            BitBoard::from(RankFile::D4).flip_diagonal(),
            BitBoard::from(RankFile::D4)
        );

        assert_eq!(
            BitBoard::from(RankFile::D5).flip_diagonal(),
            BitBoard::from(RankFile::E4)
        );
        assert_eq!(
            BitBoard::from(RankFile::H8).flip_diagonal(),
            BitBoard::from(RankFile::H8)
        );
    }

    #[test]
    fn test_flip_antidiagonal() {
        covers!("BitBoard::flip_antidiagonal");
        assert_eq!(WHITE_SQUARES.flip_antidiagonal(), WHITE_SQUARES);
        assert_eq!(BLACK_SQUARES.flip_antidiagonal(), BLACK_SQUARES);

        assert_eq!(
            BitBoard::from(RankFile::A1).flip_antidiagonal(),
            BitBoard::from(RankFile::H8)
        );
        assert_eq!(
            BitBoard::from(RankFile::C3).flip_antidiagonal(),
            BitBoard::from(RankFile::F6)
        );
        assert_eq!(
            BitBoard::from(RankFile::G2).flip_antidiagonal(),
            BitBoard::from(RankFile::G2)
        );
        assert_eq!(
            BitBoard::from(RankFile::D4).flip_antidiagonal(),
            BitBoard::from(RankFile::E5)
        );

        assert_eq!(
            BitBoard::from(RankFile::D5).flip_antidiagonal(),
            BitBoard::from(RankFile::D5)
        );
        assert_eq!(
            BitBoard::from(RankFile::H8).flip_antidiagonal(),
            BitBoard::from(RankFile::A1)
        );
    }

    #[test]
    fn test_rotate_180() {
        covers!("BitBoard::rotate_180");
        assert_eq!(WHITE_SQUARES.rotate_180(), WHITE_SQUARES);
        assert_eq!(BLACK_SQUARES.rotate_180(), BLACK_SQUARES);

        assert_eq!(
            BitBoard::from(RankFile::A1).rotate_180(),
            BitBoard::from(RankFile::H8)
        );
        assert_eq!(
            BitBoard::from(RankFile::C3).rotate_180(),
            BitBoard::from(RankFile::F6)
        );
        assert_eq!(
            BitBoard::from(RankFile::G2).rotate_180(),
            BitBoard::from(RankFile::B7)
        );
        assert_eq!(
            BitBoard::from(RankFile::D4).rotate_180(),
            BitBoard::from(RankFile::E5)
        );

        assert_eq!(
            BitBoard::from(RankFile::D5).rotate_180(),
            BitBoard::from(RankFile::E4)
        );
        assert_eq!(
            BitBoard::from(RankFile::H8).rotate_180(),
            BitBoard::from(RankFile::A1)
        );
        assert_eq!(
            BitBoard::from(RankFile::F2).rotate_180(),
            BitBoard::from(RankFile::C7)
        );
    }

    #[test]
    fn test_rotate_90cw() {
        covers!("BitBoard::rotate_90cw");
        assert_eq!(WHITE_SQUARES.rotate_90cw(), BLACK_SQUARES);
        assert_eq!(BLACK_SQUARES.rotate_90cw(), WHITE_SQUARES);

        assert_eq!(
            BitBoard::from(RankFile::A1).rotate_90cw(),
            BitBoard::from(RankFile::A8)
        );
        assert_eq!(
            BitBoard::from(RankFile::C3).rotate_90cw(),
            BitBoard::from(RankFile::C6)
        );
        assert_eq!(
            BitBoard::from(RankFile::G2).rotate_90cw(),
            BitBoard::from(RankFile::B2)
        );
        assert_eq!(
            BitBoard::from(RankFile::D4).rotate_90cw(),
            BitBoard::from(RankFile::D5)
        );

        assert_eq!(
            BitBoard::from(RankFile::D5).rotate_90cw(),
            BitBoard::from(RankFile::E5)
        );
        assert_eq!(
            BitBoard::from(RankFile::H8).rotate_90cw(),
            BitBoard::from(RankFile::H1)
        );
        assert_eq!(
            BitBoard::from(RankFile::F2).rotate_90cw(),
            BitBoard::from(RankFile::B3)
        );
    }

    #[test]
    fn test_rotate_90ccw() {
        covers!("BitBoard::rotate_90ccw");
        assert_eq!(WHITE_SQUARES.rotate_90ccw(), BLACK_SQUARES);
        assert_eq!(BLACK_SQUARES.rotate_90ccw(), WHITE_SQUARES);

        assert_eq!(
            BitBoard::from(RankFile::A1).rotate_90ccw(),
            BitBoard::from(RankFile::H1)
        );
        assert_eq!(
            BitBoard::from(RankFile::C3).rotate_90ccw(),
            BitBoard::from(RankFile::F3)
        );
        assert_eq!(
            BitBoard::from(RankFile::G2).rotate_90ccw(),
            BitBoard::from(RankFile::G7)
        );
        assert_eq!(
            BitBoard::from(RankFile::D4).rotate_90ccw(),
            BitBoard::from(RankFile::E4)
        );

        assert_eq!(
            BitBoard::from(RankFile::D5).rotate_90ccw(),
            BitBoard::from(RankFile::D4)
        );
        assert_eq!(
            BitBoard::from(RankFile::H8).rotate_90ccw(),
            BitBoard::from(RankFile::A8)
        );
        assert_eq!(
            BitBoard::from(RankFile::F2).rotate_90ccw(),
            BitBoard::from(RankFile::G6)
        );
    }
}
