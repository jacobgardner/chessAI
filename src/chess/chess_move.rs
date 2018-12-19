use crate::chess::{PieceType, RankFile};

// NOTE: It is impossible to generate Algebraic Notation (AN) or
//  Portable Game Notation (PGN) with just the piece and from/to
//  Things such as disambiguating moves (e.g. if two rooks can
//  move to the same spot), captures, and en-passant require
//  previous board data as well.
#[derive(PartialEq, Debug, Clone)]
pub struct Move {
    pub piece_type: PieceType,
    pub from: RankFile,
    pub to: RankFile,
}
