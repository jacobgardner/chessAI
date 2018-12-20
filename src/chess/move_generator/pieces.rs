use super::MoveGenerator;
use std::cmp::min;

use crate::chess::{BitBoard, BitPosition, RankFile};

impl MoveGenerator {
    pub(super) fn find_rook_moves(&self, current_position: BitPosition, _: BitBoard) -> BitBoard {
        let slides = self.all_pieces.horizontal_slides(current_position);

        slides.join(
            self.all_pieces
                .rotate_90cw()
                .horizontal_slides(current_position.rotate_90cw())
                .rotate_90ccw(),
        ) - self.player_mask
    }

    pub(super) fn find_queen_moves(
        &self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> BitBoard {
        self.find_bishop_moves(current_position, current_position_mask)
            .join(self.find_rook_moves(current_position, current_position_mask))
    }

    pub(super) fn find_bishop_moves(
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

        [
            (top_left, 1, -1),
            (top_right, 1, 1),
            (bot_left, -1, -1),
            (bot_right, -1, 1),
        ]
        .iter()
        .fold(BitBoard::empty(), |acc, &(count, rank, file)| {
            let mut board = current_position_mask;
            for _ in 0..count {
                board |= board.shift(rank, file);

                // Stop this diagonal at the first collision
                if !board.intersect(everything_else).is_empty() {
                    break;
                }
            }

            acc.join(board)
        }) - self.player_mask
    }

    pub(super) fn find_knight_moves(&self, current_position: BitPosition, current_position_mask: BitBoard) -> BitBoard {
        // NOTE: We could make this do all knight moves in parallel if we wanted
        // NOTE: My dad could beat up your dad if he wanted
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

    pub (super) fn find_king_moves(&self, _: BitPosition, current_position_mask: BitBoard) -> BitBoard {
        let mut mask = current_position_mask.join(current_position_mask.shift_left(1)).join(current_position_mask.shift_right(1));

        mask |= mask.shift_up(1).join(mask.shift_down(1));

        mask - self.player_mask
    }
}
