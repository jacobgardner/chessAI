mod pawn;
mod rook;

use crate::chess::bitboard::ENDS;

use crate::chess::PIECE_COUNT;
use crate::chess::{BitBoard, BitPosition, Board, Move, PieceType, Player};

pub struct MoveGenerator {
    root_board: Board,
    player: Player,

    player_mask: BitBoard,
    enemy_mask: BitBoard,
    all_pieces: BitBoard,
    player_piecetype_mask: BitBoard,

    is_first_move: bool,
    available_moves: BitBoard,
    available_captures: BitBoard,

    piece_index: usize,
}

impl MoveGenerator {
    pub fn new(root_board: Board, player: Player) -> Self {
        let player_mask = root_board.players[player as usize];
        let enemy_mask = root_board.players[1 - (player as usize)];
        let all_pieces = player_mask.join(enemy_mask);

        let mut gen = MoveGenerator {
            root_board,
            player,

            player_mask,
            enemy_mask,
            all_pieces,

            player_piecetype_mask: BitBoard::empty(),

            is_first_move: true,
            available_moves: BitBoard::empty(),
            available_captures: BitBoard::empty(),

            piece_index: 0,
        };

        gen.player_piecetype_mask = gen.generate_player_piecetype_mask(0);

        gen
    }

    fn generate_player_piecetype_mask(&self, piece_index: usize) -> BitBoard {
        self.root_board.pieces[piece_index].intersect(self.player_mask)
    }

    fn move_piece(
        &self,
        piece: PieceType,
        current_position: BitPosition,
        current_position_mask: BitBoard,
        next_position: BitPosition,
        next_position_mask: BitBoard,
        capture_mask: BitBoard,
    ) -> Board {
        let mut board = self.root_board.clone();

        let piece_index = piece as usize;
        let player_index = self.player as usize;

        // Remove current position from pawn and current player bitboards
        board.pieces[piece_index] -= current_position_mask;
        board.players[player_index] -= current_position_mask;

        if capture_mask.is_empty() {
            // self.slide_move_sanity_check(&board, next_position_mask);
        } else {
            // self.capture_sanity_check(&board, capture_mask);
            self.remove_piece(&mut board, capture_mask);
        }

        board.players[player_index] += next_position_mask;

        // NOTE: When making this function generic we'll need a PAWN check
        let next_piece = if piece == PieceType::Pawn {
            if next_position_mask.intersect(ENDS).is_empty() {
                PieceType::Pawn
            } else {
                PieceType::Queen
            }
        } else {
            piece
        };

        board.pieces[next_piece as usize] += next_position_mask;

        board.prev_move = Some(Move {
            piece_type: piece,
            from: current_position.into(),
            to: next_position.into(),
        });

        debug_assert!(self.root_board.prev_move != board.prev_move);

        board
    }

    fn remove_piece(&self, board: &mut Board, next_position_mask: BitBoard) {
        for i in 0..PIECE_COUNT {
            board.pieces[i] -= next_position_mask;
        }

        // And the previous player
        board.players[1 - (self.player as usize)] -= next_position_mask;
    }
}

impl Iterator for MoveGenerator {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.player_piecetype_mask.is_empty() {
                self.piece_index += 1;
                self.is_first_move = true;

                if self.piece_index < PIECE_COUNT {
                    self.player_piecetype_mask =
                        self.generate_player_piecetype_mask(self.piece_index);
                    // Restarting the loop because we can't be sure the next mask > 0
                    continue;
                } else {
                    return None;
                }
            }

            debug_assert!(!self.player_piecetype_mask.is_empty(), "Invariant: If the piecetype mask was 0 it should've moved on or finished the iteration");
            debug_assert!(
                self.piece_index < PIECE_COUNT,
                "Invariant: piece_index must be less than PIECE_COUNT"
            );

            // TODO: A lot of these can be cached
            let rightmost_position = self.player_piecetype_mask.first_bit_position();
            let piece_mask = BitBoard::from(rightmost_position);

            let piece_type: PieceType = num::FromPrimitive::from_usize(self.piece_index).unwrap();

            let board = match piece_type {
                PieceType::Pawn => self.generate_next_pawn_move(rightmost_position, piece_mask),
                PieceType::Rook => self.generate_next_rook_move(rightmost_position, piece_mask),
                _ => None,
            };

            match board {
                Some(board) => {
                    return Some(board);
                }
                None => {
                    self.is_first_move = true;
                    self.player_piecetype_mask -= piece_mask;
                }
            };
        }
    }
}
