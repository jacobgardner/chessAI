use super::MoveGenerator;
use std::cmp::min;

use crate::chess::{BitBoard, BitPosition, RankFile};

impl MoveGenerator {
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
}
