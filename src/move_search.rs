use board::ChessBoard;
use piece::Owner;
use board::score::Score;


pub struct SearchNode {
    board: ChessBoard,
    nodes: Vec<SearchNode>,
    score: Score<f64>,
    turn: Owner,
    generated: bool,
}

impl SearchNode {
    pub fn new(board: ChessBoard, turn: Owner) -> Self {
        SearchNode {
            board: board,
            nodes: vec![],
            score: Score {piece_values: 0f64, positioning: 0f64},
            turn: turn,
            generated: false
        }
    }

    pub fn generate_to_depth(&mut self, depth: usize) {
        if self.generated {

        } else {
            self.generated = true;

            for board in self.board.generate_moves(&self.turn.flip()) {
                let mut node = SearchNode::new(board, self.turn.flip());

                if depth > 0 {
                    node.generate_to_depth(depth - 1);
                }

                self.nodes.push(node);
            }
        }
    }

}

