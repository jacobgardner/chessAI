mod sanity_checks;

use crate::chess::bitboard::{CASTLE_CHECK, KINGSIDE_CASTLE, QUEENSIDE_CASTLE};
use crate::chess::{BitBoard, BitPosition, Board, PieceType, Player, RankFile};

pub struct MoveGenerator {
    /// The board/turn from which all moves we are generating from
    root_board: Board,
    /// The player whose pieces are moving this turn.
    player: Player,

    /// The moving player's bitboard
    player_mask: BitBoard,
    /// The opponent player's bitboard
    enemy_mask: BitBoard,
    /// The bitboard representing all piece positions
    all_pieces: BitBoard,
    /// As we iterate through the player's pieces that we're 
    /// generating moves for, we build up this bitboard for
    /// representing the remaining pieces left to iterate through
    /// for that piecetype
    player_piecetype_mask: BitBoard,

    /// For this piece, have we generated the move bitboards yet?
    have_piece_moves_been_generated: bool,
    /// 
    available_moves: BitBoard,
    possible_castle: BitBoard,

    /// The current piece type we're examining
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

            have_piece_moves_been_generated: false,
            available_moves: BitBoard::empty(),
            possible_castle: BitBoard::empty(),

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
        let moves = match piece_type {
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
        };

        moves - self.player_mask
    }

    fn check_for_castling(&mut self, piece_type: PieceType, current_position_mask: BitBoard) {
        if piece_type != PieceType::King
            || self
                .root_board
                .unmoved_pieces
                .intersect(current_position_mask)
                .is_empty()
        {
            return;
        }

        self.possible_castle = self.root_board.pieces[PieceType::Rook as usize]
            .intersect(self.player_mask)
            .intersect(self.root_board.unmoved_pieces);

        if !self.possible_castle.is_empty()
            && self
                .root_board
                .is_attacked(self.player, current_position_mask)
        {
            self.possible_castle = BitBoard::empty();
        }
    }

    fn generate_next_castling_board(&mut self) -> Option<Board> {
        for rook_position in self.possible_castle.by_ref() {
            let rf = RankFile::from(rook_position);

            let is_queenside = rf.file() == 0;

            let mut spaces = if is_queenside {
                QUEENSIDE_CASTLE
            } else {
                KINGSIDE_CASTLE
            };

            if self.player == Player::Black {
                spaces = spaces.shift_up(7);
            }

            if spaces.intersect(self.all_pieces).is_empty()
                && !self
                    .root_board
                    .is_attacked(self.player, spaces.intersect(CASTLE_CHECK))
            {
                return Some(self.root_board.perform_castle(is_queenside));
            }
        }

        None
    }

    fn generate_next_move(
        &mut self,
        piece_type: PieceType,
        current_position: BitPosition,
        current_position_mask: BitBoard,
    ) -> Option<Board> {
        if !self.have_piece_moves_been_generated {
            self.available_moves = self.find_available_moves_for_piece(
                piece_type,
                current_position,
                current_position_mask,
            );

            self.have_piece_moves_been_generated = true;

            if piece_type == PieceType::Pawn {
                if let Some(board) = self
                    .root_board
                    .generate_en_passant_board(current_position, current_position_mask)
                {
                    return Some(board);
                }
            }

            self.check_for_castling(piece_type, current_position_mask);
        }

        if piece_type == PieceType::King {
            if let Some(board) = self.generate_next_castling_board() {
                return Some(board);
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
                self.have_piece_moves_been_generated = false;

                if self.piece_index < PieceType::VARIANT_COUNT {
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
                self.piece_index < PieceType::VARIANT_COUNT,
                "Invariant: piece_index must be less than PieceType::VARIANT_COUNT"
            );

            // TODO: A lot of these can be cached
            let rightmost_position = self.player_piecetype_mask.first_bit_position();
            let piece_mask = BitBoard::from(rightmost_position);

            let piece_type: PieceType = num::FromPrimitive::from_usize(self.piece_index).unwrap();

            match self.generate_next_move(piece_type, rightmost_position, piece_mask) {
                Some(board) => {
                    // LOW: This loop only matters for tests where we have > 1 king.
                    //  Possibly remove and optimize for release mode?
                    let king_mask = board.players[self.player as usize]
                        .intersect(board.pieces[PieceType::King as usize]);

                    for first_king_position in king_mask {
                        let first_king_mask = BitBoard::from(first_king_position);

                        if board.is_attacked(self.player, first_king_mask) {
                            continue 'outer;
                        }
                    }

                    return Some(board);
                }
                None => {
                    self.have_piece_moves_been_generated = false;
                    self.player_piecetype_mask -= piece_mask;
                }
            };
        }
    }
}
