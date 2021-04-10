use variant_count::VariantCount;

#[derive(Debug, PartialEq, FromPrimitive, Clone, Copy, VariantCount)]
pub enum PieceType {
    Pawn = 0,
    Rook = 1,
    Knight = 2,
    Bishop = 3,
    Queen = 4,
    King = 5,
}

use self::PieceType::*;

impl PieceType {
    pub fn from(chr: char) -> Option<PieceType> {
        let piece_type = match chr.to_lowercase().next().unwrap_or('x') {
            'p' => Pawn,
            'r' => Rook,
            'n' => Knight,
            'b' => Bishop,
            'q' => Queen,
            'k' => King,
            _ => return None,
        };

        Some(piece_type)
    }

    pub fn to_char(self) -> char {
        match self {
            Pawn => 'P',
            Rook => 'R',
            Knight => 'N',
            Bishop => 'B',
            Queen => 'Q',
            King => 'K',
        }
    }
}

#[test]
fn test_piece_from_str() {
    assert_eq!(PieceType::from('P'), Some(Pawn));
    assert_eq!(PieceType::from('p'), Some(Pawn));

    assert_eq!(PieceType::from('R'), Some(Rook));
    assert_eq!(PieceType::from('r'), Some(Rook));

    assert_eq!(PieceType::from('N'), Some(Knight));
    assert_eq!(PieceType::from('n'), Some(Knight));

    assert_eq!(PieceType::from('B'), Some(Bishop));
    assert_eq!(PieceType::from('b'), Some(Bishop));

    assert_eq!(PieceType::from('Q'), Some(Queen));
    assert_eq!(PieceType::from('q'), Some(Queen));

    assert_eq!(PieceType::from('K'), Some(King));
    assert_eq!(PieceType::from('k'), Some(King));

    assert_eq!(PieceType::from('x'), None);
    assert_eq!(PieceType::from('L'), None);
}
