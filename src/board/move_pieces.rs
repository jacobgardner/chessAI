use super::*;
use std::rc::Rc;
use piece::PieceType;
// use search::{Searchable, Score};

#[derive(Debug, PartialEq)]
// TODO: Rename this to something more fitting
pub enum DiscoveredMove {
    // Move found, but there may be at least one valid move on the same angle
    Found(ChessBoard),
    // Move not found, but search not exhausted
    Terminal,
    // No more moves valid for this piece
    Exhausted,
}

use self::DiscoveredMove::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MoveResult {
    Empty,
    Enemy,
    Invalid,
}

use self::MoveResult::*;

#[derive(Clone)]
pub struct IterableChessBoard(pub Rc<ChessBoard>);

pub struct MoveIterator {
    position: usize,
    state: IterableChessBoard,
    gen: Option<Box<FnMut() -> DiscoveredMove>>,
}

impl MoveIterator {
    fn new(board: &IterableChessBoard) -> Self {
        MoveIterator {
            position: 0,
            state: board.clone(),
            gen: None,
        }
    }
}

impl Iterator for MoveIterator {
    type Item = IterableChessBoard;

    fn next(&mut self) -> Option<Self::Item> {
        while self.position < 64 {
            if self.gen.is_none() {
                if let Some(piece) = self.state.0.pieces[self.position] {
                    if piece.owner == self.state.0.turn {
                        let pos = Position::from_index(self.position as i32).unwrap();
                        self.gen = Some(self.state.find_moves(pos, piece.piece_type, piece.owner));
                    }
                };
            }

            if let Some(ref mut gen) = self.gen {
                loop {
                    let b = gen();

                    if b == Exhausted {
                        break;
                    } else if let Found(b) = b {
                        if b.is_valid(&self.state.0.turn) {
                            return Some(IterableChessBoard(Rc::new(b)));
                        }
                    }
                }
            }

            self.gen = None;
            self.position += 1;
        }

        None
    }
}

impl IterableChessBoard {
    pub fn move_iterator(&self) -> MoveIterator {
        MoveIterator::new(self)
    }

    fn slider(
        &self,
        origin: Position,
        vectors: &'static [Position],
        owner: Owner,
    ) -> Box<FnMut() -> DiscoveredMove> {
        let &IterableChessBoard(ref board) = self;
        let board = board.clone();

        let mut generator_stage = 0;
        let mut multiplier = 1;

        let slide_generator = move || {
            if generator_stage < vectors.len() {
                let vector = &vectors[generator_stage];
                let position = &origin + &(vector * &multiplier);

                match board.move_result(&position, &owner) {
                    Empty => {
                        multiplier += 1;
                        Found(board.move_piece(&origin, &position))
                    }
                    x => {
                        generator_stage += 1;
                        multiplier = 1;

                        if let Enemy = x {
                            Found(board.move_piece(&origin, &position))
                        } else {
                            Terminal
                        }
                    }
                }
            } else {
                Exhausted
            }
        };

        Box::new(slide_generator)
    }

    fn pawn(&self, origin: Position, owner: Owner) -> Box<FnMut() -> DiscoveredMove> {
        let &IterableChessBoard(ref board) = self;
        let board = board.clone();

        let mut generator_stage = 0;

        let (double_move_row, direction) = match owner {
            White => (1, 1),
            Black => (6, -1),
        };

        let pawn_generator = move || {
            match generator_stage {
                0 => {
                    generator_stage += 1;
                    if origin.1 == double_move_row {
                        let offset = &origin + &Position(0, 2 * direction);
                        if !board.has_piece(&offset) {
                            return Found(board.move_piece(&origin, &offset));
                        }
                    }
                    Terminal
                }
                1 => {
                    generator_stage += 1;
                    let offset = &origin + &Position(0, direction);
                    if !board.has_piece(&offset) {
                        Found(board.move_piece(&origin, &offset))
                    } else {
                        Terminal
                    }
                }
                2 | 3 => {
                    // Check attack vectors

                    let offset =
                        &[Position(-1, direction), Position(1, direction)][generator_stage - 2];

                    generator_stage += 1;

                    let position = &origin + offset;
                    if position.0 >= 0 && position.0 <= 7 {
                        if let Some(victim) = board.get_piece(&position) {
                            if owner != victim.owner {
                                return Found(board.move_piece(&origin, &position));
                            }
                        }
                    }

                    Terminal
                }
                _ => Exhausted,
            }
        };

        Box::new(pawn_generator)
    }

