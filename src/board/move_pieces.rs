use super::{move_generator::MoveGenerator, Board, Player};

use num;

impl Board {
    pub fn generate_moves(&self, player: Player) -> MoveGenerator {
        let root_board = self.clone();

        MoveGenerator::new(root_board, player)
    }
}
