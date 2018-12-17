use super::MoveGenerator;

use crate::chess::{BitBoard, BitPosition, Board, PieceType, RankFile};

impl MoveGenerator {
    pub(crate) fn generate_next_rook_move(
        &mut self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> Option<Board> {
        if self.is_first_move {
            self.available_moves = self.available_rook_moves(current_position);
            println!("{:?}", self.available_moves);
            self.is_first_move = false;
        }

        if !self.available_moves.is_empty() {
            let new_move = self.available_moves.first_bit_position();
            let next_position_mask = BitBoard::from(new_move);

            let board = self.move_piece(
                PieceType::Rook,
                current_position,
                current_position_mask,
                new_move,
                next_position_mask,
                next_position_mask.intersect(self.enemy_mask),
            );

            self.available_moves -= next_position_mask;

            return Some(board);
        }

        None
    }

    fn available_rook_moves(&self, current_position: BitPosition) -> BitBoard {
        let slides = self.all_pieces.horizontal_slides(current_position);
        let rf = RankFile::from(current_position);
        println!("{:?}", rf);

        // slides.join(
        //     self.all_pieces
        //         .rotate_90cw()
        //         .horizontal_slides(current_position.rotate_90cw())
        //         .rotate_90ccw(),
        // ) - self.player_mask

        slides
    }
}
