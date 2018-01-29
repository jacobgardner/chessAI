use board::ChessBoard;
use piece::Owner;


pub struct SearchNode {
    board: ChessBoard,
    nodes: Vec<SearchNode>,
    score: i64,
    turn: Owner,
    generated: bool,
}

impl SearchNode {
    pub fn new(board: ChessBoard, turn: Owner) -> Self {
        SearchNode {
            board: board,
            nodes: vec![],
            score: 0,
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

