use super::MoveGenerator;
use std::cmp::min;

use crate::chess::{BitBoard, BitPosition, Board, PieceType, RankFile};

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

        if self.available_moves.is_empty() {
            return None;
        }

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

        Some(board)
    }

    fn available_bishop_moves(
        &self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> BitBoard {
        // NOTE: We would likely get much much better performance
        //  by using rotated bitboards instead.
        let rf = RankFile::from(current_position);

        let left = rf.file();
        let bot = rf.rank();
        let right = 7 - left;
        let top = 7 - bot;

        let top_left = min(top, left);
        let top_right = min(top, right);
        let bot_left = min(bot, left);
        let bot_right = min(bot, right);

        let everything_else = self.all_pieces - current_position_mask;

        let mut top_left_positions = current_position_mask.clone();
        for _ in 0..top_left {
            top_left_positions |= top_left_positions.shift_up(1).shift_left(1);

            if !top_left_positions.intersect(everything_else).is_empty() {
                break;
            }
        }

        let mut top_right_positions = current_position_mask.clone();
        for _ in 0..top_right {
            top_right_positions |= top_right_positions.shift_up(1).shift_right(1);

            if !top_right_positions.intersect(everything_else).is_empty() {
                break;
            }
        }

        let mut bot_left_positions = current_position_mask.clone();
        for _ in 0..bot_left {
            bot_left_positions |= bot_left_positions.shift_down(1).shift_left(1);

            if !bot_left_positions.intersect(everything_else).is_empty() {
                break;
            }
        }

        let mut bot_right_positions = current_position_mask.clone();
        for _ in 0..bot_right {
            bot_right_positions |= bot_right_positions.shift_down(1).shift_right(1);

            if !bot_right_positions.intersect(everything_else).is_empty() {
                break;
            }
        }

        top_left_positions
            .join(top_right_positions)
            .join(bot_left_positions)
            .join(bot_right_positions)
            - self.player_mask

        // let rotated = current_position_mask.rotate_45cw();

        // let rotated_bit_position = rotated.first_bit_position();
        // // let

        // println!("Rotated:\n{:?}", rotated);

        // // if file > rank, above the diagonal
        // // if above diagonal: 8 - (file - rank) = row #
        // // if below diagonal: rank - file = row #
        // // if file == rank => row = 0
        // //
        // //  if above the diagonal, indicies are from
        // //      8 - row #  to 8
        // //  if below the diagonal, indicies are from
        // //      0 to 8 - row #

        // BitBoard::empty()
    }
}
