use std::fmt;
use std::fmt::{Debug, Display, Formatter};

mod attacks;
mod pawn;
mod pieces;

use crate::chess::bitboard::ENDS;
use crate::chess::errors::{BoardError, InvalidStringReason};
use crate::chess::{
    BitBoard, BitPosition, Move, MoveGenerator, MoveType, Piece, PieceType, Player, RankFile,
};

#[derive(PartialEq, Clone)]
pub struct Board {
    pub pieces: [BitBoard; PieceType::VARIANT_COUNT],
    pub players: [BitBoard; Player::VARIANT_COUNT],
    pub unmoved_pieces: BitBoard,
    pub prev_move: Option<Move>,
    pub next_player: Player,
}

pub struct PieceIter<'a> {
    board: &'a Board,
    pieces_left: BitBoard,
}

impl<'a> PieceIter<'a> {
    pub fn new(board: &'a Board) -> Self {
        Self {
            board,
            pieces_left: board.all_pieces(),
        }
    }
}

impl<'a> Iterator for PieceIter<'a> {
    type Item = (RankFile, Piece);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pieces_left.is_empty() {
            return None;
        }

        let next_piece = self.pieces_left.first_bit_position();
        let piece_mask = BitBoard::from(next_piece);
        let (board_idx, _) = self
            .board
            .pieces
            .iter()
            .enumerate()
            .find(|(_, b)| !b.intersect(piece_mask).is_empty())
            .unwrap();

        let (player_idx, _) = self.board.players.iter().enumerate().find(|(_, b)| !b.intersect(piece_mask).is_empty()).unwrap();

        let piece_type: PieceType = num::FromPrimitive::from_usize(board_idx).unwrap();
        let player: Player = num::FromPrimitive::from_usize(player_idx).unwrap();

        self.pieces_left -= piece_mask;

        Some((
            RankFile::from(next_piece),
            Piece {
                player,
                piece_type,
            },
        ))
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
            pieces: [BitBoard::empty(); PieceType::VARIANT_COUNT],
            players: [BitBoard::empty(); Player::VARIANT_COUNT],
            prev_move: None,
            next_player: Player::White,
            unmoved_pieces: BitBoard::empty().inverse(),
        }
    }
}

impl Board {
    pub fn empty_board() -> Board {
        Board::default()
    }

    pub fn all_pieces(&self) -> BitBoard {
        self.players[Player::White as usize].join(self.players[Player::Black as usize])
    }

    pub fn enemy_mask(&self) -> BitBoard {
        self.players[1 - (self.next_player as usize)]
    }

    pub fn piece_at(&self, rank: u8, file: u8) -> Option<Piece> {
        debug_assert!(rank < 8 && file < 8);

        let mask = BitBoard::from(BitPosition::from((rank, file)));

        if let Some((player_id, _)) = self
            .players
            .iter()
            .enumerate()
            .find(|&(_, player_board)| !mask.intersect(*player_board).is_empty())
        {
            let player = num::FromPrimitive::from_usize(player_id)
                .ok_or(BoardError::InvalidPlayer {
                    player_id: player_id as u8,
                })
                .unwrap();

            let (i, _) = self
                .pieces
                .iter()
                .enumerate()
                .find(|&(_, board)| !mask.intersect(*board).is_empty())
                .unwrap();

            debug_assert!(i < PieceType::VARIANT_COUNT);

            let piece_type = num::FromPrimitive::from_usize(i).unwrap();

            // let piece_board = (0..PieceType::VARIANT_COUNT).find(|&i| mask & self.pieces[i] > 0).ok_or(())?;
            Some(Piece { player, piece_type })
        } else {
            debug_assert!({
                (0..PieceType::VARIANT_COUNT)
                    .find(|&i| !mask.intersect(self.pieces[i]).is_empty())
                    .is_none()
            });

            None
        }
    }

    pub fn from(board: &str, player: Player) -> Result<Board, BoardError> {
        let mut pieces = [BitBoard::empty(); PieceType::VARIANT_COUNT];
        let mut players = [BitBoard::empty(); Player::VARIANT_COUNT];

        let board: String = board.split(char::is_whitespace).collect();

        if board.len() != 64 {
            return Err(BoardError::InvalidString(
                InvalidStringReason::IncorrectLength,
            ));
        }

        // LOW: Make sure this correctly throws an error on non-ascii
        for (i, chr) in board.chars().enumerate() {
            if !chr.is_ascii() {
                return Err(BoardError::InvalidString(
                    InvalidStringReason::NonAsciiChars,
                ));
            }

            let rank: u8 = (7 - (i / 8)) as u8;
            let file: u8 = (i % 8) as u8;

            let piece_mask = BitBoard::from(BitPosition::from((rank, file)));

            if let Some(piece) = Piece::from(chr) {
                let piece_type = piece.piece_type as usize;
                let player = piece.player as usize;

                players[player] = players[player].join(piece_mask);
                pieces[piece_type] = pieces[piece_type].join(piece_mask);
            }
        }

        Ok(Board {
            pieces,
            players,
            next_player: player,
            ..Default::default()
        })
    }

