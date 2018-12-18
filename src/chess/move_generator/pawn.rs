use super::MoveGenerator;

use crate::chess::bitboard::{FILE_1, FILE_2, FILE_7, FILE_8};
use crate::chess::BitBoard;
use crate::chess::BitPosition;
use crate::chess::Board;
use crate::chess::PieceType;
use crate::chess::Player;

impl MoveGenerator {
    pub(crate) fn generate_next_pawn_move(
        &mut self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> Option<Board> {
        let mut en_passant_mask = BitBoard::empty();

        if self.is_first_move {
            self.available_moves =
                self.available_pawn_moves(current_position, current_position_mask);
            self.available_captures = self.pawn_captures(current_position);
            en_passant_mask = self.check_en_passant();

            self.is_first_move = false;
        }

        if !en_passant_mask.is_empty() {
            if let Some(board) = self.generate_en_passant_board(
                current_position,
                current_position_mask,
                en_passant_mask,
            ) {
                return Some(board);
            }
        }

        if !self.available_moves.is_empty() {
            let (board, available_moves) = self.generate_pawn_board(
                current_position,
                current_position_mask,
                self.available_moves,
                false,
            );
            self.available_moves = available_moves;
            return Some(board);
        }

        if !self.available_captures.is_empty() {
            let (board, available_captures) = self.generate_pawn_board(
                current_position,
                current_position_mask,
                self.available_captures,
                true,
            );
            self.available_captures = available_captures;
            return Some(board);
        }

        None
    }

    fn generate_pawn_board(
        &self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
        available_moves: BitBoard,
        is_capture: bool,
    ) -> (Board, BitBoard) {
        let new_move = available_moves.first_bit_position();
        let next_position_mask = BitBoard::from(new_move);

        let board = self.move_piece(
            PieceType::Pawn,
            current_position,
            current_position_mask,
            new_move,
            next_position_mask,
            if is_capture {
                next_position_mask
            } else {
                BitBoard::empty()
            },
        );

        (board, available_moves - next_position_mask)
    }

    fn generate_en_passant_board(
        &self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
        en_passant_mask: BitBoard,
    ) -> Option<Board> {
        debug_assert!(
            en_passant_mask.count_pieces() == 1,
            "There can only be a single en passant capture per piece possible."
        );

        let next_position_mask = en_passant_mask
            .shift_down()
            .join(en_passant_mask.shift_up())
            .intersect(self.diagonals(current_position_mask.first_bit_position()));

        debug_assert!(
            next_position_mask != en_passant_mask,
            "The en passant capture mask should be the mask of the \
             pawn being captured. Not of the position being moved to."
        );

        if !next_position_mask.is_empty() {
            let new_move = next_position_mask.first_bit_position();

            let board = self.move_piece(
                PieceType::Pawn,
                current_position,
                current_position_mask,
                new_move,
                next_position_mask,
                en_passant_mask,
            );

            Some(board)
        } else {
            None
        }
    }

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

    // // NOTE: The ONLY case where capture_mask != next_position_mask is en passant
    // //  We may be able to optimize stuff so we don't need an extra 64 bits for this
    // //  function
    // // NOTE: We may be able to make this mostly work for other pieces as well
    // fn move_pawn(
    //     &self,
    //     current_position: BitPosition,
    //     current_position_mask: BitBoard,
    //     next_position: BitPosition,
    //     next_position_mask: BitBoard,
    //     capture_mask: BitBoard,
    // ) -> Board {
    //     let mut board = self.root_board.clone();

    //     let pawn_index = PieceType::Pawn as usize;
    //     let player_index = self.player as usize;

    //     // Remove current position from pawn and current player bitboards
    //     board.pieces[pawn_index] -= current_position_mask;
    //     board.players[player_index] -= current_position_mask;

    //     if capture_mask.is_empty() {
    //         self.slide_move_sanity_check(&board, next_position_mask);
    //     } else {
    //         self.capture_sanity_check(&board, capture_mask);
    //         self.remove_piece(&mut board, capture_mask);
    //     }

    //     board.players[player_index] += next_position_mask;

    //     // NOTE: When making this function generic we'll need a PAWN check
    //     let next_piece = if next_position_mask.intersect(ENDS).is_empty() {
    //         PieceType::Pawn
    //     } else {
    //         PieceType::Queen
    //     };

    //     board.pieces[next_piece as usize] += next_position_mask;

    //     board.prev_move = Some(Move {
    //         piece_type: PieceType::Pawn,
    //         from: current_position.into(),
    //         to: next_position.into(),
    //     });

    //     debug_assert!(self.root_board.prev_move != board.prev_move);

    //     board
    // }

    // fn remove_piece(&self, board: &mut Board, next_position_mask: BitBoard) {
    //     for i in 0..PIECE_COUNT {
    //         board.pieces[i] -= next_position_mask;
    //     }

    //     // And the previous player
    //     board.players[1 - (self.player as usize)] -= next_position_mask;
    // }

    fn pawn_direction(&self) -> i32 {
        match self.player {
            Player::White => 1,
            Player::Black => -1,
        }
    }

    fn check_for_single_move(&self, pawn_position: BitPosition) -> BitBoard {
        covered_by!("Pawn::available_single_moves -> White");
        covered_by!("Pawn::available_single_moves -> Black");

        let possible_single_move: BitBoard = pawn_position.shift(0, self.pawn_direction()).into();

        // Remove any collisions with existing pieces
        possible_single_move - self.all_pieces
    }

    fn check_for_double_move(
        &self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> BitBoard {
        covered_by!("Pawn::available_double_moves -> White");
        covered_by!("Pawn::available_double_moves -> Black");

        let (direction, starting_row) = match self.player {
            Player::White => (2, FILE_2),
            Player::Black => (-2, FILE_7),
        };

        let is_in_starting_row = !current_position_mask.intersect(starting_row).is_empty();

        let possible_double_move = if is_in_starting_row {
            BitBoard::from(current_position.shift(0, direction))
        } else {
            BitBoard::empty()
        };

        // Remove any collisions with existing pieces
        possible_double_move - self.all_pieces
    }

    fn available_pawn_moves(
        &self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> BitBoard {
        debug_assert!(
            current_position_mask
                .intersect(FILE_1.join(FILE_8))
                .is_empty(),
            "Pawn Invariant Invalidation: Pawn must never appear in the first or last row"
        );

        let mut moves = self.check_for_single_move(current_position);

        // If we couldn't single-move, we definitely can't double move
        if !moves.is_empty() {
            moves = moves.join(self.check_for_double_move(current_position, current_position_mask));
        }

        moves
    }

    // NOTE: Functions like this really only need the position and the player
    //  there's no real reason it needs to be apart of MoveGeneration
    fn diagonals(&self, current_position: BitPosition) -> BitBoard {
        covered_by!("Pawn::diagonals -> White");
        covered_by!("Pawn::diagonals -> Black");
        let direction = self.pawn_direction();

        let left_diagonal = if !current_position.is_leftmost() {
            BitBoard::from(current_position.shift(-1, direction))
        } else {
            BitBoard::empty()
        };

        let right_diagonal = if !current_position.is_rightmost() {
            BitBoard::from(current_position.shift(1, direction))
        } else {
            BitBoard::empty()
        };

        left_diagonal.join(right_diagonal)
    }

    fn pawn_captures(&self, current_position: BitPosition) -> BitBoard {
        covered_by!("Pawn::captures -> White");
        covered_by!("Pawn::captures -> Black");

        debug_assert!(
            !(self.root_board.prev_move.is_none() && !self.enemy_mask.is_empty()),
            "Somehow we can capture on the first move?"
        );

        self.enemy_mask.intersect(self.diagonals(current_position))
    }

    fn check_en_passant(&self) -> BitBoard {
        // The previous move MUST be a pawn double move
        if let Some(prev_move) = self.root_board.prev_move.as_ref() {
            if prev_move.piece_type == PieceType::Pawn {
                if prev_move.from.file() == 1 && prev_move.to.file() == 3 {
                    covered_by!("Pawn::en_passant -> Black");
                    return prev_move.to.into();
                } else if prev_move.from.file() == 6 && prev_move.to.file() == 4 {
                    covered_by!("Pawn::en_passant -> White");
                    return prev_move.to.into();
                }
            }
        };

        BitBoard::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::RankFile;
    use crate::fixtures::*;

    #[test]
    fn test_en_passant_white() {
        covers!("Pawn::en_passant -> White");
        let mut board = Board::from(WHITE_PAWN_TEST).unwrap();
        board.prev_move = Some(WHITE_EN_PASSANT);
        let generator = MoveGenerator::new(board, Player::White);

        assert_eq!(generator.check_en_passant(), RankFile::F5.into());
    }

    #[test]
    fn test_en_passant_black() {
        covers!("Pawn::en_passant -> Black");
        let mut board = Board::from(BLACK_PAWN_TEST).unwrap();
        board.prev_move = Some(BLACK_EN_PASSANT);
        let generator = MoveGenerator::new(board, Player::Black);

        assert_eq!(generator.check_en_passant(), RankFile::D4.into());
    }

    #[test]
    fn test_single_moves_white() {
        covers!("Pawn::available_single_moves -> White");

        let board = Board::from(WHITE_PAWN_TEST).unwrap();
        let generator = MoveGenerator::new(board, Player::White);

        assert_eq!(
            generator.check_for_single_move(RankFile::B2.into()),
            BitBoard::from(RankFile::B3)
        );

        assert_eq!(
            generator.check_for_single_move(RankFile::C3.into()),
            BitBoard::from(RankFile::C4)
        );

        assert_eq!(
            generator.check_for_single_move(RankFile::H2.into()),
            BitBoard::empty()
        );
    }

    #[test]
    fn test_double_moves_white() {
        covers!("Pawn::available_double_moves -> White");

        let board = Board::from(WHITE_PAWN_TEST).unwrap();
        let generator = MoveGenerator::new(board, Player::White);

        assert_eq!(
            generator.check_for_double_move(RankFile::F2.into(), RankFile::F2.into()),
            BitBoard::from(RankFile::F4)
        );

        assert_eq!(
            generator.check_for_double_move(RankFile::C3.into(), RankFile::C3.into()),
            BitBoard::empty()
        );

        assert_eq!(
            generator.check_for_double_move(RankFile::B2.into(), RankFile::B2.into()),
            BitBoard::empty()
        );
    }

    #[test]
    fn test_single_moves_black() {
        covers!("Pawn::available_single_moves -> Black");

        let board = Board::from(BLACK_PAWN_TEST).unwrap();
        let generator = MoveGenerator::new(board, Player::Black);

        assert_eq!(
            generator.check_for_single_move(RankFile::E2.into()),
            BitBoard::from(RankFile::E1)
        );

        assert_eq!(
            generator.check_for_single_move(RankFile::G7.into()),
            BitBoard::from(RankFile::G6)
        );

        assert_eq!(
            generator.check_for_single_move(RankFile::E3.into()),
            BitBoard::empty()
        );
    }

    #[test]
    fn test_double_moves_black() {
        covers!("Pawn::available_double_moves -> Black");

        let board = Board::from(BLACK_PAWN_TEST).unwrap();
        let generator = MoveGenerator::new(board, Player::Black);

        assert_eq!(
            generator.check_for_double_move(RankFile::G7.into(), RankFile::G7.into()),
            BitBoard::from(RankFile::G5)
        );

        assert_eq!(
            generator.check_for_double_move(RankFile::A7.into(), RankFile::A7.into()),
            BitBoard::empty()
        );

        assert_eq!(
            generator.check_for_double_move(RankFile::C5.into(), RankFile::C5.into()),
            BitBoard::empty()
        );
    }

    #[test]
    fn test_diagonals_white() {
        covers!("Pawn::diagonals -> White");
        let board = Board::from(WHITE_PAWN_TEST).unwrap();
        let generator = MoveGenerator::new(board, Player::White);

        let expected = BitBoard::from(RankFile::G3).join(RankFile::E3.into());
        assert_eq!(generator.diagonals(RankFile::F2.into()), expected);

        let expected = BitBoard::from(RankFile::G3);
        assert_eq!(generator.diagonals(RankFile::H2.into()), expected);

        let expected = BitBoard::from(RankFile::B6);
        assert_eq!(generator.diagonals(RankFile::A5.into()), expected);
    }

    #[test]
    fn test_diagonals_black() {
        covers!("Pawn::diagonals -> Black");
        let board = Board::from(BLACK_PAWN_TEST).unwrap();
        let generator = MoveGenerator::new(board, Player::Black);

        let expected = BitBoard::from(RankFile::G1).join(RankFile::E1.into());
        assert_eq!(generator.diagonals(RankFile::F2.into()), expected);

        let expected = BitBoard::from(RankFile::G1);
        assert_eq!(generator.diagonals(RankFile::H2.into()), expected);

        let expected = BitBoard::from(RankFile::B4);
        assert_eq!(generator.diagonals(RankFile::A5.into()), expected);
    }

    #[test]
    fn test_captures_white() {
        covers!("Pawn::captures -> White");
        let mut board = Board::from(WHITE_PAWN_TEST).unwrap();
        board.prev_move = Some(WHITE_EN_PASSANT);
        let generator = MoveGenerator::new(board, Player::White);

        assert_eq!(
            generator.pawn_captures(RankFile::F2.into()),
            BitBoard::empty()
        );

        let expected = BitBoard::from(RankFile::A3);
        assert_eq!(generator.pawn_captures(RankFile::B2.into()), expected);

        let expected = BitBoard::from(RankFile::B4).join(RankFile::D4.into());
        assert_eq!(generator.pawn_captures(RankFile::C3.into()), expected);
    }

    #[test]
    fn test_captures_black() {
        covers!("Pawn::captures -> Black");
        let mut board = Board::from(BLACK_PAWN_TEST).unwrap();
        board.prev_move = Some(BLACK_EN_PASSANT);
        let generator = MoveGenerator::new(board, Player::Black);

        assert_eq!(
            generator.pawn_captures(RankFile::A7.into()),
            BitBoard::empty()
        );

        let expected = BitBoard::from(RankFile::D1);
        assert_eq!(generator.pawn_captures(RankFile::E2.into()), expected);

        let expected = BitBoard::from(RankFile::H6).join(RankFile::F6.into());
        assert_eq!(generator.pawn_captures(RankFile::G7.into()), expected);
    }
}
