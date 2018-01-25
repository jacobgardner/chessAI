use super::*;

#[derive(Debug, Copy, Clone, PartialEq)]
enum MoveResult {
    Empty,
    Enemy,
    Invalid,
}

use self::MoveResult::*;

impl ChessBoard {
    pub fn move_piece(&self, from: &Position, to: &Position) -> ChessBoard {
        let mut board = self.clone();

        board.pieces[to.to_index()] = board.pieces[from.to_index()];
        board.pieces[from.to_index()] = None;

        board.pieces[to.to_index()].unwrap().has_moved = true;

        board
    }

    pub fn generate_moves(&self, turn: &Owner) -> Vec<ChessBoard> {
        let mut children = vec![];

        for (idx, piece) in self.pieces.iter().enumerate() {
            if let &Some(piece) = piece {
                if piece.owner == *turn {
                    // pieces should never exceed 64...
                    let p = Position::from_index(idx as i32).unwrap();

                    self.find_moves(&mut children, &p, &piece);
                }
            }
        }

        let mut filtered = vec![];

        let enemy = match *turn {
            Black => White,
            White => Black,
        };

        for child in children {
            let mut in_check = false;
            {
                let king_pos = child.pieces.iter().enumerate().find(|&(idx, p)| {
                    if let &Some(p) = p {
                        &p.owner == turn && p.piece_type == King
                    } else {
                        false
                    }
                });

                if let Some((king_pos, _)) = king_pos {
                    if let Ok(king_pos) = Position::from_index(king_pos as i32) {
                        if child.is_capturable(&king_pos, &enemy) {
                            in_check = true;
                        }
                    }
                }
            }

            if !in_check {
                filtered.push(child);
            }
        }

        filtered
    }

    fn move_result(&self, position: &Position, piece: &Piece) -> MoveResult {
        if position.0 < 0 || position.0 > 7 || position.1 < 0 || position.1 > 7 {
            Invalid
        } else if let Some(piece) = self.get_piece(&position) {
            if piece.owner != piece.owner {
                Enemy
            } else {
                Invalid
            }
        } else {
            Empty
        }
    }

    fn slider(
        &self,
        boards: &mut Vec<ChessBoard>,
        origin: &Position,
        piece: &Piece,
        vectors: &[Position],
    ) {
        for vector in vectors {
            let mut multiplier = 1;
            loop {
                let position = origin + &(vector * &multiplier);

                match self.move_result(&position, piece) {
                    Invalid => break,
                    Enemy => {
                        boards.push(self.move_piece(origin, &position));
                        // moves.push(position);
                        break;
                    }
                    Empty => {
                        boards.push(self.move_piece(origin, &position));
                        // moves.push(position);
                    }
                }

                multiplier += 1;
            }
        }
    }

    fn pawn(&self, boards: &mut Vec<ChessBoard>, origin: &Position, piece: &Piece) {
        let (double_move_row, direction) = match piece.owner {
            White => (1, 1),
            Black => (6, -1),
        };

        if origin.1 == double_move_row {
            let offset = origin + &Position(0, 2 * direction);
            if !self.has_piece(&offset) {
                boards.push(self.move_piece(origin, &offset))
            }
        }

        let offset = origin + &Position(0, direction);
        if !self.has_piece(&offset) {
            boards.push(self.move_piece(origin, &offset));
        }

        // Check attack vectors
        for offset in &[Position(-1, direction), Position(1, direction)] {
            let position = origin + offset;
            if position.0 >= 0 && position.0 <= 7 {
                if let Some(piece) = self.get_piece(&position) {
                    if piece.owner != piece.owner {
                        boards.push(self.move_piece(origin, &position));
                    }
                }
            }
        }
    }

    fn rook(&self, boards: &mut Vec<ChessBoard>, origin: &Position, piece: &Piece) {
        self.slider(boards, origin, piece, &ROOK_MOVE)
    }

    fn bishop(&self, boards: &mut Vec<ChessBoard>, origin: &Position, piece: &Piece) {
        self.slider(boards, origin, piece, &BISHOP_MOVE)
    }

    fn knight(&self, boards: &mut Vec<ChessBoard>, origin: &Position, piece: &Piece) {
        for offset in &KNIGHT_MOVE {
            let position = origin + offset;

            match self.move_result(&position, piece) {
                Enemy | Empty => {
                    boards.push(self.move_piece(origin, &position));
                }
                _ => {}
            }
        }
    }

    fn king(&self, boards: &mut Vec<ChessBoard>, origin: &Position, piece: &Piece) {
        for offset in &QUEEN_MOVE {
            let position = origin + offset;

            match self.move_result(&position, piece) {
                Enemy | Empty => {
                    boards.push(self.move_piece(origin, &position));
                }
                _ => {}
            }
        }
    }

    pub fn find_moves(&self, boards: &mut Vec<ChessBoard>, origin: &Position, piece: &Piece) {
        match piece.piece_type {
            Pawn => {
                self.pawn(boards, origin, piece);
            }
            Rook => {
                self.rook(boards, origin, piece);
            }
            Bishop => {
                self.bishop(boards, origin, piece);
            }
            Queen => {
                self.rook(boards, origin, piece);
                self.bishop(boards, origin, piece);
            }
            King => {
                self.king(boards, origin, piece);
            }
            Knight => {
                self.knight(boards, origin, piece);
            }
        }
    }
}
