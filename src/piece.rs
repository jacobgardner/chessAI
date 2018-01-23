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

    pub fn find_moves(&self, origin: &Position, board: &ChessBoard) -> Vec<Position> {
        let mut moves = vec![];


        match self.piece_type {
            Pawn => {moves.extend(self.pawn(origin, board));}
            _ => {}
        };

        moves
    }
}