    pub fn iter(&self) -> PieceIter {
        PieceIter::new(&self)
    }

    pub fn generate_moves(&self) -> MoveGenerator {
        MoveGenerator::new(self.clone(), self.next_player)
    }

    pub fn move_piece(
        &self,
        piece: PieceType,
        current_position: BitPosition,
        current_position_mask: BitBoard,
        next_position: BitPosition,
        next_position_mask: BitBoard,
        capture_mask: BitBoard,
    ) -> Board {
        let mut board = self.clone();

        let piece_index = piece as usize;
        let player_index = self.next_player as usize;

        board.unmoved_pieces -= current_position_mask.join(next_position_mask);

        debug_assert!(
            !board.pieces[piece_index]
                .intersect(current_position_mask)
                .is_empty(),
            "Expected to move piece, {:?}, but that piece wasn't in that space.",
            piece
        );
        debug_assert!(
            !board.players[player_index]
                .intersect(current_position_mask)
                .is_empty(),
            "Expected to move piece, {:?} {:?}, but they weren't in that space.",
            self.next_player,
            piece
        );

        // Remove current position from piece and current player bitboards
        board.pieces[piece_index] -= current_position_mask;
        board.players[player_index] -= current_position_mask;

        // TODO: Add sanity checks back
        if capture_mask.is_empty() {
            // self.slide_move_sanity_check(&board, next_position_mask);
        } else {
            // self.capture_sanity_check(&board, capture_mask);
            board.remove_piece(capture_mask);
        }

        board.players[player_index] |= next_position_mask;

        board.prev_move = Some(Move {
            piece_type: piece,
            from: current_position.into(),
            to: next_position.into(),
            is_capture: !capture_mask.is_empty(),
            ..Default::default()
        });

        let next_piece = if piece == PieceType::Pawn {
            if next_position_mask.intersect(ENDS).is_empty() {
                PieceType::Pawn
            } else {
                if let Some(m) = board.prev_move.as_mut() {
                    m.move_type = MoveType::Promotion {
                        promoted_to: PieceType::Queen,
                    }
                }

                PieceType::Queen
            }
        } else {
            piece
        };

        board.pieces[next_piece as usize] |= next_position_mask;

        debug_assert!(self.prev_move != board.prev_move);

        board.next_player = match self.next_player {
            Player::White => Player::Black,
            Player::Black => Player::White,
        };

        board
    }

    pub fn perform_castle(&self, is_queenside: bool) -> Board {
        let king_position: BitPosition = match self.next_player {
            Player::White => RankFile::E1,
            Player::Black => RankFile::E8,
        }
        .into();
        let king_position_mask = BitBoard::from(king_position);

        let rook_position: BitPosition = match self.next_player {
            Player::White => {
                if is_queenside {
                    RankFile::A1
                } else {
                    RankFile::H1
                }
            }
            Player::Black => {
                if is_queenside {
                    RankFile::A8
                } else {
                    RankFile::H8
                }
            }
        }
        .into();

        let (next_rook_mask, next_king_mask) = if is_queenside {
            (
                BitBoard::from(rook_position).shift_right(3),
                king_position_mask.shift_left(2),
            )
        } else {
            (
                BitBoard::from(rook_position).shift_left(2),
                king_position_mask.shift_right(2),
            )
        };

        let next_rook_position = next_rook_mask.first_bit_position();

        let mut board = self.move_piece(
            PieceType::Rook,
            rook_position,
            rook_position.into(),
            next_rook_position,
            next_rook_mask,
            BitBoard::empty(),
        );
        board.next_player = self.next_player;

        let next_king_position = next_king_mask.first_bit_position();
        let mut board = board.move_piece(
            PieceType::King,
            king_position,
            king_position_mask,
            next_king_position,
            next_king_mask,
            BitBoard::empty(),
        );

        if let Some(m) = board.prev_move.as_mut() {
            m.is_capture = true;
            m.move_type = MoveType::Castling { is_queenside };
        }

        board
    }

    fn remove_piece(&mut self, next_position_mask: BitBoard) {
        for i in 0..PieceType::VARIANT_COUNT {
            self.pieces[i] -= next_position_mask;
        }

        // And the previous player
        self.players[1 - (self.next_player as usize)] -= next_position_mask;
    }
}

impl Debug for Board {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        Display::fmt(self, formatter)
    }
}

