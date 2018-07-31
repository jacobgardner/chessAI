use bitboard::ROW_1;
use std::fmt;
use std::fmt::{Display, Error, Formatter};

use num;
// #[macro_use]
// use num_derive;

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum Player {
    Black = 0,
    White = 1,
}

impl<'a> From<&'a char> for Player {
    fn from(chr: &'a char) -> Player {
        if chr.is_lowercase() {
            Player::Black
        } else {
            Player::White
        }
    }
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum PieceType {
    Pawn = 0,
    Rook = 1,
    Knight = 2,
    Bishop = 3,
    Queen = 4,
    King = 5,
}

impl PieceType {
    fn from(chr: &char) -> Option<PieceType> {
        let piece_type = match chr.to_lowercase().next().unwrap_or('x') {
            'p' => PieceType::Pawn,
            'r' => PieceType::Rook,
            'n' => PieceType::Knight,
            'b' => PieceType::Bishop,
            'q' => PieceType::Queen,
            'k' => PieceType::King,
            _ => return None,
        };

        Some(piece_type)
    }
}

#[derive(Debug, PartialEq)]
pub struct Piece {
    piece_type: PieceType,
    player: Player,
}

impl Piece {
    pub fn from(chr: &char) -> Option<Piece> {
        if let Some(piece_type) = PieceType::from(chr) {
            Some(Piece {
                piece_type,
                player: Player::from(chr),
            })
        } else {
            None
        }
    }
}

const PIECE_COUNT: usize = 6;
const PLAYER_COUNT: usize = 2;
// const BOARD_COUNT: usize = PIECE_COUNT + PLAYER_COUNT;

pub struct Board {
    pub pieces: [u64; PIECE_COUNT],
    pub players: [u64; PLAYER_COUNT],
}

struct BitPosition(usize);
impl From<(usize, usize)> for BitPosition {
    fn from((rank, file): (usize, usize)) -> BitPosition {
        BitPosition(rank * 8 + file)
    }
}

struct PositionMask(u64);

impl From<(usize, usize)> for PositionMask {
    fn from((rank, file): (usize, usize)) -> PositionMask {
        PositionMask(1u64 << BitPosition::from((rank, file)).0)
    }
}

impl Board {
    pub fn piece_at(&self, rank: usize, file: usize) -> Result<Option<Piece>, ()> {
        if rank >= 8 || file >= 8 {
            return Err(());
        }

        let mask = PositionMask::from((rank, file)).0;
        // println!("{:0b}", mask);

        if let Some((player_id, _)) = self
            .players
            .iter()
            .enumerate()
            .find(|&(i, player_board)| mask & player_board > 0)
        {
            let player = num::FromPrimitive::from_usize(player_id).ok_or(())?;

            let (i, _) = self
                .pieces
                .iter()
                .enumerate()
                .find(|&(_, board)| mask & board > 0)
                .ok_or(())?;

            debug_assert!(i < PIECE_COUNT);

            let piece_type = num::FromPrimitive::from_usize(i).ok_or(())?;

            // let piece_board = (0..PIECE_COUNT).find(|&i| mask & self.pieces[i] > 0).ok_or(())?;
            Ok(Some(Piece { player, piece_type }))
        } else {
            debug_assert!({
                let mut mask_found = false;
                (0..PIECE_COUNT)
                    .find(|&i| mask & self.pieces[i] > 0)
                    .is_none()
            });

            Ok(None)
        }
    }
}

impl<'a> From<&'a str> for Board {
    fn from(board: &str) -> Board {
        Board {
            pieces: [0; PIECE_COUNT],
            players: [0; PLAYER_COUNT],
        }
    }
}

impl Display for Board {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let mut board = String::with_capacity(128);