    fn rook(&self, origin: Position, owner: Owner) -> Box<FnMut() -> DiscoveredMove> {
        self.slider(origin, &ROOK_MOVE, owner)
    }

    fn bishop(&self, origin: Position, owner: Owner) -> Box<FnMut() -> DiscoveredMove> {
        self.slider(origin, &BISHOP_MOVE, owner)
    }

    fn queen(&self, origin: Position, owner: Owner) -> Box<FnMut() -> DiscoveredMove> {
        self.slider(origin, &QUEEN_MOVE, owner)
    }

    fn knight(&self, origin: Position, owner: Owner) -> Box<FnMut() -> DiscoveredMove> {
        let &IterableChessBoard(ref board) = self;
        let board = board.clone();

        let mut generator_stage = 0;

        let knight_generator = move || {
            if generator_stage < KNIGHT_MOVE.len() {
                let offset = &KNIGHT_MOVE[generator_stage];
                generator_stage += 1;
                let position = &origin + offset;

                match board.move_result(&position, &owner) {
                    Enemy | Empty => Found(board.move_piece(&origin, &position)),
                    _ => Terminal,
                }
            } else {
                Exhausted
            }
        };

        Box::new(knight_generator)
    }

    fn king(&self, origin: Position, owner: Owner) -> Box<FnMut() -> DiscoveredMove> {
        let &IterableChessBoard(ref board) = self;
        let board = board.clone();

        let mut generator_stage = 0;

        let king_generator = move || {
            if generator_stage < 2 {
                generator_stage += 1;
                // Castling
                // TODO: Implement again...
                Terminal
            } else if generator_stage - 2 < QUEEN_MOVE.len() {
                let offset = &QUEEN_MOVE[generator_stage - 2];
                generator_stage += 1;
                let position = &origin + offset;

                match board.move_result(&position, &owner) {
                    Enemy | Empty => Found(board.move_piece(&origin, &position)),
                    _ => Terminal,
                }
            } else {
                Exhausted
            }
        };

        Box::new(king_generator)
    }

    pub fn find_moves(
        &self,
        origin: Position,
        piece_type: PieceType,
        owner: Owner,
    ) -> Box<FnMut() -> DiscoveredMove> {
        match piece_type {
            Pawn => self.pawn(origin, owner),
            King => self.king(origin, owner),
            Knight => self.knight(origin, owner),
            Bishop => self.bishop(origin, owner),
            Rook => self.rook(origin, owner),
            Queen => self.queen(origin, owner),
        }
    }
}

impl ChessBoard {
    pub fn move_piece(&self, from: &Position, to: &Position) -> ChessBoard {
        let mut board = self.clone();

        board.pieces[to.to_index()] = board.pieces[from.to_index()];
        board.pieces[from.to_index()] = None;

        let moved_piece = &mut board.pieces[to.to_index()].unwrap();
        moved_piece.has_moved = true;

        // This upgrades a Pawn to queen if it moved to an end space.
        //  No color check is made because there should be no way that
        //  it moves to it's own side's end.
        if moved_piece.piece_type == Pawn && (to.1 == 0 || to.1 == 7) {
            moved_piece.piece_type = Queen;
        }

        board.turn = board.turn.flip();

        board
    }

    pub fn is_valid(&self, turn: &Owner) -> bool {
        let king_pos = self.pieces.iter().enumerate().find(|&(_, p)| {
            if let Some(p) = *p {
                &p.owner == turn && p.piece_type == King
            } else {
                false
            }
        });

        if let Some((king_pos, _)) = king_pos {
            if let Ok(king_pos) = Position::from_index(king_pos as i32) {
                if self.is_capturable(&king_pos, &turn.flip()) {
                    // King is capturable with this board... invalid move
                    return false;
                }
            }
        } else {
            // No King found... board invalid
            return false;
        }

        // King was found and wasn't capturable
        true
    }

    pub fn move_result(&self, position: &Position, owner: &Owner) -> MoveResult {
        if position.0 < 0 || position.0 > 7 || position.1 < 0 || position.1 > 7 {
            Invalid
        } else if let Some(victim) = self.get_piece(position) {
            if owner != &victim.owner {
                Enemy
            } else {
                Invalid
            }
        } else {
            Empty
        }
    }
}
