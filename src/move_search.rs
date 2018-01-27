use board::ChessBoard;


struct SearchNode {
    board: ChessBoard,
    nodes: Vec<SearchNode>,
    score: i64,
}

// impl SearchNode {
//     fn new(board: &ChessBoard) -> Self {
//         SearchNode
//     }
// }

pub fn search(board: &ChessBoard, depth: &u32) {

}
