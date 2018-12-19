use super::MoveGenerator;

use crate::chess::{BitBoard, BitPosition, Board, PieceType};

impl MoveGenerator {
    pub(crate) fn generate_next_knight_move(
        &mut self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> Option<Board> {
        // NOTE: Think about "lazy" implemenation
        if self.is_first_move {
            self.available_moves = self.available_knight_moves(current_position);

            self.is_first_move = false;
        }

        if self.available_moves.is_empty() {
            return None;
        }

        let next_position = self.available_moves.first_bit_position();
        let next_position_mask = BitBoard::from(next_position);

        let board = self.move_piece(
            PieceType::Knight,
            current_position,
            current_position_mask,
            next_position,
            next_position_mask,
            next_position_mask.intersect(self.enemy_mask),
        );

        self.available_moves -= next_position_mask;

        Some(board)
    }

    fn available_knight_moves(&self, current_position: BitPosition) -> BitBoard {
        // NOTE: We could make this do all knight moves in parallel if we wanted
        let current_position_mask = BitBoard::from(current_position);

        let single_horiz_shift = current_position_mask
            .shift_left(1)
            .join(current_position_mask.shift_right(1));

        let double_horiz_shift = current_position_mask
            .shift_left(2)
            .join(current_position_mask.shift_right(2));

        // two over, one up
        let single_vert_shift = double_horiz_shift 
            .shift_up(1)
            .join(double_horiz_shift.shift_down(1));

        // one over, two up
        let double_vert_shift = single_horiz_shift
            .shift_up(2)
            .join(single_horiz_shift.shift_down(2));

        single_vert_shift.join(double_vert_shift) - self.player_mask
    }
}
