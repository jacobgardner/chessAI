use bitboard::ROW_1;
use std::fmt;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, PartialEq)]
pub enum Player {
    Black = 0,
    White = 1,
}

impl<'a> From<&'a char> for Player {
    fn from(chr: &'a char) -> Player {
        if chr.is_lowercase() {
            Player::Black
        } else {
            Player::White
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PieceType {
    Pawn = 0,
    Rook = 1,
    Knight = 2,
    Bishop = 3,
    Queen = 4,
    King = 5,
}

impl PieceType {
    fn from(chr: &char) -> Option<PieceType> {
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

#[derive(Debug, PartialEq)]
pub struct Piece {
    piece_type: PieceType,
    player: Player,
}

impl Piece {
    pub fn from(chr: &char) -> Option<Piece> {
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

const PIECE_COUNT: usize = 6;
const PLAYER_COUNT: usize = 2;
const BOARD_COUNT: usize = PIECE_COUNT + PLAYER_COUNT;

pub struct Board {
    pieces: [u64; PIECE_COUNT],
    players: [u64; PLAYER_COUNT],
}

impl Board {}

impl<'a> From<&'a str> for Board {
    fn from(board: &str) -> Board {
        Board {
            pieces: [0; PIECE_COUNT],
            players: [0; PLAYER_COUNT],
        }
    }
}

impl Display for Board {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let mut board = String::with_capacity(128);

        for i in 0..8 {
            ROW_1;
            board += "\n";
        }

        write!(formatter, "{}", board)
    }
}

#[test]
fn test_player_from_str() {
    assert_eq!(Player::from(&'Q'), Player::White);
    assert_eq!(Player::from(&'K'), Player::White);
    assert_eq!(Player::from(&'N'), Player::White);
    assert_eq!(Player::from(&'B'), Player::White);
    assert_eq!(Player::from(&'P'), Player::White);
    assert_eq!(Player::from(&'R'), Player::White);
    assert_eq!(Player::from(&'X'), Player::White);

    assert_eq!(Player::from(&'q'), Player::Black);
    assert_eq!(Player::from(&'k'), Player::Black);
    assert_eq!(Player::from(&'n'), Player::Black);
    assert_eq!(Player::from(&'b'), Player::Black);
    assert_eq!(Player::from(&'p'), Player::Black);
    assert_eq!(Player::from(&'r'), Player::Black);
    assert_eq!(Player::from(&'x'), Player::Black);
}

#[test]
fn test_to_piece_owner() {
    assert_eq!(
        Piece::from(&'P'),
        Some(Piece {
            piece_type: PieceType::Pawn,
            player: Player::White
        })
    );
    assert_eq!(
        Piece::from(&'p'),
        Some(Piece {
            piece_type: PieceType::Pawn,
            player: Player::Black
        })
    );

    assert_eq!(
        Piece::from(&'K'),
        Some(Piece {
            piece_type: PieceType::King,
            player: Player::White
        })
    );
    assert_eq!(
        Piece::from(&'k'),
        Some(Piece {
            piece_type: PieceType::King,
            player: Player::Black
        })
    );

    assert_eq!(Piece::from(&'l'), None);
}

#[test]
fn test_piece_from_str() {
    assert_eq!(PieceType::from(&'P'), Some(PieceType::Pawn));
    assert_eq!(PieceType::from(&'p'), Some(PieceType::Pawn));

    assert_eq!(PieceType::from(&'R'), Some(PieceType::Rook));
    assert_eq!(PieceType::from(&'r'), Some(PieceType::Rook));

    assert_eq!(PieceType::from(&'N'), Some(PieceType::Knight));
    assert_eq!(PieceType::from(&'n'), Some(PieceType::Knight));

    assert_eq!(PieceType::from(&'B'), Some(PieceType::Bishop));
    assert_eq!(PieceType::from(&'b'), Some(PieceType::Bishop));

    assert_eq!(PieceType::from(&'Q'), Some(PieceType::Queen));
    assert_eq!(PieceType::from(&'q'), Some(PieceType::Queen));

    assert_eq!(PieceType::from(&'K'), Some(PieceType::King));
    assert_eq!(PieceType::from(&'k'), Some(PieceType::King));

    assert_eq!(PieceType::from(&'x'), None);
    assert_eq!(PieceType::from(&'L'), None);
}
