use super::Board;
use super::{PieceType, Player, PIECE_COUNT};
use crate::bitboard::{ROW_2, ROW_7};

enum GeneratorState {
    Init,
    PieceLoop,
    PieceIncrement,
    DrainingPieceTypeMask,
}

pub struct MoveGenerator {
    state: GeneratorState,
    root_board: Board,
    player: Player,

    player_mask: u64,
    enemy_mask: u64,
    all_pieces: u64,
    player_piecetype_mask: u64,

    is_first_move: bool,
    available_moves: u64,
    available_captures: u64,

    piece_index: usize,
}

impl MoveGenerator {
    pub fn new(root_board: Board, player: Player) -> Self {
        let player_mask = root_board.players[player as usize];
        let enemy_mask = root_board.players[1 - (player as usize)];
        let all_pieces = player_mask | enemy_mask;

        let mut gen = MoveGenerator {
            state: GeneratorState::Init,
            root_board,
            player,

            player_mask,
            enemy_mask,
            all_pieces,

            player_piecetype_mask: 0,

            is_first_move: true,
            available_moves: 0,
            available_captures: 0,

            piece_index: 0,
        };

        gen.player_piecetype_mask = gen.generate_piecemask(0);

        gen
    }

    fn generate_piecemask(&self, piece_index: usize) -> u64 {
        self.root_board.pieces[piece_index as usize] & self.player_mask
    }

    fn generate_next_pawn_move(&mut self, index: u32, piece_mask: u64) -> Option<Board> {
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

impl Iterator for MoveGenerator {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.player_piecetype_mask == 0 {
                self.piece_index += 1;
                self.is_first_move = true;

                if self.piece_index < PIECE_COUNT {
                    self.player_piecetype_mask = self.generate_piecemask(self.piece_index);
                    // Restarting the loop because we can't be sure the next mask > 0
                    continue;
                } else {
                    return None;
                }
            }

            assert!(self.player_piecetype_mask > 0);
            assert!(self.piece_index < PIECE_COUNT);

            // TODO: A lot of these can be cached
            let index = self.player_piecetype_mask.trailing_zeros();
            let piece_mask = 1 << index;
            let piece_inverse = !piece_mask;

            let piece_type: PieceType = num::FromPrimitive::from_usize(self.piece_index).unwrap();

            match piece_type {
                PieceType::Pawn => match self.generate_next_pawn_move(index, piece_mask) {
                    Some(board) => {
                        return Some(board);
                    }
                    None => {
                        self.is_first_move = true;
                        self.player_piecetype_mask &= piece_inverse;
                    }
                },
                _ => {
                    // TODO: We'll want to remove the piece from the mask if there are no
                    //  moves left.
                    self.player_piecetype_mask &= piece_inverse;
                }
            }
        }
    }
}