        for r in 0..8 {
            for f in 0..8 {
                let piece = self.piece_at(7-r, f).map_err(|()| Error)?;

                // let piece = Some(Piece {
                //     piece_type: PieceType::Pawn,
                //     player: Player::White
                // });

                let chr = if let Some(piece) = piece {
                    let piece_chr = match piece.piece_type {
                        PieceType::Pawn => 'p',
                        PieceType::Rook => 'r',
                        PieceType::Bishop => 'b',
                        PieceType::Knight => 'n',
                        PieceType::King => 'k',
                        PieceType::Queen => 'q',
                    };

                    if piece.player == Player::White {
                        piece_chr.to_ascii_uppercase()
                    } else {
                        piece_chr
                    }
                } else {
                    '.'
                };

                board += &chr.to_string();
            }
            board += "\n";
        }

        write!(formatter, "{}", board)
    }
}

#[test]
fn test_piece_at() {
    let pieces: [u64; PIECE_COUNT] = [1, 1 << 8, 1 << 12, 1 << 16, 1 << 25, 1 << 63];

    let board = Board {
        players: [
            pieces[0] | pieces[2] | pieces[4] | 1 << 5,
            pieces[1] | pieces[3] | pieces[5],
        ],
        pieces: pieces,
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
    assert_eq!(board.piece_at(0, 5), Err(()));
}

#[test]
fn test_player_from_str() {
    assert_eq!(Player::from(&'Q'), Player::White);
    assert_eq!(Player::from(&'K'), Player::White);
    assert_eq!(Player::from(&'N'), Player::White);
    assert_eq!(Player::from(&'B'), Player::White);
    assert_eq!(Player::from(&'P'), Player::White);
    assert_eq!(Player::from(&'R'), Player::White);
    assert_eq!(Player::from(&'X'), Player::White);

    assert_eq!(Player::from(&'q'), Player::Black);
    assert_eq!(Player::from(&'k'), Player::Black);
    assert_eq!(Player::from(&'n'), Player::Black);
    assert_eq!(Player::from(&'b'), Player::Black);
    assert_eq!(Player::from(&'p'), Player::Black);
    assert_eq!(Player::from(&'r'), Player::Black);
    assert_eq!(Player::from(&'x'), Player::Black);
}

#[test]
fn test_to_piece_owner() {
    assert_eq!(
        Piece::from(&'P'),
        Some(Piece {
            piece_type: PieceType::Pawn,
            player: Player::White
        })
    );
    assert_eq!(
        Piece::from(&'p'),
        Some(Piece {
            piece_type: PieceType::Pawn,
            player: Player::Black
        })
    );

    assert_eq!(
        Piece::from(&'K'),
        Some(Piece {
            piece_type: PieceType::King,
            player: Player::White
        })
    );
    assert_eq!(
        Piece::from(&'k'),
        Some(Piece {
            piece_type: PieceType::King,
            player: Player::Black
        })
    );

    assert_eq!(Piece::from(&'l'), None);
}

#[test]
fn test_piece_from_str() {
    assert_eq!(PieceType::from(&'P'), Some(PieceType::Pawn));
    assert_eq!(PieceType::from(&'p'), Some(PieceType::Pawn));

    assert_eq!(PieceType::from(&'R'), Some(PieceType::Rook));
    assert_eq!(PieceType::from(&'r'), Some(PieceType::Rook));

    assert_eq!(PieceType::from(&'N'), Some(PieceType::Knight));
    assert_eq!(PieceType::from(&'n'), Some(PieceType::Knight));

    assert_eq!(PieceType::from(&'B'), Some(PieceType::Bishop));
    assert_eq!(PieceType::from(&'b'), Some(PieceType::Bishop));

    assert_eq!(PieceType::from(&'Q'), Some(PieceType::Queen));
    assert_eq!(PieceType::from(&'q'), Some(PieceType::Queen));

    assert_eq!(PieceType::from(&'K'), Some(PieceType::King));
    assert_eq!(PieceType::from(&'k'), Some(PieceType::King));

    assert_eq!(PieceType::from(&'x'), None);
    assert_eq!(PieceType::from(&'L'), None);
}
