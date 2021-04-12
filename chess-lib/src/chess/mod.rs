mod bitboard;
mod bitposition;
mod board;
mod chess_move;
mod errors;
mod move_generator;
mod piece;
mod piece_type;
mod player;
mod rank_file;

// LOW: When we split these out into crates, all
//  these do not need to be public
pub use crate::chess::bitboard::BitBoard;
pub use crate::chess::bitposition::BitPosition;
pub use crate::chess::board::Board;
pub use crate::chess::chess_move::{Move, MoveType};
pub use crate::chess::move_generator::MoveGenerator;
pub use crate::chess::piece::Piece;
pub use crate::chess::piece_type::PieceType;
pub use crate::chess::player::Player;
pub use crate::chess::rank_file::RankFile;

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
