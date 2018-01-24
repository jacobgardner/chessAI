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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MoveResult {
    Empty,
    Enemy,
    Invalid,
}

use self::MoveResult::*;

impl Piece {
    fn move_result(&self, position: &Position, board: &ChessBoard) -> MoveResult {
        if position.0 < 0 || position.0 > 7 || position.1 < 0 || position.1 > 7 {
            Invalid
        } else if let Some(piece) = board.get_piece(&position) {
            if piece.owner != self.owner {
                Enemy
            } else {
                Invalid
            }
        } else {
            Empty
        }
    }

    fn slider(&self, origin: &Position, board: &ChessBoard, vectors: &[Position]) -> Vec<Position> {
        let mut moves = vec![];

        for direction in vectors {
            let mut multiplier = 1;
            loop {
                let position = origin + &(direction * &multiplier);

                match self.move_result(&position, board) {
                    Invalid => break,
                    Enemy => {
                        moves.push(position);
                        break;
                    }
                    Empty => {
                        moves.push(position);
                    }
                }

                multiplier += 1;
            }
        }

        moves
    }

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

    fn rook(&self, origin: &Position, board: &ChessBoard) -> Vec<Position> {
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

    fn bishop(&self, origin: &Position, board: &ChessBoard) -> Vec<Position> {
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

    fn knight(&self, origin: &Position, board: &ChessBoard) -> Vec<Position> {
        let mut moves = vec![];

        for offset in &[
            Position(-2, -1),
            Position(-2, 1),
            Position(-1, 2),
            Position(1, 2),
            Position(2, 1),
            Position(2, -1),
            Position(1, -2),
            Position(-1, -2),
        ] {
            let position = origin + offset;

            match self.move_result(&position, board) {
                Enemy | Empty => {
                    moves.push(position);
                }
                _ => {}
            }
        }

        moves
    }

    fn king(&self, origin: &Position, board: &ChessBoard) -> Vec<Position> {
        let mut moves = vec![];

        for offset in &[
            Position(-1, 0),
            Position(-1, 1),
            Position(0, 1),
            Position(1, 1),
            Position(1, 0),
            Position(1, -1),
            Position(0, -1),
            Position(-1, -1),
        ] {
            let position = origin + offset;

            match self.move_result(&position, board) {
                Enemy | Empty => {
                    moves.push(position);
                }
                _ => {}
            }
        }

        moves
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
                moves.extend(self.king(origin, board));
            }
            Knight => {
                moves.extend(self.knight(origin, board));
            }
        };

        moves
    }
}
