use super::MoveGenerator;

use super::{BitBoard, Board};

impl MoveGenerator {
    fn slide_move_sanity_check(&self, board: &Board, next_position_mask: BitBoard) {
        if cfg!(debug_assertions) {
            debug_assert!(
                board.players[self.player as usize]
                    .intersect(next_position_mask)
                    .is_empty(),
                "Move Invariant Invalidated: Non-capture move made on space occupied by self"
            );

            debug_assert!(
                board.players[self.player as usize]
                    .intersect(next_position_mask)
                    .is_empty(),
                "Move Invariant Invalidated: Non-capture move made on space occupied by opponent"
            );

            // TODO: We'll want additional sanity checks for whether the pieces we're moving through
            //  are empty as well.
        }
    }

    fn capture_sanity_check(&self, board: &Board, capture_mask: BitBoard) {
        if cfg!(debug_assertions) {
            debug_assert!(
                !board.players[1 - self.player as usize].intersect(capture_mask).is_empty(),
                format!("Capture Invariant Invalidated: Capture move made on space not-occupied by opponent:\n{}\n{:?}", board, capture_mask)
            );

            // TODO: We'll want the same sanity check above for moving through spaces.  Should be
            //  OK to delay this until we get to a non-pawn piece
        }
    }
}
