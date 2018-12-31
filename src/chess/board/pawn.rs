use super::Board;

use crate::chess::bitboard::{FILE_A, FILE_B, FILE_G, FILE_H};
use crate::chess::{BitBoard, BitPosition, PieceType, Player, MoveType};

impl Board {
    pub fn find_pawn_moves(
        &self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> BitBoard {
        debug_assert!(
            current_position_mask
                .intersect(FILE_A.join(FILE_H))
                .is_empty(),
            "Pawn Invariant Invalidation: Pawn must never appear in the first or last row"
        );

        let mut moves = self.check_for_single_move(current_position);

        // If we couldn't single-move, we definitely can't double move
        if !moves.is_empty() {
            moves = moves.join(self.check_for_double_move(current_position, current_position_mask));
        }

        covered_by!("Pawn::captures -> White");
        covered_by!("Pawn::captures -> Black");

        // LOW: This assert doesn't... make any sense.  Must've screwed something up
        //  in the refactor.
        // debug_assert!(
        //     !(self.prev_move.is_none() && !self.enemy_mask().is_empty()),
        //     "Somehow we can capture on the first move?"
        // );

        moves.join(
            self.enemy_mask()
                .intersect(self.diagonals(current_position)),
        )
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

    fn pawn_direction(&self) -> i32 {
        match self.next_player {
            Player::White => 1,
            Player::Black => -1,
        }
    }

    fn check_for_single_move(&self, pawn_position: BitPosition) -> BitBoard {
        covered_by!("Pawn::available_single_moves -> White");
        covered_by!("Pawn::available_single_moves -> Black");

        let possible_single_move: BitBoard = pawn_position.shift(0, self.pawn_direction()).into();

        // Remove any collisions with existing pieces
        possible_single_move - self.all_pieces()
    }

    fn check_for_double_move(
        &self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> BitBoard {
        covered_by!("Pawn::available_double_moves -> White");
        covered_by!("Pawn::available_double_moves -> Black");

        let (direction, starting_row) = match self.next_player {
            Player::White => (2, FILE_B),
            Player::Black => (-2, FILE_G),
        };

        let is_in_starting_row = !current_position_mask.intersect(starting_row).is_empty();

        let possible_double_move = if is_in_starting_row {
            BitBoard::from(current_position.shift(0, direction))
        } else {
            BitBoard::empty()
        };

        // Remove any collisions with existing pieces
        possible_double_move - self.all_pieces()
    }

    pub fn generate_en_passant_board(
        &self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> Option<Board> {
        let en_passant_mask = self.check_en_passant();

        if en_passant_mask.is_empty() {
            return None;
        }

        debug_assert!(
            en_passant_mask.count_pieces() == 1,
            "There can only be a single en passant capture per piece possible."
        );

        let next_position_mask = en_passant_mask
            .shift_down(1)
            .join(en_passant_mask.shift_up(1))
            .intersect(self.diagonals(current_position));

        debug_assert!(
            next_position_mask != en_passant_mask,
            "The en passant capture mask should be the mask of the \
             pawn being captured. Not of the position being moved to."
        );

        if !next_position_mask.is_empty() {
            let new_move = next_position_mask.first_bit_position();

            let mut board = self.move_piece(
                PieceType::Pawn,
                current_position,
                current_position_mask,
                new_move,
                next_position_mask,
                en_passant_mask,
            );

            board
                .prev_move
                .as_mut()
                .map(|m| m.move_type = MoveType::EnPassant);

            Some(board)
        } else {
            None
        }
    }

    fn available_pawn_moves(
        &self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> BitBoard {
        debug_assert!(
            current_position_mask
                .intersect(FILE_A.join(FILE_H))
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

    pub(super) fn check_en_passant(&self) -> BitBoard {
        // The previous move MUST be a pawn double move
        if let Some(prev_move) = self.prev_move.as_ref() {
            if prev_move.piece_type == PieceType::Pawn {
                if prev_move.from.rank() == 1 && prev_move.to.rank() == 3 {
                    covered_by!("Pawn::en_passant -> Black");
                    return prev_move.to.into();
                } else if prev_move.from.rank() == 6 && prev_move.to.rank() == 4 {
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
        let mut board = Board::from(WHITE_PAWN_TEST, Player::White).unwrap();
        board.prev_move = Some(WHITE_EN_PASSANT);

        assert_eq!(board.check_en_passant(), RankFile::F5.into());
    }

    #[test]
    fn test_en_passant_black() {
        covers!("Pawn::en_passant -> Black");
        let mut board = Board::from(BLACK_PAWN_TEST, Player::Black).unwrap();
        board.prev_move = Some(BLACK_EN_PASSANT);

        assert_eq!(board.check_en_passant(), RankFile::D4.into());
    }

    #[test]
    fn test_single_moves_white() {
        covers!("Pawn::available_single_moves -> White");

        let board = Board::from(WHITE_PAWN_TEST, Player::White).unwrap();

        assert_eq!(
            board.check_for_single_move(RankFile::B2.into()),
            BitBoard::from(RankFile::B3)
        );

        assert_eq!(
            board.check_for_single_move(RankFile::C3.into()),
            BitBoard::from(RankFile::C4)
        );

        assert_eq!(
            board.check_for_single_move(RankFile::H2.into()),
            BitBoard::empty()
        );
    }

    #[test]
    fn test_double_moves_white() {
        covers!("Pawn::available_double_moves -> White");

        let board = Board::from(WHITE_PAWN_TEST, Player::White).unwrap();

        assert_eq!(
            board.check_for_double_move(RankFile::F2.into(), RankFile::F2.into()),
            BitBoard::from(RankFile::F4)
        );

        assert_eq!(
            board.check_for_double_move(RankFile::C3.into(), RankFile::C3.into()),
            BitBoard::empty()
        );

        assert_eq!(
            board.check_for_double_move(RankFile::B2.into(), RankFile::B2.into()),
            BitBoard::empty()
        );
    }

    #[test]
    fn test_single_moves_black() {
        covers!("Pawn::available_single_moves -> Black");

        let board = Board::from(BLACK_PAWN_TEST, Player::Black).unwrap();

        assert_eq!(
            board.check_for_single_move(RankFile::E2.into()),
            BitBoard::from(RankFile::E1)
        );

        assert_eq!(
            board.check_for_single_move(RankFile::G7.into()),
            BitBoard::from(RankFile::G6)
        );

        assert_eq!(
            board.check_for_single_move(RankFile::E3.into()),
            BitBoard::empty()
        );
    }

    #[test]
    fn test_double_moves_black() {
        covers!("Pawn::available_double_moves -> Black");

        let board = Board::from(BLACK_PAWN_TEST, Player::Black).unwrap();

        assert_eq!(
            board.check_for_double_move(RankFile::G7.into(), RankFile::G7.into()),
            BitBoard::from(RankFile::G5)
        );

        assert_eq!(
            board.check_for_double_move(RankFile::A7.into(), RankFile::A7.into()),
            BitBoard::empty()
        );

        assert_eq!(
            board.check_for_double_move(RankFile::C5.into(), RankFile::C5.into()),
            BitBoard::empty()
        );
    }

    #[test]
    fn test_diagonals_white() {
        covers!("Pawn::diagonals -> White");
        let board = Board::from(WHITE_PAWN_TEST, Player::White).unwrap();

        let expected = BitBoard::from(RankFile::G3).join(RankFile::E3.into());
        assert_eq!(board.diagonals(RankFile::F2.into()), expected);

        let expected = BitBoard::from(RankFile::G3);
        assert_eq!(board.diagonals(RankFile::H2.into()), expected);

        let expected = BitBoard::from(RankFile::B6);
        assert_eq!(board.diagonals(RankFile::A5.into()), expected);
    }

    #[test]
    fn test_diagonals_black() {
        covers!("Pawn::diagonals -> Black");
        let board = Board::from(BLACK_PAWN_TEST, Player::Black).unwrap();

        let expected = BitBoard::from(RankFile::G1).join(RankFile::E1.into());
        assert_eq!(board.diagonals(RankFile::F2.into()), expected);

        let expected = BitBoard::from(RankFile::G1);
        assert_eq!(board.diagonals(RankFile::H2.into()), expected);

        let expected = BitBoard::from(RankFile::B4);
        assert_eq!(board.diagonals(RankFile::A5.into()), expected);
    }

    #[test]
    fn test_captures_white() {
        covers!("Pawn::captures -> White");
        let mut board = Board::from(WHITE_PAWN_TEST, Player::White).unwrap();
        board.prev_move = Some(WHITE_EN_PASSANT);

        assert_eq!(
            board.find_pawn_moves(RankFile::F2.into(), RankFile::F2.into()),
            BitBoard::from(RankFile::F3).join(RankFile::F4.into())
        );

        let expected = BitBoard::from(RankFile::A3).join(RankFile::B3.into());
        assert_eq!(
            board.find_pawn_moves(RankFile::B2.into(), RankFile::B2.into()),
            expected
        );

        let expected = BitBoard::from(RankFile::B4)
            .join(RankFile::D4.into())
            .join(RankFile::C4.into());
        assert_eq!(
            board.find_pawn_moves(RankFile::C3.into(), RankFile::C3.into()),
            expected
        );
    }

    #[test]
    fn test_captures_black() {
        covers!("Pawn::captures -> Black");
        let mut board = Board::from(BLACK_PAWN_TEST, Player::Black).unwrap();
        board.prev_move = Some(BLACK_EN_PASSANT);

        assert_eq!(
            board.find_pawn_moves(RankFile::A7.into(), RankFile::A7.into()),
            BitBoard::empty()
        );

        let expected = BitBoard::from(RankFile::D1).join(RankFile::E1.into());
        assert_eq!(
            board.find_pawn_moves(RankFile::E2.into(), RankFile::E2.into()),
            expected
        );

        let expected = BitBoard::from(RankFile::H6)
            .join(RankFile::F6.into())
            .join(RankFile::G5.into())
            .join(RankFile::G6.into());
        assert_eq!(
            board.find_pawn_moves(RankFile::G7.into(), RankFile::G7.into()),
            expected
        );
    }
}
