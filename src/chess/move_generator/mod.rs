mod sanity_checks;

use crate::chess::PIECE_COUNT;
use crate::chess::{BitBoard, BitPosition, Board, PieceType, Player};

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

    fn find_available_moves_for_piece(
        &self,
        piece_type: PieceType,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> BitBoard {
        match piece_type {
            PieceType::Rook => self
                .root_board
                .find_rook_moves(current_position, current_position_mask),
            PieceType::Bishop => self
                .root_board
                .find_bishop_moves(current_position, current_position_mask),
            PieceType::Queen => self
                .root_board
                .find_queen_moves(current_position, current_position_mask),
            PieceType::Knight => self
                .root_board
                .find_knight_moves(current_position, current_position_mask),
            PieceType::Pawn => self
                .root_board
                .find_pawn_moves(current_position, current_position_mask),
            PieceType::King => self
                .root_board
                .find_king_moves(current_position, current_position_mask),
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
                if let Some(board) = self
                    .root_board
                    .generate_en_passant_board(current_position, current_position_mask)
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

        let board = self.root_board.move_piece(
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
        'outer: loop {
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
                    // TODO: Test if these moves are actually removed
                    let mut king_mask = board.players[self.player as usize]
                        .intersect(board.pieces[PieceType::King as usize]);

                    while !king_mask.is_empty() {
                        let first_king_position = king_mask.first_bit_position();
                        let first_king_mask = BitBoard::from(first_king_position);

                        if board.is_attacked(first_king_position, first_king_mask) {
                            continue 'outer;
                        }

                        king_mask -= first_king_mask;
                    }

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
