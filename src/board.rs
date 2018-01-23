use piece::Piece;
use piece::PieceType::*;
use piece::Owner::*;
use std::fmt;

use piece::Owner;

use position::Position;

// TODO: Use real errors in this module


#[derive(Debug, PartialEq, Clone)]
pub struct ChessBoard {
    pieces: Vec<Option<Piece>>,
}

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "   --------\n");
        for (idx, chunk) in self.pieces.chunks(8).enumerate().rev() {
            write!(f, "{} |", idx + 1);
            for piece in chunk {
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
            write!(f, "|\n")?;
        }


        write!(f, "   --------\n");
        write!(f, "   abcdefgh\n");

        Ok(())
        // write!(f, );
    }
}

impl ChessBoard {
    // pub fn new() -> Board {
    //     let mut pieces = vec![];

    //     for x in 0..8 {
    //         pieces.push(Piece::new((x, 1), White, Pawn));
    //         pieces.push(Piece::new((x, 6), Black, Pawn));
    //     }


    //     Board { pieces: pieces }
    // }

    pub fn get_piece(&self, position: &Position) -> Option<Piece> {
        self.pieces[position.to_index()]
    }

    pub fn has_piece(&self, position: &Position) -> bool {
        self.pieces[position.to_index()] != None
    }

    pub fn move_piece(&self, from: &Position, to: &Position) -> ChessBoard {
        let mut board = self.clone();

        board.pieces[to.to_index()] = board.pieces[from.to_index()];
        board.pieces[from.to_index()] = None;

        board
    }

    pub fn generate_moves(&self, turn: &Owner) -> Result<Vec<ChessBoard>, ()> {
        let mut children = vec![];

        for (idx, piece) in self.pieces.iter().enumerate() {
            if let &Some(piece) = piece {
                if piece.owner == *turn {
                    let p = Position::from_index(idx as i32)?;
                    println!("{:?}", piece);

                    let valid_moves = piece.find_moves(&p, self);

                    for chess_move in valid_moves {
                        children.push(self.move_piece(&p, &chess_move))
                    }
                }
            }
        }

        Ok(children)
    }

    pub fn from_ascii(board: &str) -> Result<ChessBoard, ()> {
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
                _ => None,
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

        Ok(ChessBoard { pieces: output })
    }
}

#[test]
fn test_from_ascii() {
    // Test typical board.
    let board = ChessBoard::from_ascii(
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

    // #[rustfmt_skip]
    {
        assert_eq!(board.pieces[0], Some(Piece { piece_type: Rook, owner: White, }));
        assert_eq!(board.pieces[7], Some(Piece { piece_type: Rook, owner: White, }));
        assert_eq!(board.pieces[1], Some(Piece { piece_type: Knight, owner: White, }));
        assert_eq!(board.pieces[6], Some(Piece { piece_type: Knight, owner: White, }));
        assert_eq!(board.pieces[2], Some(Piece { piece_type: Bishop, owner: White, }));
        assert_eq!(board.pieces[5], Some(Piece { piece_type: Bishop, owner: White, }));
        assert_eq!(board.pieces[3], Some(Piece { piece_type: Queen, owner: White, }));
        assert_eq!(board.pieces[4], Some(Piece { piece_type: King, owner: White, }));
    }

    for i in (8..16).chain(48..56) {
        let owner = if i > 16 { Black } else { White };
        assert_eq!(
            board.pieces[i],
            Some(Piece {
                piece_type: Pawn,
                owner: owner,
            })
        );
    }

    // #[rustfmt_skip]
    {
        assert_eq!(board.pieces[56], Some(Piece { piece_type: Rook, owner: Black, }));
        assert_eq!(board.pieces[63], Some(Piece { piece_type: Rook, owner: Black, }));
        assert_eq!(board.pieces[57], Some(Piece { piece_type: Knight, owner: Black, }));
        assert_eq!(board.pieces[62], Some(Piece { piece_type: Knight, owner: Black, }));
        assert_eq!(board.pieces[58], Some(Piece { piece_type: Bishop, owner: Black, }));
        assert_eq!(board.pieces[61], Some(Piece { piece_type: Bishop, owner: Black, }));
        assert_eq!(board.pieces[59], Some(Piece { piece_type: Queen, owner: Black, }));
        assert_eq!(board.pieces[60], Some(Piece { piece_type: King, owner: Black, }));
    }

    for i in 16..48 {
        assert_eq!(board.pieces[i], None);
    }

    // Too Few spaces
    let board2 = ChessBoard::from_ascii(
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
    let board3 = ChessBoard::from_ascii(
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
fn test_generate_boards() {
    let board = ChessBoard::from_ascii(
        "
    xxxxxxxx
    xPxxxxPx
    xxxxxxxx
    xpxxxxxP
    xxxpxxxp
    Pxxxxxxx
    xpxxxxxx
    xxxxxxxx
    ",
    ).unwrap();

    println!("WHITE!");

    for sub_board in board.generate_moves(&White).unwrap() {
        println!("{}", sub_board);
    }

    println!("BLACK!");

    for sub_board in board.generate_moves(&Black).unwrap() {
        println!("{}", sub_board);
    }
}
