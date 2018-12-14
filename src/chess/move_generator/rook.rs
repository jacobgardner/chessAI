use super::MoveGenerator;

use crate::chess::BitBoard;
use crate::chess::BitPosition;
use crate::chess::Board;

impl MoveGenerator {
    pub(crate) fn generate_next_rook_move(
        &mut self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> Option<Board> {
        if self.is_first_move {

        }

        None
    }
}
