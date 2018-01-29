use super::*;

#[derive(Debug, Copy, Clone, PartialEq)]
enum MoveResult {
    Empty,
    Enemy,
    Invalid,
}

#[derive(Debug, PartialEq)]
pub enum DiscoveredMove {
    // Move found, but there may be at least one valid move on the same angle
    Found(ChessBoard),
    // Move not found, but search not exhausted
    Terminal,
    // No more moves valid for this piece
    Exhausted,
}

use self::MoveResult::*;
use self::DiscoveredMove::*;
use piece::PieceType;

pub struct MoveIterator<'a> {
    position: usize,
    board: &'a ChessBoard,
    turn: Owner,
    gen: Option<Box<FnMut() -> DiscoveredMove + 'a>>,
}

impl<'a> MoveIterator<'a> {
    fn new(board: &'a ChessBoard, turn: Owner) -> Self {
        MoveIterator {
            position: 0,
            board: board,
            turn: turn,
            gen: None,
        }
    }
}

impl<'a> Iterator for MoveIterator<'a> {
    type Item = ChessBoard;

    fn next(&mut self) -> Option<Self::Item> {
        while self.position < 64 {
            if self.gen.is_none() {
                if let Some(piece) = self.board.pieces[self.position] {
                    if piece.owner == self.turn {
                        let pos = Position::from_index(self.position as i32).unwrap();
                        self.gen = Some(self.board.find_moves(pos, piece.piece_type, piece.owner));
                    }
                };
            }

            if let Some(ref mut gen) = self.gen {
                loop {
                    let b = gen();
                    // println!("{:?}", b);

                    if b == Exhausted {
                        break;
                    } else if let Found(b) = b {
                        if b.is_valid(&self.turn) {
                            return Some(b);
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

impl ChessBoard {
    pub fn move_piece(&self, from: &Position, to: &Position) -> ChessBoard {
        let mut board = self.clone();

        board.pieces[to.to_index()] = board.pieces[from.to_index()];
        board.pieces[from.to_index()] = None;

        board.pieces[to.to_index()].unwrap().has_moved = true;

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

    pub fn generate_moves(&self, turn: &Owner) -> MoveIterator {
        MoveIterator::new(self, *turn)
    }

    fn move_result(&self, position: &Position, owner: &Owner) -> MoveResult {
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

    fn slider<'a>(
        self: &'a Self,
        origin: Position,
        vectors: &'static [Position],
        owner: Owner,
    ) -> Box<FnMut() -> DiscoveredMove + 'a> {

        let mut generator_stage = 0;
        let mut multiplier = 1;

        let slide_generator = move || {
            if generator_stage < vectors.len() {
                let vector = &vectors[generator_stage];
                let position = &origin + &(vector * &multiplier);

                match self.move_result(&position, &owner) {
                    Empty => {
                        multiplier += 1;
                        Found(self.move_piece(&origin, &position))
                    }
                    x => {
                        generator_stage += 1;
                        multiplier = 1;

                        if let Enemy = x {
                            Found(self.move_piece(&origin, &position))
                        } else {
                            Terminal
                        }
                    },
                }
            } else {
                Exhausted
            }

        };

        Box::new(slide_generator)

        // for vector in vectors {
        //     let mut multiplier = 1;
        //     loop {
        //         let position = origin + &(vector * &multiplier);

        //         match self.move_result(&position, piece) {
        //             Invalid => break,
        //             Enemy => {
        //                 boards.push(self.move_piece(origin, &position));
        //                 break;
        //             }
        //             Empty => {
        //                 boards.push(self.move_piece(origin, &position));
        //             }
        //         }

        //         multiplier += 1;
        //     }
        // }
    }

    fn pawn<'a>(
        self: &'a Self,
        origin: Position,
        owner: Owner,
    ) -> Box<FnMut() -> DiscoveredMove + 'a> {
        let mut generator_stage = 0;

        let (double_move_row, direction) = match owner {
            White => (1, 1),
            Black => (6, -1),
        };

        let pawn_generator = move || {
            // println!("Generator_Stage: {}", generator_stage);
            match generator_stage {
                0 => {
                    generator_stage += 1;
                    if origin.1 == double_move_row {
                        let offset = &origin + &Position(0, 2 * direction);
                        if !self.has_piece(&offset) {
                            return Found(self.move_piece(&origin, &offset));
                        }
                    }
                    Terminal
                }
                1 => {
                    generator_stage += 1;
                    let offset = &origin + &Position(0, direction);
                    if !self.has_piece(&offset) {
                        Found(self.move_piece(&origin, &offset))
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
                        if let Some(victim) = self.get_piece(&position) {
                            if owner != victim.owner {
                                return Found(self.move_piece(&origin, &position));
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

    fn rook<'a>(self: &'a Self, origin: Position, owner: Owner) -> Box<FnMut() -> DiscoveredMove + 'a> {
        self.slider(origin, &ROOK_MOVE, owner)
    }

    fn bishop<'a>(self: &'a Self, origin: Position, owner: Owner) -> Box<FnMut() -> DiscoveredMove + 'a> {
        self.slider(origin, &BISHOP_MOVE, owner)
    }

    fn queen<'a>(self: &'a Self, origin: Position, owner: Owner) -> Box<FnMut() -> DiscoveredMove + 'a> {
        self.slider(origin, &QUEEN_MOVE, owner)
    }

    fn knight<'a>(self: &'a Self, origin: Position, owner: Owner) -> Box<FnMut() -> DiscoveredMove + 'a> {
        let mut generator_stage = 0;  

        let knight_generator = move || {
            if generator_stage < KNIGHT_MOVE.len() {
                let offset = &KNIGHT_MOVE[generator_stage];
                generator_stage += 1;
                let position = &origin + offset;

                match self.move_result(&position, &owner) {
                    Enemy | Empty => {
                        Found(self.move_piece(&origin, &position))
                    },
                    _ => Terminal
                }
            } else {
                Exhausted
            }
        };

        Box::new(knight_generator)
    }

    fn king<'a>(self: &'a Self, origin: Position, owner: Owner) -> Box<FnMut() -> DiscoveredMove + 'a> {
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

                match self.move_result(&position, &owner) {
                    Enemy | Empty => {
                        Found(self.move_piece(&origin, &position))
                    }
                    _ => Terminal
                } 
            } else {
                Exhausted
            }
        };

        Box::new(king_generator)
    }

