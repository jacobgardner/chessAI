use piece::Piece;
use piece::PieceType::*;
use piece::Owner::*;
// use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Board {
    pieces: Vec<Option<Piece>>,
}

pub fn index_to_pos(index: i32) -> Result<(u32, u32), ()> {
    if index < 0 || index >= 64 {
        return Err(());
    }

    let row = 8 - (index / 8) as u32;
    let col = (index % 8) as u32;

    Ok((row, col))
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for chunk in self.pieces.chunks(8).rev() {
            for piece in chunk {
                // let char = "";
                match piece {
                    &Some(ref piece) => {
                        let is_black = match piece.owner {
                            Black => true,
                            White => false,
                        };
                        let char = match piece.piece_type {
                            Pawn => {
                                if is_black {
                                    "P"
                                } else {
                                    "p"
                                }
                            }
                            Rook => {
                                if is_black {
                                    "R"
                                } else {
                                    "r"
                                }
                            }
                            Knight => {
                                if is_black {
                                    "N"
                                } else {
                                    "n"
                                }
                            }
                            Bishop => {
                                if is_black {
                                    "B"
                                } else {
                                    "b"
                                }
                            }
                            Queen => {
                                if is_black {
                                    "Q"
                                } else {
                                    "q"
                                }
                            }
                            King => {
                                if is_black {
                                    "K"
                                } else {
                                    "k"
                                }
                            }
                        };

                        write!(f, "{}", char)?;
                    }
                    &None => {
                        write!(f, " ")?;
                    }
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
        // write!(f, );
    }
}

impl Board {
    // pub fn new() -> Board {
    //     let mut pieces = vec![];

    //     for x in 0..8 {
    //         pieces.push(Piece::new((x, 1), White, Pawn));
    //         pieces.push(Piece::new((x, 6), Black, Pawn));
    //     }


    //     Board { pieces: pieces }
    // }

    pub fn from_ascii(board: &str) -> Result<Board, ()> {
        let mut pieces = Vec::with_capacity(64);

        for ch in board.trim().chars() {
            let owner = if ch.is_uppercase() { Black } else { White };

            let piece = match ch {
                'p' | 'P' => Some(Pawn),
                'r' | 'R' => Some(Rook),
                'n' | 'N' => Some(Knight),
                'b' | 'B' => Some(Bishop),
                'q' | 'Q' => Some(Queen),
                'k' | 'K' => Some(King),
                'x' => {
                    pieces.push(None);
                    None
                }
                _ => {
                    None
                }
            };

            if let Some(piece) = piece {
                pieces.push(Some(Piece {
                    owner: owner,
                    piece_type: piece,
                }));
            }
        }

        if pieces.len() != 64 {
            return Err(());
        }

        let mut output = vec![];

        for chunk in pieces.chunks(8).rev() {
            output.extend(chunk);
        }

        Ok(Board { pieces: output })
    }
}

#[test]
fn test_from_ascii() {
    // Test typical board.
    let board = Board::from_ascii(
        "
    RNBQKBNR
    PPPPPPPP
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    pppppppp
    rnbqkbnr
    ",
    ).unwrap();

    assert_eq!(board.pieces[0], Some(Piece {piece_type: Rook, owner: White}));
    assert_eq!(board.pieces[7], Some(Piece {piece_type: Rook, owner: White}));
    assert_eq!(board.pieces[1], Some(Piece {piece_type: Knight, owner: White}));
    assert_eq!(board.pieces[6], Some(Piece {piece_type: Knight, owner: White}));
    assert_eq!(board.pieces[2], Some(Piece {piece_type: Bishop, owner: White}));
    assert_eq!(board.pieces[5], Some(Piece {piece_type: Bishop, owner: White}));
    assert_eq!(board.pieces[3], Some(Piece {piece_type: Queen, owner: White}));
    assert_eq!(board.pieces[4], Some(Piece {piece_type: King, owner: White}));

    for i in (8..16).chain(48..56) {
        let owner = if i > 16 { Black } else { White };
        assert_eq!(board.pieces[i], Some(Piece {piece_type: Pawn, owner: owner}));
    }

    assert_eq!(board.pieces[56], Some(Piece {piece_type: Rook, owner: Black}));
    assert_eq!(board.pieces[63], Some(Piece {piece_type: Rook, owner: Black}));
    assert_eq!(board.pieces[57], Some(Piece {piece_type: Knight, owner: Black}));
    assert_eq!(board.pieces[62], Some(Piece {piece_type: Knight, owner: Black}));
    assert_eq!(board.pieces[58], Some(Piece {piece_type: Bishop, owner: Black}));
    assert_eq!(board.pieces[61], Some(Piece {piece_type: Bishop, owner: Black}));
    assert_eq!(board.pieces[59], Some(Piece {piece_type: Queen, owner: Black}));
    assert_eq!(board.pieces[60], Some(Piece {piece_type: King, owner: Black}));

    for i in 16..48 {
        assert_eq!(board.pieces[i], None);
    }

    // Too Few spaces
    let board2 = Board::from_ascii(
        "
    RNBQKBNR
    PPPPPPP
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    pppppppp
    rnbqkbnr
    ",
    );

    assert_eq!(board2, Err(()));

    // Too many spaces
    let board3 = Board::from_ascii(
        "
    RNBQKBNR
    PPPPPPPPP
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    pppppppp
    rnbqkbnr
    ",
    );

    assert_eq!(board3, Err(()));
}

#[test]
fn test_board_init() {
    // let b = Board::new();
}
