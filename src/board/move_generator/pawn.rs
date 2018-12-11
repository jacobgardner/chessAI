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

    // TODO: index and current_position_mask represent the same thing.  Do we need both?
    #[inline(always)]
    fn pawn_available_moves(
        &self,
        index: BitPosition,
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
        let mut moves = self.all_pieces.inverse().intersect(match self.player {
            Player::White => BitBoard::from(index.shift(0, 1)),
            Player::Black => BitBoard::from(index.shift(0, -1)),
        });

        if !moves.is_empty() {
            moves = moves.join(self.all_pieces.inverse().intersect(match self.player {
                Player::White => {
                    if !current_position_mask.intersect(ROW_2).is_empty() {
                        BitBoard::from(index.shift(0, 2))
                    } else {
                        BitBoard::empty()
                    }
                }
                Player::Black => {
                    if !current_position_mask.intersect(ROW_7).is_empty() {
                        BitBoard::from(index.shift(0, -2))
                    } else {
                        BitBoard::empty()
                    }
                }
            }))
        }

        moves
    }

    #[inline(always)]
    fn pawn_captures(&self, index: BitPosition) -> BitBoard {
        self.enemy_mask.intersect(match self.player {
            Player::White => if !index.is_leftmost() {
                BitBoard::from(index.shift(-1, 1))
            } else {
                BitBoard::empty()
            }
            .join(if !index.is_rightmost() {
                BitBoard::from(index.shift(1, 1))
            } else {
                BitBoard::empty()
            }),
            Player::Black => if !index.is_leftmost() {
                BitBoard::from(index.shift(-1, -1))
            } else {
                BitBoard::empty()
            }
            .join(if !index.is_rightmost() {
                BitBoard::from(index.shift(1, -1))
            } else {
                BitBoard::empty()
            }),
        })
    }

    pub(crate) fn generate_next_pawn_move(
        &mut self,
        index: BitPosition,
        current_position_mask: BitBoard,
    ) -> Option<Board> {
        if self.is_first_move {
            self.available_moves = self.pawn_available_moves(index, current_position_mask);
            self.available_captures = self.pawn_captures(index);

            self.is_first_move = false;
        }

        if !self.available_moves.is_empty() {
            // We can probably join these two
            let new_move = self.available_moves.right_position();
            let next_position_mask = BitBoard::from(new_move);

            let board = self.move_pawn(current_position_mask, next_position_mask, false);

            self.available_moves = self.available_moves.intersect(next_position_mask.inverse());

            return Some(board);
        }

        if !self.available_captures.is_empty() {
            let new_move = self.available_captures.right_position();
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
    NxxxxnxN
    xxpxxxxx
    xxxxxxxx
    xxxxpxxx
    xxxxpNxx
    xxxNxxxx
";

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