    // fn king(&self, boards: &mut Vec<ChessBoard>, origin: &Position, piece: &Piece) {
    //     for offset in &QUEEN_MOVE {
    //         let position = origin + offset;

    //         match self.move_result(&position, piece) {
    //             Enemy | Empty => {
    //                 boards.push(self.move_piece(origin, &position));
    //             }
    //             _ => {}
    //         }
    //     }

    //     if !piece.has_moved && !self.is_capturable(origin, &piece.owner.flip()) {
    //         for rook_pos in &[Position(0, origin.1), Position(7, origin.1)] {
    //             if let Some(rook) = self.get_piece(rook_pos) {
    //                 if !rook.has_moved && rook.piece_type == Rook {
    //                     // check if row empty
    //                     let mut row_open = true;

    //                     for i in exclusive_range(rook_pos.0, origin.0) {
    //                         if self.get_piece(&Position(i, origin.1)).is_some() {
    //                             row_open = false;
    //                             break;
    //                         }
    //                     }

    //                     if !row_open {
    //                         continue;
    //                     }

    //                     // we can castle if not in check during move
    //                     let (rook, king) = if rook_pos.0 == 0 { (3, 2) } else { (5, 6) };

    //                     if !self.is_capturable(&Position(rook, origin.1), &piece.owner.flip()) {
    //                         let mut board = self.clone();
    //                         board.pieces[Position(king, origin.1).to_index()] =
    //                             board.pieces[origin.to_index()];
    //                         board.pieces[origin.to_index()] = None;
    //                         board.pieces[Position(rook, origin.1).to_index()] =
    //                             board.pieces[rook_pos.to_index()];
    //                         board.pieces[rook_pos.to_index()] = None;

    //                         boards.push(board);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    pub fn find_moves<'a>(
        self: &'a Self,
        origin: Position,
        piece_type: PieceType,
        owner: Owner,
    ) -> Box<FnMut() -> DiscoveredMove + 'a> {
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
