// LOW: Some of these may make more sense in a module up a directory
pub mod bitboard;
mod bitposition;
pub mod board;
mod chess_move;
mod errors;
mod move_generator;
mod piece;
mod piece_type;
mod player;
mod rank_file;

// LOW: Write/find macros that attaches .count() method to enum
pub const PIECE_COUNT: usize = 6;
pub const PLAYER_COUNT: usize = 2;

pub const DEFAULT_BOARD: &str = "
rnbkqbnr
pppppppp
xxxxxxxx
xxxxxxxx
xxxxxxxx
xxxxxxxx
PPPPPPPP
RNBKQBNR
";
