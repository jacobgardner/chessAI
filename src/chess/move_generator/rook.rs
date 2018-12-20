use super::MoveGenerator;

use crate::chess::{BitBoard, BitPosition, Board, PieceType};

impl MoveGenerator {
    pub(crate) fn generate_next_rook_move(
        &mut self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> Option<Board> {

        // TODO: rename to indicate this is the first time we're analyzing this piece
        if self.is_first_move {
            self.available_moves = self.available_rook_moves(current_position, current_position_mask);
            self.is_first_move = false;
        }

        if !self.available_moves.is_empty() {
            let next_position = self.available_moves.first_bit_position();
            let next_position_mask = BitBoard::from(next_position);

            let board = self.move_piece(
                PieceType::Rook,
                current_position,
                current_position_mask,
                next_position,
                next_position_mask,
                next_position_mask.intersect(self.enemy_mask),
            );

            self.available_moves -= next_position_mask;

            return Some(board);
        }

        None
    }

    pub(super) fn available_rook_moves(&self, current_position: BitPosition, _: BitBoard) -> BitBoard {
        let slides = self.all_pieces.horizontal_slides(current_position);

        slides.join(
            self.all_pieces
                .rotate_90cw()
                .horizontal_slides(current_position.rotate_90cw())
                .rotate_90ccw(),
        ) - self.player_mask
    }
}
