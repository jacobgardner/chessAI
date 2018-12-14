use crate::chess::bitposition::BitPosition;
use num;

#[cfg_attr( rustfmt, rustfmt_skip)]
#[derive(PartialEq, Clone, Copy, Debug, FromPrimitive)]
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

impl From<BitPosition> for RankFile {
    fn from(position: BitPosition) -> Self {
        num::FromPrimitive::from_u32(position.right_index).unwrap()
    }
}

impl RankFile {
    pub fn rank(self) -> u8 {
        self as u8 % 8
    }

    pub fn file(self) -> u8 {
        self as u8 / 8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank() {
        assert_eq!(RankFile::A1.rank(), 0);
        assert_eq!(RankFile::A3.rank(), 0);
        assert_eq!(RankFile::A8.rank(), 0);
        assert_eq!(RankFile::B2.rank(), 1);
        assert_eq!(RankFile::C8.rank(), 2);
        assert_eq!(RankFile::D1.rank(), 3);
        assert_eq!(RankFile::H8.rank(), 7);
    }

    #[test]
    fn test_file() {
        assert_eq!(RankFile::A1.file(), 0);
        assert_eq!(RankFile::A3.file(), 2);
        assert_eq!(RankFile::A8.file(), 7);
        assert_eq!(RankFile::B2.file(), 1);
        assert_eq!(RankFile::C8.file(), 7);
        assert_eq!(RankFile::D1.file(), 0);
        assert_eq!(RankFile::H8.file(), 7);
    }
}
