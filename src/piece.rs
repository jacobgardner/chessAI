#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Owner {
    White,
    Black,
}

use self::Owner::*;

impl Owner {
    pub fn flip(&self) -> Self {
        match *self {
            White => Black,
            Black => White,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Piece {
    pub owner: Owner,
    pub piece_type: PieceType,
    pub has_moved: bool,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

use self::PieceType::*;

// impl Default for Piece {
//     fn default() -> Piece {
//         Piece { owner: Black, piece_type: Pawn, has_moved: false }
//     }
// }

impl Piece {
    pub fn new(piece_type: PieceType, owner: Owner) -> Self {
        Piece {owner: owner, piece_type: piece_type, has_moved: false}
    }

    pub fn value(&self) -> f64 {
        match self.piece_type {
            Pawn => 1f64,
            Rook => 3f64,
            Bishop => 3f64,
            Knight => 5f64,
            Queen => 12f64,
            King => 1000f64,
        }
    }
}
