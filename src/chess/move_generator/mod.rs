mod pawn;
mod pieces;
mod sanity_checks;

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

        // TODO: Add sanity checks back
        if capture_mask.is_empty() {
            // self.slide_move_sanity_check(&board, next_position_mask);
        } else {
            // self.capture_sanity_check(&board, capture_mask);
            self.remove_piece(&mut board, capture_mask);
        }

        board.players[player_index] |= next_position_mask;

        let next_piece = if piece == PieceType::Pawn {
            if next_position_mask.intersect(ENDS).is_empty() {
                PieceType::Pawn
            } else {
                PieceType::Queen
            }
        } else {
            piece
        };

        board.pieces[next_piece as usize] |= next_position_mask;

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

    fn find_available_moves_for_piece(
        &self,
        piece_type: PieceType,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> BitBoard {
        match piece_type {
            PieceType::Rook => self.find_rook_moves(current_position, current_position_mask),
            PieceType::Bishop => self.find_bishop_moves(current_position, current_position_mask),
            PieceType::Queen => self.find_queen_moves(current_position, current_position_mask),
            PieceType::Knight => self.find_knight_moves(current_position, current_position_mask),
            PieceType::Pawn => self.find_pawn_moves(current_position, current_position_mask),
            PieceType::King => self.find_king_moves(current_position, current_position_mask),
        }
    }

    fn generate_next_move(
        &mut self,
        piece_type: PieceType,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> Option<Board> {
        if self.is_first_move {
            self.available_moves = self.find_available_moves_for_piece(
                piece_type,
                current_position,
                current_position_mask,
            );
            self.is_first_move = false;

            if piece_type == PieceType::Pawn {
                if let Some(board) =
                    self.generate_en_passant_board(current_position, current_position_mask)
                {
                    return Some(board);
                }
            }
        }

        if self.available_moves.is_empty() {
            return None;
        }

        let next_position = self.available_moves.first_bit_position();
        let next_position_mask = BitBoard::from(next_position);

        let board = self.move_piece(
            piece_type,
            current_position,
            current_position_mask,
            next_position,
            next_position_mask,
            next_position_mask.intersect(self.enemy_mask),
        );

        self.available_moves -= next_position_mask;

        Some(board)
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

            match self.generate_next_move(piece_type, rightmost_position, piece_mask) {
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
