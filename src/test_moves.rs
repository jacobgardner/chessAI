use crate::chess::{Board, Move, Player};

pub fn generate_moves_for_board(
    board: &str,
    player: Player,
    prev_move: Option<Move>,
) -> Vec<String> {
    let mut boards = vec![];

    let mut board = Board::from(board).unwrap();
    board.prev_move = prev_move;
    boards.push(format!("{}", board).to_owned());

    let mut generator = board.generate_moves(player);

    loop {
        let new_board = match generator.next() {
            Some(board) => board,
            None => break,
        };

        boards.push(format!("{}", new_board).to_owned());
    }

    boards
}
