mod tests;
mod move_pieces;

use piece::Piece;
use piece::PieceType::*;
use piece::Owner::*;
use std::fmt;
use moves::*;
use utils::is_within_bounds;

use piece::Owner;

use position::Position;

// TODO: Use real errors in this module

pub const DEFAULT_CONFIGURATION: &str = "
    RNBQKBNR
    PPPPPPPP
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    pppppppp
    rnbqkbnr";

#[derive(Debug, PartialEq, Clone)]
pub struct ChessBoard {
    pieces: Vec<Option<Piece>>,
}

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "   abcdefgh\n")?;
        write!(f, "   --------\n")?;
        for (idx, chunk) in self.pieces.chunks(8).enumerate().rev() {
            write!(f, "{} |", idx + 1)?;
            for piece in chunk {
                match *piece {
                    Some(piece) => {
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
                    None => {
                        write!(f, " ")?;
                    }
                }
            }
            write!(f, "| {}\n", idx + 1)?;
        }

        write!(f, "   --------\n")?;
        write!(f, "   abcdefgh\n")?;

        Ok(())
        // write!(f, );
    }
}

impl ChessBoard {
    pub fn is_capturable(&self, origin: &Position, owner: &Owner) -> bool {
        for vector in &QUEEN_MOVE {
            let mut multiplier = 1;

            loop {
                let position = origin + &(vector * &multiplier);

                if !is_within_bounds(&position) {
                    break;
                } else if let Some(piece) = self.pieces[position.to_index()] {
                    let is_capturable = if &piece.owner == owner {
                        match piece.piece_type {
                            Bishop => (vector.0 + vector.1) % 2 == 0,
                            Rook => (vector.0 + vector.1) % 2 != 0,
                            Queen => true,
                            King => multiplier == 1,
                            Pawn => {
                                if multiplier == 1 {
                                    match *owner {
                                        Black => {
                                            vector == &Position(1, 1) || vector == &Position(-1, 1)
                                        }
                                        White => {
                                            vector == &Position(1, -1)
                                                || vector == &Position(-1, -1)
                                        }
                                    }
                                } else {
                                    false
                                }
                            }
                            _ => false,
                        }
                    } else {
                        false
                    };

                    if is_capturable {
                        return true;
                    }

                    break;
                }

                multiplier += 1;
            }
        }

        for offset in &KNIGHT_MOVE {
            let position = origin + offset;
            if is_within_bounds(&position) {
                if let Some(piece) = self.pieces[position.to_index()] {
                    if &piece.owner == owner && piece.piece_type == Knight {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn get_piece(&self, position: &Position) -> Option<Piece> {
        self.pieces[position.to_index()]
    }

    pub fn has_piece(&self, position: &Position) -> bool {
        self.pieces[position.to_index()] != None
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
                pieces.push(Some(Piece::new(piece, owner)));
            }
        }

        if pieces.len() != 64 {
            return Err(());
        }

        // TODO: This is predicated on Piece being a Copy type.
        //  This feels... off... not sure why?
        let mut output = vec![];

        for chunk in pieces.chunks(8).rev() {
            output.extend(chunk);
        }

        Ok(ChessBoard { pieces: output })
    }
}
