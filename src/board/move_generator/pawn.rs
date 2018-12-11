use super::MoveGenerator;
use crate::bitboard::{ROW_1, ROW_2, ROW_7, ROW_8};
use crate::board::{Board, PieceType, Player, PIECE_COUNT};

// TODO: En Passant (https://en.wikipedia.org/wiki/En_passant)

impl MoveGenerator {
    // TODO: We may be able to make this mostly work for other pieces as well
    fn move_pawn(
        &mut self,
        current_position_mask: u64,
        next_position_mask: u64,
        capture: bool,
    ) -> Board {
        let mut board = self.root_board.clone();

        // Remove current position from pawn and current player bitboards
        board.pieces[PieceType::Pawn as usize] &= !current_position_mask;
        board.players[self.player as usize] &= !current_position_mask;

        if capture {
            debug_assert!(
                board.players[self.player as usize] & next_position_mask == 0,
                "Pawn Move Invariant Invalidated: Capture move made on space occupied by self"
            );
            debug_assert!(board.players[1 - (self.player as usize)] & next_position_mask != 0, "Pawn Move Invariant Invalidated: Capture move made on space not-occupied by opponent");
            // Because this is a capture we need to remove the previous piece
            for i in 0..PIECE_COUNT {
                board.pieces[i] &= !next_position_mask;
            }

            // And the previous player
            board.players[1 - (self.player as usize)] &= !next_position_mask;
        } else {
            debug_assert!(
                board.players[self.player as usize] & next_position_mask == 0,
                "Pawn Move Invariant Invalidated: Non-capture move made on space occupied by self"
            );
            debug_assert!(board.players[1 - (self.player as usize)] & next_position_mask == 0, "Pawn Move Invariant Invalidated: Non-capture move made on space occupied by opponent");
        }

        board.players[self.player as usize] |= next_position_mask;
        if next_position_mask & ROW_1 > 0 || next_position_mask & ROW_8 > 0 {
            board.pieces[PieceType::Queen as usize] |= next_position_mask;
        } else {
            board.pieces[PieceType::Pawn as usize] |= next_position_mask;
        }

        board
    }

    #[inline(always)]
    fn pawn_available_moves(&self, index: u32, current_position_mask: u64) -> u64 {
        debug_assert!(current_position_mask & (ROW_1 | ROW_8) == 0);

        let mut moves = !self.all_pieces
            & match self.player {
                Player::White => 1 << (index + 8),
                Player::Black => 1 << (index - 8),
            };

        if moves > 0 {
            moves |= !self.all_pieces
                & match self.player {
                    Player::White => {
                        if current_position_mask & ROW_2 > 0 {
                            1 << (index + 16)
                        } else {
                            0
                        }
                    }
                    Player::Black => {
                        if current_position_mask & ROW_7 > 0 {
                            1 << (index - 16)
                        } else {
                            0
                        }
                    }
                }
        }

        moves
    }

    pub(crate) fn generate_next_pawn_move(
        &mut self,
        index: u32,
        current_position_mask: u64,
    ) -> Option<Board> {
        if self.is_first_move == true {
            self.available_moves = self.pawn_available_moves(index, current_position_mask);

            self.available_captures = match self.player {
                Player::White => {
                    (if index % 8 != 0 { 1 << (index + 9) } else { 0 })
                        | (if index % 8 != 7 { 1 << (index + 7) } else { 0 })
                }
                Player::Black => {
                    (if index % 8 != 0 { 1 << (index - 9) } else { 0 })
                        | (if index % 8 != 7 { 1 << (index - 7) } else { 0 })
                }
            } & self.enemy_mask;

            self.is_first_move = false;
        }

        if self.available_moves > 0 {
            let new_move = self.available_moves.trailing_zeros();
            let next_position_mask = 1 << new_move;

            let board = self.move_pawn(current_position_mask, next_position_mask, false);

            self.available_moves &= !next_position_mask;

            return Some(board);
        }

        if self.available_captures > 0 {
            let new_move = self.available_captures.trailing_zeros();
            let next_position_mask = 1 << new_move;
            let inverse_move = !next_position_mask;

            let board = self.move_pawn(current_position_mask, next_position_mask, true);

            self.available_captures &= inverse_move;

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
