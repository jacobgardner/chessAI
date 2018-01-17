use piece::Piece;
use piece::PieceType::*;
use piece::Owner::*;
// use std::error::Error;
use std::fmt;

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
                    // position -= 1;
                    None
                }
            };

            if let Some(piece) = piece {
                pieces.push(Some(Piece {
                    owner: owner,
                    piece_type: piece,
                }));
            }

            // position += 1;
        }

        let mut output: Vec<Option<Piece>> = vec![];

        for chunk in pieces.chunks(8).rev() {
            output.extend(chunk);
            // output.push(chunk);
        }

        // let chunks: Vec<Vec<Option<Piece>>> = pieces.chunks_mut(8).collect();
        Ok(Board { pieces: output })
    }
}

#[test]
fn test_from_ascii() {
    let board = Board::from_ascii(
        "
    RNBQKBNR
    PPPPPPPP
    xxxxxxxx
    xxnxQxxx
    xxKxnxxx
    xxxxxxxx
    pppppppp
    rnbqkbnr
    ",
    );

    println!("\n{}", board.unwrap());
}

#[test]
fn test_board_init() {
    // let b = Board::new();
}
