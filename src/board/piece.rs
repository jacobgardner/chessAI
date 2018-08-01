use super::piece_type::PieceType;
use super::player::Player;

#[derive(Debug, PartialEq)]
pub struct Piece {
    pub(super) piece_type: PieceType,
    pub(super) player: Player,
}

impl Piece {
    pub fn from(chr: char) -> Option<Piece> {
        if let Some(piece_type) = PieceType::from(chr) {
            Some(Piece {
                piece_type,
                player: Player::from(chr),
            })
        } else {
            None
        }
    }
}

#[test]
fn test_to_piece_owner() {
    assert_eq!(
        Piece::from('P'),
        Some(Piece {
            piece_type: PieceType::Pawn,
            player: Player::White
        })
    );
    assert_eq!(
        Piece::from('p'),
        Some(Piece {
            piece_type: PieceType::Pawn,
            player: Player::Black
        })
    );

    assert_eq!(
        Piece::from('K'),
        Some(Piece {
            piece_type: PieceType::King,
            player: Player::White
        })
    );
    assert_eq!(
        Piece::from('k'),
        Some(Piece {
            piece_type: PieceType::King,
            player: Player::Black
        })
    );

    assert_eq!(Piece::from('l'), None);
}
