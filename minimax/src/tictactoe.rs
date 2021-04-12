#![allow(dead_code)]
use std::fmt;

#[derive(Clone, Copy, Debug)]
enum GameState {
    InProgress,
    GameOver { winner: Option<Piece> },
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Piece {
    X,
    O,
}

#[derive(Clone)]
pub(crate) struct TicTacToe {
    spaces: Vec<Option<Piece>>,
    next_player_turn: Piece,
    width: usize,
    height: usize,
}

impl TicTacToe {
    pub(crate) fn new() -> Self {
        Self::with_size(3)
    }

    pub(crate) fn with_size(width: usize) -> Self {
        Self {
            spaces: vec![None; width * width],
            next_player_turn: Piece::X,
            width,
            height: width,
        }
    }

    pub(crate) fn generate_moves(&self) -> TicTacToeMoveGenerator {
        TicTacToeMoveGenerator::new(self.clone())
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    pub(crate) fn score_board(&self) -> f64 {
        match self.get_game_state() {
            GameState::InProgress => 0.0,
            GameState::GameOver { winner: None } => 0.0,
            GameState::GameOver {
                winner: Some(Piece::X),
            } => 100.0,
            GameState::GameOver {
                winner: Some(Piece::O),
            } => -100.0,
        }
    }

    /// This is super inefficient and gross, but I don't care
    fn get_game_state(&self) -> GameState {
        'outer: for row in 0..self.height {
            let index = self.get_index(row, 0);
            let initial_mark = self.spaces[index];

            for column in 1..self.width {
                let index = self.get_index(row, column);

                if initial_mark != self.spaces[index] {
                    continue 'outer;
                }
            }

            if let Some(p) = initial_mark {
                return GameState::GameOver { winner: Some(p) };
            }
        }

        'outer2: for column in 0..self.width {
            let index = self.get_index(0, column);
            let initial_mark = self.spaces[index];

            for row in 1..self.height {
                let index = self.get_index(row, column);

                if initial_mark != self.spaces[index] {
                    continue 'outer2;
                }
            }

            if let Some(p) = initial_mark {
                return GameState::GameOver { winner: Some(p) };
            }
        }

        let index = self.get_index(0, 0);
        let initial_mark = self.spaces[index];
        let mut we_good = true;

        for column in 1..self.width {
            let index = self.get_index(column, column);

            if initial_mark != self.spaces[index] {
                we_good = false;
                break;
            }
        }

        if we_good {
            if let Some(p) = initial_mark {
                return GameState::GameOver { winner: Some(p) };
            }
        }

        let index = self.get_index(self.height - 1, 0);
        let initial_mark = self.spaces[index];
        let mut we_good = true;

        for column in 1..self.width {
            let index = self.get_index(self.height - column - 1, column);

            if initial_mark != self.spaces[index] {
                we_good = false;
                break;
            }
        }

        if we_good {
            if let Some(p) = initial_mark {
                return GameState::GameOver { winner: Some(p) };
            }
        }

        match self.spaces.iter().find(|space| space.is_none()) {
            Some(_) => GameState::InProgress,
            None => GameState::GameOver { winner: None },
        }
    }
}

pub(crate) struct TicTacToeMoveGenerator {
    root_board: TicTacToe,
    index: usize,
}

impl TicTacToeMoveGenerator {
    fn new(root_board: TicTacToe) -> Self {
        Self {
            root_board,
            index: 0,
        }
    }
}

impl Iterator for TicTacToeMoveGenerator {
    type Item = TicTacToe;

    fn next(&mut self) -> Option<Self::Item> {
        if let GameState::GameOver {
            winner: Some(Piece::O),
        } = self.root_board.get_game_state()
        {
            return None;
        }

        while self.index < self.root_board.spaces.len() {
            if let None = self.root_board.spaces[self.index] {
                let mut new_board = self.root_board.clone();

                new_board.spaces[self.index] = Some(self.root_board.next_player_turn);
                new_board.next_player_turn = match self.root_board.next_player_turn {
                    Piece::O => Piece::X,
                    Piece::X => Piece::O,
                };

                self.index += 1;
                return Some(new_board);
            }

            self.index += 1;
        }

        None
    }
}

impl fmt::Display for TicTacToe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_string = String::new();

        for row in 0..self.height {
            for column in 0..self.width {
                let index = self.get_index(row, column);

                let piece = self.spaces[index];

                let piece_str = match piece {
                    Some(Piece::O) => "O",
                    Some(Piece::X) => "X",
                    None => " ",
                };

                board_string += piece_str;

                if column != self.width - 1 {
                    board_string += "│";
                }
            }

            board_string += "\n";

            if row != self.height - 1 {
                for column in 0..self.width {
                    if column != self.width - 1 {
                        board_string += "─┼";
                    } else {
                        board_string += "─"
                    }
                }

                board_string += "\n";
            }
        }

        write!(f, "{}", board_string)
    }
}
