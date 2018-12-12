use crate::rank_file::RankFile;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BitPosition {
    // This is called right_index because it's the distance from the
    //  right side of the bits.  
    // 1 << 10 would have a right index of 10...
    // TODO: Figure out a better way to word that
    // TODO: Figure out a better name
    pub(crate) right_index: u32,
}

// NOTE: This is like a vec in that just panics if bounds are violated
impl BitPosition {
    #[inline(always)]
    pub fn shift(self, x: i32, y: i32) -> Self {
        covered_by!("BitPosition::shift");
        covered_by!("BitPosition::shift_errors");
        debug_assert!(
            x + (self.right_index as i32 % 8) >= 0 && x + (self.right_index as i32 % 8) < 8,
            "Attempted to shift a bit left/right outside of the board"
        );

        let right_index = self.right_index as i32 + (8 * y + x);

        debug_assert!(
            right_index >= 0 && right_index < 64,
            "Shifted bit position outside of the board"
        );

        BitPosition {
            right_index: right_index as u32,
        }
    }

    // LOW: Check if modulus on position is faster than
    //  using RANK_A bitboard constant
    pub fn is_leftmost(self) -> bool {
        covered_by!("BitPosition::is_leftmost");
        self.right_index % 8 == 0
    }

    pub fn is_rightmost(self) -> bool {
        covered_by!("BitPosition::is_rightmost");
        self.right_index % 8 == 7
    }
}

impl From<u32> for BitPosition {
    fn from(right_index: u32) -> Self {
        BitPosition { right_index }
    }
}

impl From<RankFile> for BitPosition {
    fn from(rank_file: RankFile) -> Self {
        BitPosition {
            right_index: rank_file as u32,
        }
    }
}

impl From<(u8, u8)> for BitPosition {
    fn from((rank, file): (u8, u8)) -> Self {
        BitPosition {
            right_index: u32::from(rank * 8 + file),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::catch_unwind;

    #[test]
    fn test_shift() {
        covers!("BitPosition::shift");
        let position = BitPosition::from(RankFile::D4);
        assert_eq!(position.shift(1, 1), BitPosition::from(RankFile::E5));
        assert_eq!(position.shift(4, 0), BitPosition::from(RankFile::H4));
        assert_eq!(position.shift(4, 1), BitPosition::from(RankFile::H5));
        assert_eq!(position.shift(4, 4), BitPosition::from(RankFile::H8));
        assert_eq!(position.shift(-3, 0), BitPosition::from(RankFile::A4));
        assert_eq!(position.shift(-3, -3), BitPosition::from(RankFile::A1));
        assert_eq!(position.shift(-3, 3), BitPosition::from(RankFile::A7));
    }

    #[test]
    fn test_shift_bounds() {
        covers!("BitPosition::shift_errors");
        let position = BitPosition::from(RankFile::A1);

        assert!(catch_unwind(|| position.shift(-1, -1)).is_err());
        assert!(catch_unwind(|| position.shift(-1, 0)).is_err());
        assert!(catch_unwind(|| position.shift(0, -1)).is_err());
        assert!(catch_unwind(|| position.shift(8, 0)).is_err());
        assert!(catch_unwind(|| position.shift(8, 8)).is_err());
        assert!(catch_unwind(|| position.shift(0, 8)).is_err());
    }

    #[test]
    fn test_is_leftmost() {
        covers!("BitPosition::is_leftmost");
        let should_be_true = vec![
            RankFile::A1,
            RankFile::A2,
            RankFile::A3,
            RankFile::A4,
            RankFile::A5,
            RankFile::A6,
            RankFile::A7,
            RankFile::A8,
        ];

        let should_be_false = vec![
            RankFile::H1,
            RankFile::H2,
            RankFile::H3,
            RankFile::H4,
            RankFile::H5,
            RankFile::H6,
            RankFile::H7,
            RankFile::H8,
            RankFile::B1,
            RankFile::G8,
            RankFile::D3,
            RankFile::F7,

        ];

        for rf in should_be_true {
            assert!(BitPosition::from(rf).is_leftmost());
        }

        for rf in should_be_false {
            assert!(!BitPosition::from(rf).is_leftmost());
        }

    }

    #[test]
    fn test_is_rightmost() {
        covers!("BitPosition::is_rightmost");
        let should_be_true = vec![
            RankFile::H1,
            RankFile::H2,
            RankFile::H3,
            RankFile::H4,
            RankFile::H5,
            RankFile::H6,
            RankFile::H7,
            RankFile::H8,
        ];

        let should_be_false = vec![
            RankFile::A1,
            RankFile::A2,
            RankFile::A3,
            RankFile::A4,
            RankFile::A5,
            RankFile::A6,
            RankFile::A7,
            RankFile::A8,
            RankFile::B1,
            RankFile::G8,
            RankFile::D3,
            RankFile::F7,

        ];

        for rf in should_be_true {
            assert!(BitPosition::from(rf).is_rightmost());
        }

        for rf in should_be_false {
            assert!(!BitPosition::from(rf).is_rightmost());
        }


    }

}
