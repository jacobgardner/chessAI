use board::ChessBoard;
use position::Position;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Owner {
    White,
    Black,
}

use self::PieceType::*;
use self::Owner::*;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Piece {
    pub owner: Owner,
    pub piece_type: PieceType,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Piece {
    fn pawn(&self, origin: &Position, board: &ChessBoard) -> Vec<Position> {
        let mut moves = vec![];

        let (double_move_row, direction) = match self.owner {
            White => (1, 1),
            Black => (6, -1),
        };

        if origin.1 == double_move_row {
            let offset = origin + &Position(0, 2 * direction);
            if !board.has_piece(&offset) {
                moves.push(offset);
            }
        }

        let offset = origin + &Position(0, direction);
        if !board.has_piece(&offset) {
            moves.push(offset);
        }

        // Check attack vectors
        for offset in &[Position(-1, direction), Position(1, direction)] {
            let position = origin + offset;
            if position.0 >= 0 && position.0 <= 7 {
                if let Some(piece) = board.get_piece(&position) {
                    if piece.owner != self.owner {
                        moves.push(position);
                    }
                }
            }
        }

        moves
    }

    pub fn slider(
        &self,
        origin: &Position,
        board: &ChessBoard,
        vectors: &[Position],
    ) -> Vec<Position> {
        let mut moves = vec![];

        for direction in vectors {
            let mut multiplier = 1;
            loop {
                let position = origin + &(direction * &multiplier);

                if position.0 < 0 || position.0 > 7 || position.1 < 0 || position.1 > 7 {
                    break;
                } else if let Some(piece) = board.get_piece(&position) {
                    if piece.owner != self.owner {
                        moves.push(position);
                    }
                    break;
                } else {
                    moves.push(position);
                }

                multiplier += 1;
            }
        }

        moves
    }

    pub fn rook(&self, origin: &Position, board: &ChessBoard) -> Vec<Position> {
        self.slider(
            origin,
            board,
            &[
                Position(-1, 0),
                Position(1, 0),
                Position(0, -1),
                Position(0, 1),
            ],
        )
    }

    pub fn bishop(&self, origin: &Position, board: &ChessBoard) -> Vec<Position> {
        self.slider(
            origin,
            board,
            &[
                Position(-1, -1),
                Position(1, 1),
                Position(1, -1),
                Position(-1, 1),
            ],
        )
    }

    pub fn find_moves(&self, origin: &Position, board: &ChessBoard) -> Vec<Position> {
        let mut moves = vec![];


        match self.piece_type {
            Pawn => {
                moves.extend(self.pawn(origin, board));
            }
            Rook => {
                moves.extend(self.rook(origin, board));
            }
            Bishop => {
                moves.extend(self.bishop(origin, board));
            }
            Queen => {
                moves.extend(self.rook(origin, board));
                moves.extend(self.bishop(origin, board));
            }
            King => {
                // TODO
                panic!("Not yet implemented!")
            }
            Knight => {
                // TODO
                panic!("Not yet implemented!")
            }
        };

        moves
    }
}
