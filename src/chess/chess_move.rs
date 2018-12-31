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
            "{}{:?}{}{:?}{}{}",
            self.piece_type.to_char(),
            self.from,
            if self.is_capture { "x" } else { "" },
            self.to,
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
            }
        )
    }
}
