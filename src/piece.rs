pub enum Owner {
    White,
    Black
}

pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

// use self::Owner::*;
// use self::PieceType::*;

pub struct Piece {
    pub owner: Owner,
    pub piece_type: PieceType,
}

impl Piece {
    // pub fn new(owner: Owner, piece_type: PieceType) -> Piece {
    //     Piece {
    //         owner: owner,
    //         piece_type: piece_type
    //     }
    // }
}