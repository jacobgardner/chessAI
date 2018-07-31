use std::fmt::{Display, Error, Formatter};

enum Player {
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

#[derive(Debug)]
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

const PIECE_COUNT: u32 = 6;

pub struct Board {
    boards: [u64; 12],
}

impl Board {
    pub fn default() -> Self {
        let mut boards = [0u64; 12];

        Board { boards: boards }
    }

    // pub fn from_string(board_str: &str) -> Self {
    //     let mut boards = [0u64; 12];

    //     for (idx, ch) in board_str.enumerate() {
    //         let x = idx % 8;
    //         let y = idx / 8;

    //         // let boardIndex =

    //    }
    // }
}

impl<'a> From<&'a str> for Board {
    fn from(board: &str) -> Board {
        Board { boards: [0; 12] }
    }
}

impl Display for Board {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        Ok(())
    }
}
