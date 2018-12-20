use super::MoveGenerator;

use crate::chess::{BitBoard, BitPosition, PieceType};

impl MoveGenerator {
    fn piece_type(&self) -> PieceType {
        PieceType::Queen
    }

    pub(super) fn find_queen_moves(
        &self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> BitBoard {
        self.find_bishop_moves(current_position, current_position_mask)
            .join(self.find_rook_moves(current_position, current_position_mask))
    }
}
