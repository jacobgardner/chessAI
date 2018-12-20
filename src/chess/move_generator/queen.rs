use super::MoveGenerator;

use crate::chess::{BitBoard, BitPosition, Board, PieceType};

impl MoveGenerator {
    pub(crate) fn generate_next_queen_move(
        &mut self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> Option<Board> {
        if self.is_first_move {
            self.available_moves = self.available_queen_moves(current_position, current_position_mask);
            self.is_first_move = false;
        }

        if !self.available_moves.is_empty() {
            let next_position = self.available_moves.first_bit_position();
            let next_position_mask = BitBoard::from(next_position);

            let board = self.move_piece(
                PieceType::Queen,
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

    fn available_queen_moves(&self, current_position: BitPosition, current_position_mask: BitBoard) -> BitBoard {
        self.available_rook_moves(current_position, current_position_mask).join(self.available_bishop_moves(current_position, current_position_mask))
    }
}
