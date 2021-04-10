use crate::chess::{Board, Move, Player};

pub fn generate_moves_for_board(
    board: &str,
    player: Player,
    prev_move: Option<Move>,
) -> Vec<String> {
    let mut boards = vec![];

    let mut board = Board::from(board, player).unwrap();
    board.prev_move = prev_move;
    boards.push(format!("{}", board));

    for board in board.generate_moves() {
        boards.push(format!("{}", board));
    }

    boards
}
