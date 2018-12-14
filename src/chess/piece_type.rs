#[derive(Debug, PartialEq, FromPrimitive, Clone, Copy)]
pub enum PieceType {
    Pawn = 0,
    Rook = 1,
    Knight = 2,
    Bishop = 3,
    Queen = 4,
    King = 5,
}

impl PieceType {
    pub fn from(chr: char) -> Option<PieceType> {
        let piece_type = match chr.to_lowercase().next().unwrap_or('x') {
            'p' => PieceType::Pawn,
            'r' => PieceType::Rook,
            'n' => PieceType::Knight,
            'b' => PieceType::Bishop,
            'q' => PieceType::Queen,
            'k' => PieceType::King,
            _ => return None,
        };

        Some(piece_type)
    }
}

#[test]
fn test_piece_from_str() {
    assert_eq!(PieceType::from('P'), Some(PieceType::Pawn));
    assert_eq!(PieceType::from('p'), Some(PieceType::Pawn));

    assert_eq!(PieceType::from('R'), Some(PieceType::Rook));
    assert_eq!(PieceType::from('r'), Some(PieceType::Rook));

    assert_eq!(PieceType::from('N'), Some(PieceType::Knight));
    assert_eq!(PieceType::from('n'), Some(PieceType::Knight));

    assert_eq!(PieceType::from('B'), Some(PieceType::Bishop));
    assert_eq!(PieceType::from('b'), Some(PieceType::Bishop));

    assert_eq!(PieceType::from('Q'), Some(PieceType::Queen));
    assert_eq!(PieceType::from('q'), Some(PieceType::Queen));

    assert_eq!(PieceType::from('K'), Some(PieceType::King));
    assert_eq!(PieceType::from('k'), Some(PieceType::King));

    assert_eq!(PieceType::from('x'), None);
    assert_eq!(PieceType::from('L'), None);
}
