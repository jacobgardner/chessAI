use super::{MoveGenerator, PieceMoveGenerator};

use crate::chess::{BitBoard, BitPosition, PieceType};
use super::{RookMoveGen, BishopMoveGen};

pub(in super) struct QueenMoveGen;

impl PieceMoveGenerator for QueenMoveGen {
    fn piece_type(&self) -> PieceType {
        PieceType::Queen
    }

    fn find_available_moves(&self, move_gen: &MoveGenerator, current_position: BitPosition, current_position_mask: BitBoard) -> BitBoard {
        BishopMoveGen.find_available_moves(move_gen, current_position, current_position_mask)
            .join(RookMoveGen.find_available_moves(move_gen, current_position, current_position_mask))
    }
}
