use std::cmp::min;
use std::num::Wrapping;
use std::ops::{BitOr, BitOrAssign, Sub, SubAssign};

use crate::chess::{BitPosition, RankFile};

pub const FILE_H: BitBoard = BitBoard::new(0xff00_0000_0000_0000);
pub const FILE_G: BitBoard = BitBoard::new(0x00ff_0000_0000_0000);
pub const FILE_F: BitBoard = BitBoard::new(0x0000_ff00_0000_0000);
pub const FILE_E: BitBoard = BitBoard::new(0x0000_00ff_0000_0000);
pub const FILE_D: BitBoard = BitBoard::new(0x0000_0000_ff00_0000);
pub const FILE_C: BitBoard = BitBoard::new(0x0000_0000_00ff_0000);
pub const FILE_B: BitBoard = BitBoard::new(0x0000_0000_0000_ff00);
pub const FILE_A: BitBoard = BitBoard::new(0x0000_0000_0000_00ff);

pub const RANK_1: BitBoard = BitBoard::new(0x0101_0101_0101_0101);
pub const RANK_2: BitBoard = BitBoard::new(0x0202_0202_0202_0202);
pub const RANK_3: BitBoard = BitBoard::new(0x0404_0404_0404_0404);
pub const RANK_4: BitBoard = BitBoard::new(0x0808_0808_0808_0808);
pub const RANK_5: BitBoard = BitBoard::new(0x1010_1010_1010_1010);
pub const RANK_6: BitBoard = BitBoard::new(0x2020_2020_2020_2020);
pub const RANK_7: BitBoard = BitBoard::new(0x4040_4040_4040_4040);
pub const RANK_8: BitBoard = BitBoard::new(0x8080_8080_8080_8080);

pub const ENDS: BitBoard = BitBoard::new(FILE_A.board | FILE_H.board);
pub const SIDES: BitBoard = BitBoard::new(RANK_1.board | RANK_8.board);

pub const QUEENSIDE_CASTLE: BitBoard = BitBoard::new(0b0110_0000);
pub const KINGSIDE_CASTLE: BitBoard = BitBoard::new(0b0000_1110);
pub const CASTLE_CHECK: BitBoard = BitBoard::new(0b0110_1100 << 56 | 0b0110_1100);

pub const RANKS: [BitBoard; 8] = [
    RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8,
];
pub const FILES: [BitBoard; 8] = [
    FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H,
];

lazy_static! {
    static ref LEFT_SHIFT_MASK: [BitBoard; 9] = {
        let mut masks: [BitBoard; 9] = [BitBoard::empty(); 9];

        for shift in 1..9 {
            masks[shift] = BitBoard::new(RANK_8.board >> (shift - 1)).join(masks[shift - 1])
        }

        masks
    };
    static ref RIGHT_SHIFT_MASK: [BitBoard; 9] = {
        let mut masks: [BitBoard; 9] = [BitBoard::empty(); 9];

        for shift in 1..9 {
            masks[shift] = BitBoard::new(RANK_1.board << (shift - 1)).join(masks[shift - 1])
        }

        masks
    };
}

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

/// For two boards, `A` and `B` if we perform, `A - B` the result will be
///    the unsetting of any bits (or spaces) in `A` that exist in `B`.
///
/// ```
/// # use lib::chess::BitBoard;
/// assert_eq!(
///     BitBoard::new(0b11001100)
///   - BitBoard::new(0b01100110),
///     BitBoard::new(0b10001000)   
/// );
/// ```
impl Sub for BitBoard {
    type Output = BitBoard;

    fn sub(self, rhs: BitBoard) -> Self::Output {
        self.intersect(rhs.inverse())
    }
}

impl SubAssign for BitBoard {
    fn sub_assign(&mut self, rhs: BitBoard) {
        *self = *self - rhs;
    }
}

