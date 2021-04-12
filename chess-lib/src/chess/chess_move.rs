use std::fmt;

use crate::chess::{PieceType, RankFile};

#[derive(PartialEq, Debug, Clone)]
pub enum MoveType {
    Standard,
    Castling { is_queenside: bool },
    Promotion { promoted_to: PieceType },
    EnPassant,
}

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
    pub move_type: MoveType,
    pub is_capture: bool,
    pub threatens_king: bool,
}

impl Default for Move {
    fn default() -> Self {
        Move {
            piece_type: PieceType::Pawn,
            from: RankFile::A1,
            to: RankFile::H8,
            move_type: MoveType::Standard,
            is_capture: false,
            threatens_king: false,
        }
    }
}

/// This is not true Standard Algebraic Notation
///   SAN typically only provides the full rank and file of departure,
///   for example, if it is ambiguous with another piece's move. There are
///   other thigns that are also typically captured such as checkmates that
///   we do not yet cover here.
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let MoveType::Castling { is_queenside } = self.move_type {
            if is_queenside {
                return write!(f, "O-O-O");
            } else {
                return write!(f, "O-O");
            }
        }

        write!(
            f,
            "{}{}{}{}{}{}{}",
            self.piece_type.to_char(),
            format!("{:?}", self.from).to_lowercase(),
            if self.is_capture { "x" } else { "" },
            format!("{:?}", self.to).to_lowercase(),
            if self.move_type == MoveType::EnPassant {
                "e.p."
            } else {
                ""
            },
            if let MoveType::Promotion { promoted_to } = self.move_type {
                let mut s = "=".to_owned();
                s.push(promoted_to.to_char());

                s
            } else {
                "".to_owned()
            },
            if self.threatens_king {
                "+"
            } else {
                ""
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_san() {
        assert_eq!(format!("{}", Move {
            piece_type: PieceType::Pawn,
            from: RankFile::A7,
            to: RankFile::A8,
            move_type: MoveType::Promotion { promoted_to: PieceType::Queen },
            is_capture: false,
            threatens_king: false
        }), "Pa7a8=Q");

        assert_eq!(format!("{}", Move {
            piece_type: PieceType::Pawn,
            from: RankFile::A7,
            to: RankFile::A8,
            move_type: MoveType::Promotion { promoted_to: PieceType::Queen },
            is_capture: true,
            threatens_king: true 
        }), "Pa7xa8=Q+");
    }
}
