use super::MoveGenerator;

use crate::chess::{BitBoard, BitPosition};

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
}