/// For two boards, `A` and `B` if we perform, `A | B` the result will be
///    the setting of any bits (or spaces) in `A` or exist in `B`.
///
/// ```
/// # use lib::chess::BitBoard;
/// assert_eq!(
///     BitBoard::new(0b11001100)
///   | BitBoard::new(0b01100110),
///     BitBoard::new(0b11101110)   
/// );
/// ```
impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: BitBoard) -> Self::Output {
        self.join(rhs)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: BitBoard) {
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

    pub fn shift_down(self, count: usize) -> Self {
        covered_by!("BitBoard::shift_down");
        BitBoard::from(self.board >> (8 * count))
    }

    pub fn shift_up(self, count: usize) -> Self {
        covered_by!("BitBoard::shift_up");
        BitBoard::from(self.board << (8 * count))
    }

    /// Shifts all the pieces left on the board by `count` spaces. If a piece is pushed left off the board
    /// it is removed.
    ///
    /// # Panics
    ///
    /// This function panics if `count` is greater than 8.
    pub fn shift_left(self, count: usize) -> Self {
        covered_by!("BitBoard::shift_left");
        BitBoard::from(self.board >> count) - LEFT_SHIFT_MASK[count]
    }

    // TODO: We could join shift_left/shift_right into shift_horizontal or something...
    /// See shift_left for details.  This has the same result but to the right and same constraints.
    pub fn shift_right(self, count: usize) -> Self {
        covered_by!("BitBoard::shift_right");
        BitBoard::from(self.board << count) - RIGHT_SHIFT_MASK[count]
    }

    pub fn shift(self, rank_count: i32, file_count: i32) -> Self {
        let vert_board = if rank_count >= 0 {
            self.shift_up(rank_count as usize)
        } else {
            self.shift_down(-rank_count as usize)
        };

        if file_count >= 0 {
            vert_board.shift_right(file_count as usize)
        } else {
            vert_board.shift_left(-file_count as usize)
        }
    }

    pub fn count_pieces(self) -> u32 {
        covered_by!("BitBoard::count_pieces");
        self.board.count_ones()
    }

    fn count_left_spaces_inclusive(self, position: BitPosition) -> u32 {
        covered_by!("BitBoard::count_left_spaces_inclusive");

        debug_assert!(position.right_index < 64);

        let shifted_board = if position.right_index > 0 {
            self.board << (64 - position.right_index)
        } else {
            0
        };

        let max_left_spaces = (position.right_index) % 8;
        min(shifted_board.leading_zeros() + 1, max_left_spaces)
    }

    fn count_right_spaces_inclusive(self, position: BitPosition) -> u32 {
        covered_by!("BitBoard::count_right_spaces_inclusive");
        debug_assert!(position.right_index < 64);

        let shifted_board = if position.right_index < 63 {
            self.board >> (position.right_index + 1)
        } else {
            0
        };

        let max_right_spaces = 7 - (position.right_index % 8);
        min(shifted_board.trailing_zeros() + 1, max_right_spaces)
    }

    pub fn fill_spaces(self, start: u32, end: u32) -> BitBoard {
        covered_by!("BitBoard::fill_spaces");
        debug_assert!(start <= 64, "Start is past final board index");
        debug_assert!(end <= 64, "End is past final board index");
        debug_assert!(start <= end, "Start must appear before end");

        let count = end - start;

        if count == 64 {
            BitBoard::empty().inverse()
        } else if count == 0 {
            self
        } else {
            let bits = ((1 << count) - 1) << start;

            self.join(BitBoard::new(bits))
        }
    }

    // The returned bitboard includes up to a single collision per side
    pub fn horizontal_slides(self, position: BitPosition) -> BitBoard {
        covered_by!("BitBoard::horizontal_slides");

        let left_spaces = self.count_left_spaces_inclusive(position);
        let right_spaces = self.count_right_spaces_inclusive(position);

        BitBoard::empty()
            .fill_spaces(
                position.right_index + 1,
                position.right_index + right_spaces + 1,
            )
            .fill_spaces(position.right_index - left_spaces, position.right_index)
    }

    // TODO: Test
    // TODO: This should probably be an option... for when it's empty
    // TODO: Rename.  This sucks
    pub fn first_bit_position(self) -> BitPosition {
        debug_assert!(!self.is_empty());
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

    // TODO: Test
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

impl Iterator for BitBoard {
    type Item = BitPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            return None;
        }

        let position = self.first_bit_position();

        *self -= position.into();

        Some(position)
    }
}

fn bitrange(start: u64, end: u64) -> u64 {
    ((1 << start as u64) - 1) ^ ((1 << end as u64) - 1)
}

fn to_bitstring(bits: u64, padding: u64) -> String {
    format!(
        "{:0padding$b}",
        (bits & FILE_H.board) >> (64 - 8),
        padding = padding as usize
    )
    .chars()
    .rev()
    .collect::<String>()
}

