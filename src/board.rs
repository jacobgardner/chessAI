use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use num;

#[derive(Debug, PartialEq)]
pub enum InvalidStringReason {
    IncorrectLength,
    NonAsciiChars
}

impl Display for InvalidStringReason {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            InvalidStringReason::IncorrectLength => write!(formatter, "Expected string to have exactly 64 non-space characters"),
            InvalidStringReason::NonAsciiChars => write!(formatter, "Detected 1 or more non-ascii characters")
        }
    }
}

#[derive(Debug, Fail, PartialEq)]
pub enum BoardError {
    #[fail(display = "invalid player id: {}", player_id)]
    InvalidPlayer { player_id: u8 },

    #[fail(display = "invalid piece id: {}", piece_id)]
    InvalidPiece { piece_id: u8 },

    #[fail(display = "Bit found on player mask, but no board masks")]
    MalformedBoard,

    #[fail(
        display = "Rank/File exceeded board limits: {} {}",
        rank,
        file
    )]
    OutOfBounds { rank: u8, file: u8 },

    #[fail(display = "Malformed string for board: {}", _0)]
    InvalidString(InvalidStringReason),
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum Player {
    Black = 0,
    White = 1,
}

impl From<char> for Player {
    fn from(chr: char) -> Player {
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
    fn from(chr: char) -> Option<PieceType> {
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
    pub fn from(chr: char) -> Option<Piece> {
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

#[derive(PartialEq)]
pub struct Board {
    pub pieces: [u64; PIECE_COUNT],
    pub players: [u64; PLAYER_COUNT],
}

struct BitPosition(u8);
impl From<(u8, u8)> for BitPosition {
    fn from((rank, file): (u8, u8)) -> BitPosition {
        BitPosition(rank * 8 + file)
    }
}

struct PositionMask(u64);

impl From<(u8, u8)> for PositionMask {
    fn from((rank, file): (u8, u8)) -> PositionMask {
        PositionMask(1u64 << BitPosition::from((rank, file)).0)
    }
}

impl Board {
    pub fn piece_at(&self, rank: u8, file: u8) -> Result<Option<Piece>, BoardError> {
        if rank >= 8 || file >= 8 {
            return Err(BoardError::OutOfBounds { rank, file });
        }

        let mask = PositionMask::from((rank, file)).0;

        if let Some((player_id, _)) = self
            .players
            .iter()
            .enumerate()
            .find(|&(_, player_board)| mask & player_board > 0)
        {
            let player =
                num::FromPrimitive::from_usize(player_id).ok_or(BoardError::InvalidPlayer {
                    player_id: player_id as u8,
                })?;

            let (i, _) = self
                .pieces
                .iter()
                .enumerate()
                .find(|&(_, board)| mask & board > 0)
                .ok_or(BoardError::MalformedBoard)?;

            debug_assert!(i < PIECE_COUNT);

            let piece_type = num::FromPrimitive::from_usize(i)
                .ok_or(BoardError::InvalidPiece { piece_id: i as u8 })?;

            // let piece_board = (0..PIECE_COUNT).find(|&i| mask & self.pieces[i] > 0).ok_or(())?;
            Ok(Some(Piece { player, piece_type }))
        } else {
            debug_assert!({
                (0..PIECE_COUNT)
                    .find(|&i| mask & self.pieces[i] > 0)
                    .is_none()
            });

            Ok(None)
        }
    }

    pub fn from(board: &str) -> Result<Board, BoardError> {
        let mut pieces = [0; PIECE_COUNT];
        let mut players = [0; PLAYER_COUNT];

        let board: String = board.split(char::is_whitespace).collect();

        if board.len() != 64 {
            return Err(BoardError::InvalidString(InvalidStringReason::IncorrectLength));
        }

        // TODO: Make sure this correctly throws an error on non-ascii
        for (i, chr) in board.chars().enumerate() {
            if !chr.is_ascii() {
                return Err(BoardError::InvalidString(InvalidStringReason::NonAsciiChars));
            }

            let rank: u8 = (7 - (i / 8)) as u8;
            let file: u8 = (i % 8) as u8;

            let piece_mask = PositionMask::from((rank, file)).0;

            if let Some(piece) = Piece::from(chr) {
                players[piece.player as usize] |= piece_mask;
                pieces[piece.piece_type as usize] |= piece_mask;
            }
        }

        Ok(Board { pieces, players })
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
            board += &format!("| {}\n", (8 - r) * 8 - 1);
        }

        board += "       +--------+\n";
        board += "        ABCDEFGH\n";

        write!(formatter, "{}", board)
    }
}

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
    ).unwrap();

    assert_eq!(Board::from(""), Err(BoardError::InvalidString(InvalidStringReason::IncorrectLength)));
    assert_eq!(board.players[0] | board.players[1], 0);

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
    ).unwrap();

    let rook_mask = PositionMask::from((7, 7)).0;
    let king_mask = PositionMask::from((3, 2)).0;

    let queen_mask = PositionMask::from((1, 1)).0;
    let pawn_mask =
        PositionMask::from((0, 2)).0 | PositionMask::from((1, 5)).0 | PositionMask::from((6, 3)).0;

    let black_mask = rook_mask | king_mask;
    let white_mask = queen_mask | pawn_mask;

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

    assert_eq!(board, Err(BoardError::InvalidString(InvalidStringReason::NonAsciiChars)));
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
    assert_eq!(board.piece_at(0, 5), Err(BoardError::MalformedBoard));
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
