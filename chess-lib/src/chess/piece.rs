use crate::chess::{PieceType, Player};

#[derive(Debug, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub player: Player,
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

    pub fn to_char(&self) -> char {
        let (white_piece, black_piece) = match self.piece_type {
            PieceType::Pawn => ('♙', '♟'),
            PieceType::Rook => ('♖', '♜'),
            PieceType::Bishop => ('♗', '♝'),
            PieceType::Knight => ('♘', '♞'),
            PieceType::King => ('♔', '♚'),
            PieceType::Queen => ('♕', '♛'),
        };

        if self.player == Player::White {
            white_piece
        } else {
            black_piece
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