// TODO: Clean up tests
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
        assert_eq!(BitBoard::new(A) | BitBoard::new(B), BitBoard::new(A_JOIN_B));
    }

    #[test]
    fn test_addassign() {
        let mut c_a = BitBoard::new(A);
        c_a |= BitBoard::new(B);

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
        assert_eq!(RANK_4.count_pieces(), 8);
        assert_eq!(FILE_D.count_pieces(), 8);

        assert_eq!(BitBoard::new(0x00100).count_pieces(), 1);
    }

    #[test]
    fn test_count_left_spaces_inclusive() {
        covers!("BitBoard::count_left_spaces_inclusive");

        assert_eq!(
            BitBoard::new(0).count_left_spaces_inclusive(BitPosition::from(7)),
            7
        );
        assert_eq!(
            BitBoard::new(0).count_left_spaces_inclusive(BitPosition::from(4)),
            4
        );
        assert_eq!(
            BitBoard::new(0).count_left_spaces_inclusive(BitPosition::from(0)),
            0
        );

        assert_eq!(
            BitBoard::new(0).count_left_spaces_inclusive(BitPosition::from(8)),
            0
        );
        assert_eq!(
            BitBoard::new(0).count_left_spaces_inclusive(BitPosition::from(11)),
            3
        );
        assert_eq!(
            BitBoard::new(0).count_left_spaces_inclusive(BitPosition::from(15)),
            7
        );

        assert_eq!(
            BitBoard::new(0).count_left_spaces_inclusive(BitPosition::from(63)),
            7
        );
        assert_eq!(
            BitBoard::new(0).count_left_spaces_inclusive(BitPosition::from(60)),
            4
        );
        assert_eq!(
            BitBoard::new(0).count_left_spaces_inclusive(BitPosition::from(56)),
            0
        );

        assert_eq!(
            BitBoard::new(1 << 5 | 1 << 7).count_left_spaces_inclusive(BitPosition::from(7)),
            2
        );
        assert_eq!(
            BitBoard::new(1).count_left_spaces_inclusive(BitPosition::from(4)),
            4
        );
        assert_eq!(
            BitBoard::new(1 << 4).count_left_spaces_inclusive(BitPosition::from(0)),
            0
        );

        assert_eq!(
            BitBoard::new(1 << 10).count_left_spaces_inclusive(BitPosition::from(8)),
            0
        );
        assert_eq!(
            BitBoard::new(1 << 9).count_left_spaces_inclusive(BitPosition::from(11)),
            2
        );
        assert_eq!(
            BitBoard::new(1 << 10).count_left_spaces_inclusive(BitPosition::from(15)),
            5
        );

        assert_eq!(
            BitBoard::new(1 << 60).count_left_spaces_inclusive(BitPosition::from(63)),
            3
        );
        assert_eq!(
            BitBoard::new(1 << 56).count_left_spaces_inclusive(BitPosition::from(60)),
            4
        );
        assert_eq!(
            BitBoard::new(1 << 60).count_left_spaces_inclusive(BitPosition::from(56)),
            0
        );
    }

    #[test]
    fn test_count_right_spaces_inclusive() {
        covers!("BitBoard::count_right_spaces_inclusive");

        assert_eq!(
            BitBoard::new(0).count_right_spaces_inclusive(BitPosition::from(7)),
            0
        );
        assert_eq!(
            BitBoard::new(0).count_right_spaces_inclusive(BitPosition::from(4)),
            3
        );
        assert_eq!(
            BitBoard::new(0).count_right_spaces_inclusive(BitPosition::from(0)),
            7
        );

        assert_eq!(
            BitBoard::new(0).count_right_spaces_inclusive(BitPosition::from(8)),
            7
        );
        assert_eq!(
            BitBoard::new(0).count_right_spaces_inclusive(BitPosition::from(11)),
            4
        );
        assert_eq!(
            BitBoard::new(0).count_right_spaces_inclusive(BitPosition::from(15)),
            0
        );

        assert_eq!(
            BitBoard::new(0).count_right_spaces_inclusive(BitPosition::from(63)),
            0
        );
        assert_eq!(
            BitBoard::new(0).count_right_spaces_inclusive(BitPosition::from(60)),
            3
        );
        assert_eq!(
            BitBoard::new(0).count_right_spaces_inclusive(BitPosition::from(56)),
            7
        );

        assert_eq!(
            BitBoard::new(1 << 5 | 1 << 7).count_right_spaces_inclusive(BitPosition::from(7)),
            0
        );
        assert_eq!(
            BitBoard::new(1).count_right_spaces_inclusive(BitPosition::from(4)),
            3
        );
        assert_eq!(
            BitBoard::new(1 << 4).count_right_spaces_inclusive(BitPosition::from(0)),
            4
        );

        assert_eq!(
            BitBoard::new(1 << 10).count_right_spaces_inclusive(BitPosition::from(8)),
            2
        );
        assert_eq!(
            BitBoard::new(1 << 9).count_right_spaces_inclusive(BitPosition::from(11)),
            4
        );
        assert_eq!(
            BitBoard::new(1 << 10).count_right_spaces_inclusive(BitPosition::from(15)),
            0
        );

        assert_eq!(
            BitBoard::new(1 << 60).count_right_spaces_inclusive(BitPosition::from(63)),
            0
        );
        assert_eq!(
            BitBoard::new(1 << 56).count_right_spaces_inclusive(BitPosition::from(60)),
            3
        );
        assert_eq!(
            BitBoard::new(1 << 60).count_right_spaces_inclusive(BitPosition::from(56)),
            4
        );
    }

    #[test]
    fn test_fill_spaces() {
        covers!("BitBoard::fill_spaces");

        assert_eq!(
            BitBoard::empty().fill_spaces(0, 64),
            BitBoard::empty().inverse()
        );

        assert_eq!(BitBoard::empty().fill_spaces(64, 64), BitBoard::empty());
        assert_eq!(BitBoard::empty().fill_spaces(24, 24), BitBoard::empty());
        assert_eq!(BitBoard::empty().fill_spaces(0, 0), BitBoard::empty());

        // TODO: we could make this const fn and derive FILE_A through FILE_H
        //  using this function instead.  If we do, we can't use FILE for testing
        assert_eq!(BitBoard::empty().fill_spaces(0, 8), FILE_A);
        assert_eq!(BitBoard::empty().fill_spaces(8, 16), FILE_B);
        assert_eq!(BitBoard::empty().fill_spaces(16, 24), FILE_C);
        assert_eq!(BitBoard::empty().fill_spaces(24, 32), FILE_D);
        assert_eq!(BitBoard::empty().fill_spaces(32, 40), FILE_E);
        assert_eq!(BitBoard::empty().fill_spaces(40, 48), FILE_F);
        assert_eq!(BitBoard::empty().fill_spaces(48, 56), FILE_G);
        assert_eq!(BitBoard::empty().fill_spaces(56, 64), FILE_H);

        assert_eq!(BitBoard::empty().fill_spaces(4, 8), BitBoard::new(0xF0));

        assert_eq!(
            WHITE_SQUARES.fill_spaces(56, 64),
            WHITE_SQUARES.join(FILE_H)
        );
    }

    #[test]
    fn test_shift_down() {
        covers!("BitBoard::shift_down");

        assert_eq!(FILE_D.shift_down(1), FILE_C);
        assert_eq!(FILE_A.shift_down(1), BitBoard::empty());
        assert_eq!(FILE_F.shift_down(2), FILE_D);
        assert_eq!(FILE_H.shift_down(7), FILE_A);
    }

    #[test]
    fn test_horizontal_slides() {
        covers!("BitBoard::horizontal_slides");

        //                              R    X     L
        let all_pieces = BitBoard::new(0b100_1_0001);
        let slides = all_pieces.horizontal_slides(BitPosition::from(4));
        assert_eq!(slides, BitBoard::from(0b111_0_1111));

        //                              R    X     L
        let all_pieces = BitBoard::new(0b100_1_0001 << 8 * 7);
        let slides = all_pieces.horizontal_slides(BitPosition::from(4 + 8 * 7));
        assert_eq!(slides, BitBoard::from(0b111_0_1111 << 8 * 7));

        //                              R    X     L
        let all_pieces = BitBoard::new(0b100_1_0010);
        let slides = all_pieces.horizontal_slides(BitPosition::from(4));
        assert_eq!(slides, BitBoard::from(0b111_0_1110));

        //                              R    X     L
        let all_pieces = BitBoard::new(0b100_1_0010 << 8 * 7);
        let slides = all_pieces.horizontal_slides(BitPosition::from(4 + 8 * 7));
        assert_eq!(slides, BitBoard::from(0b111_0_1110 << 8 * 7));

        //                              R    X     L
        let all_pieces = BitBoard::new(0b000_1_0001);
        let slides = all_pieces.horizontal_slides(BitPosition::from(4));
        assert_eq!(slides, BitBoard::from(0b111_0_1111));

        //                              R    X     L
        let all_pieces = BitBoard::new(0b000_1_0000 << (2 * 8));
        let slides = all_pieces.horizontal_slides(BitPosition::from(4 + 2 * 8));
        assert_eq!(slides, BitBoard::from(0b111_0_1111 << (2 * 8)));

        let all_pieces = BitBoard::new(0b10000000 << (2 * 8));
        let slides = all_pieces.horizontal_slides(BitPosition::from(7 + 2 * 8));
        assert_eq!(slides, BitBoard::from(0b01111111 << (2 * 8)));

        let all_pieces = BitBoard::new(0b00000001 << (2 * 8));
        let slides = all_pieces.horizontal_slides(BitPosition::from(0 + 2 * 8));
        assert_eq!(slides, BitBoard::from(0b11111110 << (2 * 8)));
    }

    #[test]
    fn test_shift_up() {
        covers!("BitBoard::shift_up");

        assert_eq!(FILE_D.shift_up(1), FILE_E);
        assert_eq!(FILE_D.shift_up(3), FILE_G);
        assert_eq!(FILE_G.shift_up(1), FILE_H);
        assert_eq!(FILE_G.shift_up(2), BitBoard::empty());
    }

    #[test]
    fn test_shift_left() {
        covers!("BitBoard::shift_left");

        assert_eq!(RANK_2.shift_left(1), RANK_1);
        assert_eq!(RANK_2.shift_left(2), BitBoard::empty());
        assert_eq!(RANK_2.shift_left(3), BitBoard::empty());
        assert_eq!(RANK_2.shift_left(5), BitBoard::empty());
        assert_eq!(RANK_2.shift_left(8), BitBoard::empty());

        assert_eq!(RANK_5.shift_left(1), RANK_4);
        assert_eq!(RANK_5.shift_left(2), RANK_3);
        assert_eq!(RANK_5.shift_left(3), RANK_2);
        assert_eq!(RANK_5.shift_left(4), RANK_1);
        assert_eq!(RANK_5.shift_left(8), BitBoard::empty());
        assert_eq!(RANK_5.shift_left(5), BitBoard::empty());

        assert_eq!(RANK_8.shift_left(1), RANK_7);
        assert_eq!(RANK_8.shift_left(2), RANK_6);
        assert_eq!(RANK_8.shift_left(3), RANK_5);
        assert_eq!(RANK_8.shift_left(4), RANK_4);
        assert_eq!(RANK_8.shift_left(5), RANK_3);
        assert_eq!(RANK_8.shift_left(6), RANK_2);
        assert_eq!(RANK_8.shift_left(7), RANK_1);
        assert_eq!(RANK_8.shift_left(8), BitBoard::empty());
    }

    #[test]
    fn test_shift_right() {
        covers!("BitBoard::shift_right");

        assert_eq!(RANK_2.shift_right(1), RANK_3);
        assert_eq!(RANK_2.shift_right(2), RANK_4);
        assert_eq!(RANK_2.shift_right(3), RANK_5);
        assert_eq!(RANK_2.shift_right(5), RANK_7);
        assert_eq!(RANK_2.shift_right(8), BitBoard::empty());

        assert_eq!(RANK_5.shift_right(1), RANK_6);
        assert_eq!(RANK_5.shift_right(2), RANK_7);
        assert_eq!(RANK_5.shift_right(3), RANK_8);
        assert_eq!(RANK_5.shift_right(4), BitBoard::empty());
        assert_eq!(RANK_5.shift_right(8), BitBoard::empty());
        assert_eq!(RANK_5.shift_right(5), BitBoard::empty());

        assert_eq!(RANK_7.shift_right(1), RANK_8);
        assert_eq!(RANK_7.shift_right(2), BitBoard::empty());
        assert_eq!(RANK_7.shift_right(3), BitBoard::empty());
        assert_eq!(RANK_7.shift_right(4), BitBoard::empty());
        assert_eq!(RANK_7.shift_right(5), BitBoard::empty());
    }

    #[test]
    fn test_flip_vertical() {
        covers!("BitBoard::flip_vertical");
        assert_eq!(FILE_A.flip_vertical(), FILE_H);
        assert_eq!(FILE_H.flip_vertical(), FILE_A);

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
