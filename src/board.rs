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
pub enum Piece {
    Pawn = 0,
    Rook = 1,
    Knight = 2,
    Bishop = 3,
    Queen = 4,
    King = 5,
    Empty
}

impl<'a> From<&'a char> for Piece {
    fn from(chr: &char) -> Piece {
        match chr.to_lowercase().next().unwrap_or('x') {
            'p' => Piece::Pawn,
            'r' => Piece::Rook,
            'n' => Piece::Knight,
            'b' => Piece::Bishop,
            'q' => Piece::Queen,
            'k' => Piece::King,
            _ => Piece::Empty,
        }
    }
}

pub fn to_piece_owner(chr: &char) -> Result<(Piece, Player), ()> {
    let piece = Piece::from(chr);

    if piece == Piece::Empty {
        Err(())
    } else {
        Ok((piece, Player::from(chr)))
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
    assert_eq!(to_piece_owner(&'P').unwrap(), (Piece::Pawn, Player::White));
    assert_eq!(to_piece_owner(&'p').unwrap(), (Piece::Pawn, Player::Black));


    assert_eq!(to_piece_owner(&'K').unwrap(), (Piece::King, Player::White));
    assert_eq!(to_piece_owner(&'k').unwrap(), (Piece::King, Player::Black));

    assert_eq!(to_piece_owner(&'l'), Err(()));
}

#[test]
fn test_piece_from_str() {
    assert_eq!(Piece::from(&'P') as u32, Piece::Pawn as u32);
    assert_eq!(Piece::from(&'p') as u32, Piece::Pawn as u32);

    assert_eq!(Piece::from(&'R') as u32, Piece::Rook as u32);
    assert_eq!(Piece::from(&'r') as u32, Piece::Rook as u32);

    assert_eq!(Piece::from(&'N') as u32, Piece::Knight as u32);
    assert_eq!(Piece::from(&'n') as u32, Piece::Knight as u32);

    assert_eq!(Piece::from(&'B') as u32, Piece::Bishop as u32);
    assert_eq!(Piece::from(&'b') as u32, Piece::Bishop as u32);

    assert_eq!(Piece::from(&'Q') as u32, Piece::Queen as u32);
    assert_eq!(Piece::from(&'q') as u32, Piece::Queen as u32);

    assert_eq!(Piece::from(&'K') as u32, Piece::King as u32);
    assert_eq!(Piece::from(&'k') as u32, Piece::King as u32);

    assert_eq!(Piece::from(&'x') as u32, Piece::Empty as u32);
    assert_eq!(Piece::from(&'L') as u32, Piece::Empty as u32);
}

const PIECE_COUNT: usize = 6;
const PLAYER_COUNT: usize = 2;

pub struct Board {
    boards: [u64; PIECE_COUNT + PLAYER_COUNT],
}

impl<'a> From<&'a str> for Board {
    fn from(board: &str) -> Board {
        Board { boards: [0; PIECE_COUNT + PLAYER_COUNT] }
    }
}

impl Display for Board {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        Ok(())
    }
}
