use super::MoveGenerator;

use crate::chess::{BitBoard, BitPosition};

impl MoveGenerator {
    pub(super) fn find_knight_moves(&self, current_position: BitPosition, _: BitBoard) -> BitBoard {
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