impl Display for Board {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let mut board = String::with_capacity(128);

        board += &self
            .prev_move
            .as_ref()
            .map_or("First Move".to_owned(), |m| format!("{}", m));

        board += "\n\n";
        board += "       ╔═════════════════╗\n";

        for r in 0..8 {
            board += &format!("0x{: <02x} {} ║ ", (7 - r) * 8, 8 - r);

            for f in 0..8 {
                let piece = self.piece_at(7 - r, f);

                let chr = if let Some(piece) = piece {
                    piece.to_char()
                } else {
                    '·'
                };

                board += &chr.to_string();
                board += " ";
            }
            board += &format!("║ {}\n", (8 - r) * 8 - 1);
        }

        board += "       ╚═════════════════╝\n";
        board += "         A B C D E F G H\n";

        write!(formatter, "{}", board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::*;

    #[test]
    fn test_board_from_str() {
        let board = Board::from(
            "
            xxxxxxxx
            xxxxxxxx
            xxxxxxxx
            xxxxxxxx
            xxxxxxxx
            xxxxxxxx
            xxxxxxxx
            xxxxxxxx
            ",
            Player::White,
        )
        .unwrap();

        assert_eq!(
            Board::from("", Player::White),
            Err(BoardError::InvalidString(
                InvalidStringReason::IncorrectLength
            ))
        );
        assert_eq!(board.players[0].join(board.players[1]), BitBoard::empty());

        let board = Board::from(
            "
            .......r
            ...P....
            ........
            ........
            ..k.....
            ........
            .Q...P..
            ..P.....
            ",
            Player::White,
        )
        .unwrap();

        let rook_mask = BitBoard::from(BitPosition::from((7, 7)));
        let king_mask = BitBoard::from(BitPosition::from((3, 2)));

        let queen_mask = BitBoard::from(BitPosition::from((1, 1)));
        let pawn_mask = BitBoard::from(BitPosition::from((0, 2)))
            .join(BitPosition::from((1, 5)).into())
            .join(BitPosition::from((6, 3)).into());

        let black_mask = rook_mask.join(king_mask);
        let white_mask = queen_mask.join(pawn_mask);

        assert_eq!(board.players[Player::Black as usize], black_mask);
        assert_eq!(board.players[Player::White as usize], white_mask);

        assert_eq!(board.pieces[PieceType::Pawn as usize], pawn_mask);
        assert_eq!(board.pieces[PieceType::Queen as usize], queen_mask);
        assert_eq!(board.pieces[PieceType::Rook as usize], rook_mask);
        assert_eq!(board.pieces[PieceType::King as usize], king_mask);

        let board = Board::from(
            "
            .......r
            ...😀...
            ........
            ......
            ..k.....
            ........
            .Q...P..
            ..P.....
            ",
            Player::White,
        );

        assert_eq!(
            board,
            Err(BoardError::InvalidString(
                InvalidStringReason::NonAsciiChars
            ))
        );
    }

    #[test]
    fn test_piece_at() {
        let pieces: [BitBoard; PieceType::VARIANT_COUNT] = [
            BitBoard::from(1),
            BitBoard::from(1 << 8),
            BitBoard::from(1 << 12),
            BitBoard::from(1 << 16),
            BitBoard::from(1 << 25),
            BitBoard::from(1 << 63),
        ];

        let board = Board {
            players: [
                pieces[0]
                    .join(pieces[2])
                    .join(pieces[4])
                    .join(BitPosition::from(5).into()),
                pieces[1].join(pieces[3]).join(pieces[5]),
            ],
            pieces,
            next_player: Player::White,
            ..Default::default()
        };

        assert_eq!(
            board.piece_at(0, 0),
            Some(Piece {
                piece_type: PieceType::Pawn,
                player: Player::Black
            })
        );

        assert_eq!(
            board.piece_at(1, 0),
            Some(Piece {
                piece_type: PieceType::Rook,
                player: Player::White
            })
        );

        assert_eq!(
            board.piece_at(1, 4),
            Some(Piece {
                piece_type: PieceType::Knight,
                player: Player::Black
            })
        );

        assert_eq!(
            board.piece_at(2, 0),
            Some(Piece {
                piece_type: PieceType::Bishop,
                player: Player::White
            })
        );

        assert_eq!(
            board.piece_at(3, 1),
            Some(Piece {
                piece_type: PieceType::Queen,
                player: Player::Black
            })
        );

        assert_eq!(
            board.piece_at(7, 7),
            Some(Piece {
                piece_type: PieceType::King,
                player: Player::White
            })
        );

        assert_eq!(board.piece_at(4, 4), None);
        // TODO: Use panic catch test
        // assert_eq!(board.piece_at(0, 5), Err(BoardError::MalformedBoard));
    }

}
