// // As long as this is just a u64 we should be able to clone.
// #[derive(Copy, Clone)]
// pub struct BitBoard {
//     board: u64,
// }
use std::num::Wrapping;

const ROW_1: u64 = 0xff00000000000000;
const ROW_2: u64 = 0x00ff000000000000;
const ROW_3: u64 = 0x0000ff0000000000;
const ROW_4: u64 = 0x000000ff00000000;
const ROW_5: u64 = 0x00000000ff000000;
const ROW_6: u64 = 0x0000000000ff0000;
const ROW_7: u64 = 0x000000000000ff00;
const ROW_8: u64 = 0x00000000000000ff;

const WHITE_SQUARES: u64 = 0b0101010110101010010101011010101001010101101010100101010110101010;
const BLACK_SQUARES: u64 = 0b1010101001010101101010100101010110101010010101011010101001010101;

const k1: u64 = (0x5555555555555555);
const k2: u64 = (0x3333333333333333);
const k4: u64 = (0x0f0f0f0f0f0f0f0f);

trait BitBoard {
    fn flip_vertical(self) -> Self;
    fn flip_horizontal(self) -> Self;
}

impl BitBoard for u64 {
    fn flip_vertical(self) -> Self {
        self.swap_bytes()
    }

    fn flip_horizontal(mut self) -> Self {

        self = (Wrapping((self >> 1) & k1) + Wrapping(2) * Wrapping(self & k1)).0;
        self = (Wrapping((self >> 2) & k2) + Wrapping(4) * Wrapping(self & k2)).0;
        self = (Wrapping((self >> 4) & k4) + Wrapping(16) * Wrapping(self & k4)).0;
        self

        // self ^= k4 & (self ^ self.rotate_left(8));
        // self ^= k2 & (self ^ self.rotate_left(4));
        // self ^= k1 & (self ^ self.rotate_left(2));

        // self.rotate_right(7)
    }
}

// impl BitBoard {
//     pub fn flip_vertical(self) -> BitBoard {
//         BitBoard {
//             board: self.board.swap_bytes()
//         }
//     }

//     pub fn flip_horizontal(self) -> BitBoard {

//     }
// }

// impl From<u64> for BitBoard {
//     fn from(board: u64) -> BitBoard {
//         BitBoard { board }
//     }
// }

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
