use board::ChessBoard;
use position::Position;
use moves::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Owner {
    White,
    Black,
}

use self::PieceType::*;
use self::Owner::*;


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

// impl Default for Piece {
//     fn default() -> Piece {
//         Piece { owner: Black, piece_type: Pawn, has_moved: false }
//     }
// }

impl Piece {
    pub fn new(piece_type: PieceType, owner: Owner) -> Self {
        Piece {owner: owner, piece_type: piece_type, has_moved: false}
    }
}
