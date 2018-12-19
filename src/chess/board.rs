use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use crate::chess::errors::{BoardError, InvalidStringReason};
use crate::chess::{BitBoard, BitPosition, Move, MoveGenerator, Piece, PieceType, Player};
use crate::chess::{PIECE_COUNT, PLAYER_COUNT};

#[derive(PartialEq, Clone)]
pub struct Board {
    pub pieces: [BitBoard; PIECE_COUNT],
    pub players: [BitBoard; PLAYER_COUNT],
    pub prev_move: Option<Move>,
}

impl Board {
    pub fn empty_board() -> Board {
        Board {
            pieces: [BitBoard::empty(); PIECE_COUNT],
            players: [BitBoard::empty(); PLAYER_COUNT],
            prev_move: None,
        }
    }

    pub fn piece_at(&self, rank: u8, file: u8) -> Result<Option<Piece>, BoardError> {
        if rank >= 8 || file >= 8 {
            return Err(BoardError::OutOfBounds { rank, file });
        }

        let mask = BitBoard::from(BitPosition::from((rank, file)));

        if let Some((player_id, _)) = self
            .players
            .iter()
            .enumerate()
            .find(|&(_, player_board)| !mask.intersect(*player_board).is_empty())
        {
            let player =
                num::FromPrimitive::from_usize(player_id).ok_or(BoardError::InvalidPlayer {
                    player_id: player_id as u8,
                })?;

            let (i, _) = self
                .pieces
                .iter()
                .enumerate()
                .find(|&(_, board)| !mask.intersect(*board).is_empty())
                .ok_or(BoardError::MalformedBoard)?;

            debug_assert!(i < PIECE_COUNT);

            let piece_type = num::FromPrimitive::from_usize(i)
                .ok_or(BoardError::InvalidPiece { piece_id: i as u8 })?;

            // let piece_board = (0..PIECE_COUNT).find(|&i| mask & self.pieces[i] > 0).ok_or(())?;
            Ok(Some(Piece { player, piece_type }))
        } else {
            debug_assert!({
                (0..PIECE_COUNT)
                    .find(|&i| !mask.intersect(self.pieces[i]).is_empty())
                    .is_none()
            });

            Ok(None)
        }
    }

    pub fn from(board: &str) -> Result<Board, BoardError> {
        let mut pieces = [BitBoard::empty(); PIECE_COUNT];
        let mut players = [BitBoard::empty(); PLAYER_COUNT];

        let board: String = board.split(char::is_whitespace).collect();

        if board.len() != 64 {
            return Err(BoardError::InvalidString(
                InvalidStringReason::IncorrectLength,
            ));
        }

        // TODO: Make sure this correctly throws an error on non-ascii
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
            prev_move: None,
        })
    }

    pub fn generate_moves(&self, player: Player) -> MoveGenerator {
        MoveGenerator::new(self.clone(), player)
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

        board += "       +--------+\n";

        for r in 0..8 {
            // let rank_chr = (65u8 + (7 - r as u8))  as char;

            board += &format!("0x{: <02x} {} |", (7 - r) * 8, 8 - r);

            for f in 0..8 {
                let piece = self.piece_at(7 - r, f).map_err(|_| fmt::Error)?;

                let chr = if let Some(piece) = piece {
                    let (white_piece, black_piece) = match piece.piece_type {
                        PieceType::Pawn => ( '♙', '♟' ),
                        PieceType::Rook => ('♖', '♜'),
                        PieceType::Bishop => ('♗', '♝'),
                        PieceType::Knight => ('♘', '♞'),
                        PieceType::King => ('♔', '♚'),
                        PieceType::Queen => ('♕', '♛'),
                    };

                    if piece.player == Player::White {
                        white_piece
                    } else {
                        black_piece
                    }
                } else {
                    '.'
                };

                board += &chr.to_string();
            }
            board += &format!("| {}\n", (8 - r) * 8 - 1);
        }

        board += "       +--------+\n";
        board += "        ABCDEFGH\n";

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
        )
        .unwrap();

        assert_eq!(
            Board::from(""),
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
        let pieces: [BitBoard; PIECE_COUNT] = [
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
            prev_move: None,
        };

        assert_eq!(
            board.piece_at(0, 0).unwrap(),
            Some(Piece {
                piece_type: PieceType::Pawn,
                player: Player::Black
            })
        );

        assert_eq!(
            board.piece_at(1, 0).unwrap(),
            Some(Piece {
                piece_type: PieceType::Rook,
                player: Player::White
            })
        );

        assert_eq!(
            board.piece_at(1, 4).unwrap(),
            Some(Piece {
                piece_type: PieceType::Knight,
                player: Player::Black
            })
        );

        assert_eq!(
            board.piece_at(2, 0).unwrap(),
            Some(Piece {
                piece_type: PieceType::Bishop,
                player: Player::White
            })
        );

        assert_eq!(
            board.piece_at(3, 1).unwrap(),
            Some(Piece {
                piece_type: PieceType::Queen,
                player: Player::Black
            })
        );

        assert_eq!(
            board.piece_at(7, 7).unwrap(),
            Some(Piece {
                piece_type: PieceType::King,
                player: Player::White
            })
        );

        assert_eq!(board.piece_at(4, 4).unwrap(), None);
        assert_eq!(board.piece_at(0, 5), Err(BoardError::MalformedBoard));
    }

}
