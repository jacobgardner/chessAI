use super::MoveGenerator;

use crate::chess::{BitBoard, BitPosition, Board, PieceType};

impl MoveGenerator {
    pub(crate) fn generate_next_bishop_move(
        &mut self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> Option<Board> {
        if self.is_first_move {
            self.available_moves =
                self.available_bishop_moves(current_position, current_position_mask);

            self.is_first_move = false;
        }

        // TODO: This implemenation is literally the same for everything but pawn.  
        //  We can unify these for all the move generators
        if !self.available_moves.is_empty() {
            let next_position = self.available_moves.first_bit_position();
            let next_position_mask = BitBoard::from(next_position);

            let board = self.move_piece(
                PieceType::Bishop,
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

    fn available_bishop_moves(
        &self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> BitBoard {
        BitBoard::empty()
    }
}
