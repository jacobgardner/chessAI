use super::{MoveGenerator, PieceMoveGenerator};

use crate::chess::{BitBoard, BitPosition, PieceType};

pub(in super) struct RookMoveGen;

impl PieceMoveGenerator for RookMoveGen {
    fn piece_type(&self) -> PieceType {
        PieceType::Rook
    }

    fn find_available_moves(&self, move_gen: &MoveGenerator, current_position: BitPosition, _: BitBoard) -> BitBoard {
        let slides = move_gen.all_pieces.horizontal_slides(current_position);

        slides.join(
            move_gen.all_pieces
                .rotate_90cw()
                .horizontal_slides(current_position.rotate_90cw())
                .rotate_90ccw(),
        ) - move_gen.player_mask
    }
}
