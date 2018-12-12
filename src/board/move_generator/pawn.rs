use super::MoveGenerator;
use crate::bitboard::BitBoard;
use crate::bitboard::{ROW_1, ROW_2, ROW_7, ROW_8};
use crate::bitposition::BitPosition;
use crate::board::{Board, PieceType, Player, PIECE_COUNT};

// TODO: En Passant (https://en.wikipedia.org/wiki/En_passant)

impl MoveGenerator {
    // TODO: We may be able to make this mostly work for other pieces as well
    fn move_pawn(
        &mut self,
        current_position_mask: BitBoard,
        next_position_mask: BitBoard,
        capture: bool,
    ) -> Board {
        let mut board = self.root_board.clone();

        // Remove current position from pawn and current player bitboards
        board.pieces[PieceType::Pawn as usize] =
            board.pieces[PieceType::Pawn as usize].intersect(current_position_mask.inverse());
        board.players[self.player as usize] =
            board.players[self.player as usize].intersect(current_position_mask.inverse());

        if capture {
            debug_assert!(
                board.players[self.player as usize]
                    .intersect(next_position_mask)
                    .is_empty(),
                "Pawn Move Invariant Invalidated: Capture move made on space occupied by self"
            );

            debug_assert!(
                !board.players[1 - (self.player as usize)].intersect(next_position_mask).is_empty(),
                "Pawn Move Invariant Invalidated: Capture move made on space not-occupied by opponent"
            );

            // Because this is a capture we need to remove the previous piece
            for i in 0..PIECE_COUNT {
                board.pieces[i] = board.pieces[i].intersect(next_position_mask.inverse());
            }

            // And the previous player
            board.players[1 - (self.player as usize)] =
                board.players[1 - (self.player as usize)].intersect(next_position_mask.inverse());
        } else {
            debug_assert!(
                board.players[self.player as usize]
                    .intersect(next_position_mask)
                    .is_empty(),
                "Pawn Move Invariant Invalidated: Non-capture move made on space occupied by self"
            );
            debug_assert!(
                board.players[1 - (self.player as usize)].intersect(next_position_mask).is_empty(),
                "Pawn Move Invariant Invalidated: Non-capture move made on space occupied by opponent"
            );
        }

        board.players[self.player as usize] =
            board.players[self.player as usize].join(next_position_mask);

        if !next_position_mask.intersect(ROW_1).is_empty()
            || !next_position_mask.intersect(ROW_8).is_empty()
        {
            board.pieces[PieceType::Queen as usize] =
                board.pieces[PieceType::Queen as usize].join(next_position_mask);
        } else {
            board.pieces[PieceType::Pawn as usize] =
                board.pieces[PieceType::Pawn as usize].join(next_position_mask);
        }

        board
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
        self.all_pieces.inverse().intersect(BitBoard::from(
            pawn_position.shift(0, self.pawn_direction()),
        ))
    }

    fn check_for_double_move(
        &self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> BitBoard {
        covered_by!("Pawn::available_double_moves -> White");
        covered_by!("Pawn::available_double_moves -> Black");

        let (direction, row) = match self.player {
            Player::White => (2, ROW_2),
            Player::Black => (-2, ROW_7),
        };

        if !current_position_mask.intersect(row).is_empty() {
            BitBoard::from(current_position.shift(0, direction))
        } else {
            BitBoard::empty()
        }.intersect(self.all_pieces.inverse())
    }

    // TODO: index and current_position_mask represent the same thing.  Do we need both?
    #[inline(always)]
    fn available_moves(
        &self,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> BitBoard {
        debug_assert!(
            current_position_mask
                .intersect(ROW_1.join(ROW_8))
                .is_empty(),
            "Pawn Invariant Invalidation: Pawn must never appear in the first or last row"
        );

        // TODO: Our snapshot tests caught this bug occuring but it wasn't able to pinpoint
        //  where the bug was occuring.  We need more granular unit tests to catch stuff
        //  like this.
        let mut moves = self.check_for_single_move(current_position);

        // If we couldn't single-move, we definitely can't double move
        if !moves.is_empty() {
            moves = moves.join(self.check_for_double_move(current_position, current_position_mask));
        }

        moves
    }

    #[inline(always)]
    fn diagonals(&self, current_position: BitPosition) -> BitBoard {
        match self.player {
            Player::White => {
                let left_diagonal = if !current_position.is_leftmost() {
                    BitBoard::from(current_position.shift(-1, 1))
                } else {
                    BitBoard::empty()
                };

                let right_diagonal = if !current_position.is_rightmost() {
                    BitBoard::from(current_position.shift(1, 1))
                } else {
                    BitBoard::empty()
                };

                left_diagonal.join(right_diagonal)
            }

            Player::Black => {
                let left_diagonal = if !current_position.is_leftmost() {
                    BitBoard::from(current_position.shift(-1, -1))
                } else {
                    BitBoard::empty()
                };

                let right_diagonal = if !current_position.is_rightmost() {
                    BitBoard::from(current_position.shift(1, -1))
                } else {
                    BitBoard::empty()
                };

                left_diagonal.join(right_diagonal)
            }
        }
    }

    #[inline(always)]
    fn pawn_captures(&self, current_position: BitPosition) -> BitBoard {
        self.enemy_mask.intersect(self.diagonals(current_position))
    }

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

            self.available_moves = self.available_moves.intersect(next_position_mask.inverse());

            return Some(board);
        }

        if !self.available_captures.is_empty() {
            let new_move = self.available_captures.first_bit_position();
            let next_position_mask = BitBoard::from(new_move);
            let inverse_move = next_position_mask.inverse();

            let board = self.move_pawn(current_position_mask, next_position_mask, true);

            self.available_captures = self.available_captures.intersect(inverse_move);

            return Some(board);
        }

        None
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

    const BLACK_PAWN_TEST: &'static str = "
    xxxxxxxx
    pxxxxxpx
    NxxxxnxN
    Nxpxxxxx
    xxxxxxxx
    xxxxpxxx
    xxxxpNxx
    xxxNxxxx
    ";

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
