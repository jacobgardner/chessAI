use super::{move_generator::MoveGenerator, Board, PieceType, Player, PIECE_COUNT};
use crate::bitboard::{ROW_2, ROW_7};

use num;

impl Board {
    pub fn generate_moves(&self, player: Player) -> MoveGenerator {
        let root_board = self.clone();

        MoveGenerator::new(root_board, player)
    }
}

const WHITE_PAWN_TEST: &'static str = "
    xxxrxxxx
    xxPxxxxx
    xxxxPxxx
    xxxxxxxx
    xnxnxxxx
    nxPxxxxx
    xPxxxPxP
    xxxxxxxx
    ";

const BLACK_PAWN_TEST: &'static str = "
    xxxxxxxx
    pxxxxxpx
    xxxxxnxN
    xxpxxxxx
    xxxxxxxx
    xxxxpxxx
    xxxxpNxx
    xxxNxxxx
";

#[test]
fn test_generate_moves() {
    let board = Board::from(WHITE_PAWN_TEST).unwrap();

    let mut generator = board.generate_moves(Player::White);

    loop {
        let new_board = match generator.next() {
            Some(board) => board,
            None => break,
        };

        println!("{}", new_board);
    }

    let board = Board::from(BLACK_PAWN_TEST).unwrap();

    let mut generator = board.generate_moves(Player::Black);

    loop {
        let new_board = match generator.next() {
            Some(board) => board,
            None => break,
        };

        println!("{}", new_board);
    }
}
