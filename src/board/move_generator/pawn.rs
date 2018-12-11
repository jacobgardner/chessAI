use super::MoveGenerator;
use crate::bitboard::{ROW_2, ROW_7};
use crate::board::{Board, Player};

impl MoveGenerator {
    pub(crate) fn generate_next_pawn_move(&mut self, index: u32, piece_mask: u64) -> Option<Board> {
        if self.is_first_move == true {
            self.available_moves = match self.player {
                // I'm not too worred about this overflowing (vertical moves only)
                //  because if it gets to the end it turns into a queen
                Player::White => {
                    (if piece_mask & ROW_2 > 0 {
                        1 << (index + 16)
                    } else {
                        0
                    }) | (1 << (index + 8))
                }
                Player::Black => {
                    (if piece_mask & ROW_7 > 0 {
                        1 << (index - 16)
                    } else {
                        0
                    }) | (1 << (index - 8))
                }
            } & !self.all_pieces;

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
            let new_move_mask = 1 << new_move;

            let mut board = self.root_board.clone();
            board.pieces[self.piece_index] |= new_move_mask;
            board.pieces[self.piece_index] &= !piece_mask;
            board.players[self.player as usize] |= new_move_mask;
            board.players[self.player as usize] &= !piece_mask;

            self.available_moves &= !new_move_mask;

            return Some(board);
        }

        if self.available_captures > 0 {
            let new_move = self.available_captures.trailing_zeros();
            let new_move_mask = 1 << new_move;
            let inverse_move = !new_move_mask;

            let mut board = self.root_board.clone();
            board.pieces[self.piece_index] |= new_move_mask;
            board.pieces[self.piece_index] &= !piece_mask;
            board.players[self.player as usize] |= new_move_mask;
            board.players[self.player as usize] &= !piece_mask;
            board.players[1 - (self.player as usize)] &= inverse_move;

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
    nxPxxxxx
    xPxxxPxP
    xxxxxxxx
    ";

    const BLACK_PAWN_TEST: &'static str = "
    xxxxxxxx
    pxxxxxpx
    xxxxxnxN
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
