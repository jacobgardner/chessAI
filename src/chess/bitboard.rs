use std::num::Wrapping;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::chess::BitPosition;
use crate::chess::RankFile;

pub const FILE_8: BitBoard = BitBoard::new(0xff00_0000_0000_0000);
pub const FILE_7: BitBoard = BitBoard::new(0x00ff_0000_0000_0000);
pub const FILE_6: BitBoard = BitBoard::new(0x0000_ff00_0000_0000);
pub const FILE_5: BitBoard = BitBoard::new(0x0000_00ff_0000_0000);
pub const FILE_4: BitBoard = BitBoard::new(0x0000_0000_ff00_0000);
pub const FILE_3: BitBoard = BitBoard::new(0x0000_0000_00ff_0000);
pub const FILE_2: BitBoard = BitBoard::new(0x0000_0000_0000_ff00);
pub const FILE_1: BitBoard = BitBoard::new(0x0000_0000_0000_00ff);

pub const RANK_A: BitBoard = BitBoard::new(0x0101_0101_0101_0101);
pub const RANK_B: BitBoard = BitBoard::new(0x0202_0202_0202_0202);
pub const RANK_C: BitBoard = BitBoard::new(0x0404_0404_0404_0404);
pub const RANK_D: BitBoard = BitBoard::new(0x0808_0808_0808_0808);
pub const RANK_E: BitBoard = BitBoard::new(0x1010_1010_1010_1010);
pub const RANK_F: BitBoard = BitBoard::new(0x2020_2020_2020_2020);
pub const RANK_G: BitBoard = BitBoard::new(0x4040_4040_4040_4040);
pub const RANK_H: BitBoard = BitBoard::new(0x8080_8080_8080_8080);

pub const ENDS: BitBoard = BitBoard::new(FILE_1.board | FILE_8.board);
pub const SIDES: BitBoard = BitBoard::new(RANK_A.board | RANK_H.board);

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

#[derive(PartialEq, Clone, Copy)]
pub struct BitBoard {
    board: u64,
}

impl std::fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "\n{}", self.to_bitboard())
    }
}

impl From<u64> for BitBoard {
    fn from(bits: u64) -> Self {
        BitBoard { board: bits }
    }
}

impl From<BitPosition> for BitBoard {
    fn from(position: BitPosition) -> Self {
        BitBoard {
            board: 1 << position.right_index,
        }
    }
}

impl From<RankFile> for BitBoard {
    fn from(rank_file: RankFile) -> Self {
        BitBoard::from(BitPosition::from(rank_file))
    }
}

impl Sub for BitBoard {
    type Output = BitBoard;

    fn sub(self, rhs: BitBoard) -> Self::Output {
        self.sub(rhs)
    }
}

impl SubAssign for BitBoard {
    fn sub_assign(&mut self, rhs: BitBoard) {
        *self = self.sub(rhs);
    }
}

impl Add for BitBoard {
    type Output = BitBoard;

    fn add(self, rhs: BitBoard) -> Self::Output {
        self.join(rhs)
    }
}

impl AddAssign for BitBoard {
    fn add_assign(&mut self, rhs: BitBoard) {
        *self = self.join(rhs);
    }
}

impl BitBoard {
    pub const fn new(board: u64) -> Self {
        BitBoard { board }
    }

    pub fn empty() -> Self {
        covered_by!("BitBoard::empty");
        BitBoard { board: 0 }
    }

    pub fn join(self, rhs: BitBoard) -> Self {
        covered_by!("BitBoard::join");
        BitBoard::from(self.board | rhs.board)
    }

    pub fn intersect(self, rhs: BitBoard) -> Self {
        covered_by!("BitBoard::intersect");
        BitBoard::from(self.board & rhs.board)
    }

    pub fn inverse(self) -> Self {
        covered_by!("BitBoard::inverse");
        BitBoard::from(!self.board)
    }

    pub fn is_empty(self) -> bool {
        self.board == 0
    }

    pub fn shift_down(self) -> Self {
        covered_by!("BitBoard::shift_down");
        BitBoard::from(self.board >> 8)
    }

    pub fn shift_up(self) -> Self {
        covered_by!("BitBoard::shift_up");
        BitBoard::from(self.board << 8)
    }

    pub fn count_pieces(self) -> u32 {
        covered_by!("BitBoard::count_pieces");
        self.board.count_ones()
    }

    // TODO: Rename.  This sucks
    pub fn first_bit_position(self) -> BitPosition {
        BitPosition::from(self.board.trailing_zeros())
    }

    pub fn sub(self, rhs: BitBoard) -> BitBoard {
        self.intersect(rhs.inverse())
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
    // format!("{:0padding$b}\n", (bits & FILE_8) >> (64 - 8), padding = padding).chars().rev().collect::<String>()
    format!(
        "{:0padding$b}",
        (bits & FILE_8.board) >> (64 - 8),
        padding = padding as usize
    )
    .chars()
    .rev()
    .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: u64 = 0b001011001;
    const B: u64 = 0b100110111;

    const A_MIN_B: u64 = 0b001001000;
    const A_JOIN_B: u64 = 0b101111111;

    #[test]
    fn test_sub() {
        assert_eq!(BitBoard::new(A) - BitBoard::new(B), BitBoard::new(A_MIN_B));
    }

    #[test]
    fn test_subassign() {
        let mut c_a = BitBoard::new(A);
        c_a -= BitBoard::new(B);

        assert_eq!(c_a, BitBoard::new(A_MIN_B));
    }

    #[test]
    fn test_add() {
        assert_eq!(BitBoard::new(A) + BitBoard::new(B), BitBoard::new(A_JOIN_B));
    }

    #[test]
    fn test_addassign() {
        let mut c_a = BitBoard::new(A);
        c_a += BitBoard::new(B);

        assert_eq!(c_a, BitBoard::new(A_JOIN_B));
    }

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
    fn test_count_pieces() {
        covers!("BitBoard::count_pieces");

        assert_eq!(WHITE_SQUARES.count_pieces(), 32);
        assert_eq!(BLACK_SQUARES.count_pieces(), 32);
        assert_eq!(RANK_D.count_pieces(), 8);
        assert_eq!(FILE_4.count_pieces(), 8);

        assert_eq!(BitBoard::new(0x00100).count_pieces(), 1);

    }

    #[test]
    fn test_shift_down() {
        covers!("BitBoard::shift_down");

        assert_eq!(FILE_4.shift_down(), FILE_3);
    }

    #[test]
    fn test_shift_up() {
        covers!("BitBoard::shift_up");

        assert_eq!(FILE_4.shift_up(), FILE_5);
    }

    #[test]
    fn test_flip_vertical() {
        covers!("BitBoard::flip_vertical");
        assert_eq!(FILE_1.flip_vertical(), FILE_8);
        assert_eq!(FILE_8.flip_vertical(), FILE_1);

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
