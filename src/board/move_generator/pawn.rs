use super::MoveGenerator;
use crate::bitboard::BitBoard;
use crate::bitboard::{ENDS, FILE_1, FILE_2, FILE_7, FILE_8};
use crate::bitposition::BitPosition;
use crate::board::{Board, PieceType, Player, PIECE_COUNT};

impl MoveGenerator {
    pub(crate) fn generate_next_pawn_move(
        &mut self,
        index: BitPosition,
        current_position_mask: BitBoard,
    ) -> Option<Board> {
        if self.is_first_move {
            self.available_moves = self.available_moves(index, current_position_mask);
            self.available_captures = self.pawn_captures(index);

            self.is_first_move = false;
        }

        if !self.available_moves.is_empty() {
            // We can probably join these two
            let new_move = self.available_moves.first_bit_position();
            let next_position_mask = BitBoard::from(new_move);

            let board = self.move_pawn(current_position_mask, next_position_mask, false);

            self.available_moves -= next_position_mask;

            return Some(board);
        }

        if !self.available_captures.is_empty() {
            let new_move = self.available_captures.first_bit_position();
            let next_position_mask = BitBoard::from(new_move);

            let board = self.move_pawn(current_position_mask, next_position_mask, true);

            self.available_captures -= next_position_mask;

            return Some(board);
        }

        None
    }

    // NOTE: We may be able to make this mostly work for other pieces as well
    fn move_pawn(
        &mut self,
        current_position_mask: BitBoard,
        next_position_mask: BitBoard,
        capture: bool,
    ) -> Board {
        let mut board = self.root_board.clone();

        let pawn_index = PieceType::Pawn as usize;
        let player_index = self.player as usize;

        // Remove current position from pawn and current player bitboards
        board.pieces[pawn_index] -= current_position_mask;
        board.players[player_index] -= current_position_mask;

        if capture {
            debug_assert!(
                !board.players[1 - player_index].intersect(next_position_mask).is_empty(),
                "Pawn Move Invariant Invalidated: Capture move made on space not-occupied by opponent"
            );

            // Because this is a capture we need to remove the previous piece
            self.remove_piece(&mut board, next_position_mask);
        } else {
            debug_assert!(
                board.players[player_index]
                    .intersect(next_position_mask)
                    .is_empty(),
                "Pawn Move Invariant Invalidated: Non-capture move made on space occupied by self"
            );

            debug_assert!(
                board.players[1 - player_index].intersect(next_position_mask).is_empty(),
                "Pawn Move Invariant Invalidated: Non-capture move made on space occupied by opponent"
            );
        }

        board.players[player_index] += next_position_mask;

        let next_piece = if next_position_mask.intersect(ENDS).is_empty() {
            PieceType::Pawn
        } else {
            PieceType::Queen
        };

        board.pieces[next_piece as usize] += next_position_mask;

        board
    }

    fn remove_piece(&self, board: &mut Board, next_position_mask: BitBoard) {
        for i in 0..PIECE_COUNT {
            board.pieces[i] -= next_position_mask;
        }

        // And the previous player
        board.players[1 - (self.player as usize)] -= next_position_mask;
    }

    #[inline(always)]
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

    #[inline(always)]
    fn available_moves(
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
    #[inline(always)]
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

    #[inline(always)]
    fn pawn_captures(&self, current_position: BitPosition) -> BitBoard {
        covered_by!("Pawn::captures -> White");
        covered_by!("Pawn::captures -> Black");
        self.enemy_mask.intersect(self.diagonals(current_position))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rank_file::RankFile;
    use snapshot::snapshot;

    const WHITE_PAWN_TEST: &'static str = "
    xxxrxxxx
    xxPxxxxx
    xxxxPxxx
    xxxxxxxx
    xnxnxxxx
    nxPxxxxn
    xPxxxPxP
    xxxxxxxx
    ";

    const BLACK_PAWN_TEST: &'static str = "
    xxxxxxxx
    pxxxxxpx
    NxxxxNxN
    Nxpxxxxx
    xxxxxxxx
    xxxxpxxx
    xxxxpNxx
    xxxNxxxx
    ";

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
        let board = Board::from(WHITE_PAWN_TEST).unwrap();
        let generator = MoveGenerator::new(board, Player::White);

        assert_eq!(generator.pawn_captures(RankFile::F2.into()), BitBoard::empty());

        let expected = BitBoard::from(RankFile::A3);
        assert_eq!(generator.pawn_captures(RankFile::B2.into()), expected);

        let expected = BitBoard::from(RankFile::B4).join(RankFile::D4.into());
        assert_eq!(generator.pawn_captures(RankFile::C3.into()), expected);
    }

    #[test]
    fn test_captures_black() {
        covers!("Pawn::captures -> Black");
        let board = Board::from(BLACK_PAWN_TEST).unwrap();
        let generator = MoveGenerator::new(board, Player::Black);

        assert_eq!(generator.pawn_captures(RankFile::A7.into()), BitBoard::empty());

        let expected = BitBoard::from(RankFile::D1);
        assert_eq!(generator.pawn_captures(RankFile::E2.into()), expected);

        let expected = BitBoard::from(RankFile::H6).join(RankFile::F6.into());
        assert_eq!(generator.pawn_captures(RankFile::G7.into()), expected);
    }


    #[snapshot]
    fn test_generate_white_pawn_moves() -> Vec<String> {
        let mut boards = vec![];

        let board = Board::from(WHITE_PAWN_TEST).unwrap();
        boards.push(format!("{}", board).to_owned());

        let mut generator = board.generate_moves(Player::White);

        loop {
            let new_board = match generator.next() {
                Some(board) => board,
                None => break,
            };

            boards.push(format!("{}", new_board).to_owned());
        }
        boards
    }

    #[snapshot]
    fn test_generate_black_pawn_moves() -> Vec<String> {
        let mut boards = vec![];

        let board = Board::from(BLACK_PAWN_TEST).unwrap();
        boards.push(format!("{}", board).to_owned());

        let mut generator = board.generate_moves(Player::Black);

        loop {
            let new_board = match generator.next() {
                Some(board) => board,
                None => break,
            };

            boards.push(format!("{}", new_board).to_owned());
        }

        boards
    }
}
