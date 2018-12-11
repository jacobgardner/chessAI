use super::{move_generator::MoveGenerator, Board, PieceType, Player, PIECE_COUNT};
use crate::bitboard::{ROW_2, ROW_7};

use num;

impl Board {
    pub fn generate_moves(&self, player: Player) -> MoveGenerator {
        let root_board = self.clone();

        MoveGenerator::new(root_board, player)
    }
}
