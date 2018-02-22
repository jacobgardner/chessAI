use super::Searchable;
use super::NodeRole;
use super::NodeRole::*;
use super::Score;
// use std::rc::Rc;

pub struct SearchNode<State: Searchable<State, ScoreType> + Clone, ScoreType: Score> {
    pub state: State,
    pub score: ScoreType,
    iterator: Option<Box<Iterator<Item = State>>>,
    children: Vec<SearchNode<State, ScoreType>>,
}

fn score_improved<ScoreType: Score>(role: &NodeRole, best_score: &ScoreType, compare: &ScoreType) -> bool {
    match *role {
        Minimizer => compare < best_score,
        Maximizer => compare > best_score,
    }
}

impl<State: Searchable<State, ScoreType> + Clone, ScoreType: Score> SearchNode<State, ScoreType> {
    pub fn new(state: State) -> SearchNode<State, ScoreType> {
        SearchNode {
            score: state.score(),
            iterator: Some(state.generate_moves()),
            state: state,
            children: vec![],
        }
    }

    pub fn search(&mut self, search_depth: usize, role: &NodeRole) -> (ScoreType, Option<State>, usize) {
        let mut moves = 0usize;
        let mut best_score = match *role {
            Minimizer => ScoreType::min_default(),
            Maximizer => ScoreType::max_default(),
        };

        let mut best_move: Option<State> = None;

        for child in &mut self.children {
            moves += 1;
            if search_depth == 0 {
                if score_improved(role, &best_score, &child.score) {
                    best_score = child.score;
                    best_move = Some(child.state.clone());
                }
            } else {
                let results = child.search(search_depth - 1, &role.flip());
                moves += results.2;

                if score_improved(role, &best_score, &results.0) {
                    best_score = results.0;
                    best_move = Some(child.state.clone());
                }
            }
        }

        if let Some(ref mut iter_box) = self.iterator {
            moves += 1;
            for child in iter_box {
                let mut search_node = SearchNode::new(child);

                if search_depth == 0 {
                    if score_improved(role, &best_score, &search_node.score) {
                        best_score = search_node.score;
                        best_move = Some(search_node.state.clone());
                    }

               } else {
                    let results = search_node.search(search_depth - 1, &role.flip());
                    moves += results.2;

                    if score_improved(role, &best_score, &results.0) {
                        best_score = results.0;
                        best_move = Some(search_node.state.clone());
                    }
                }

                self.children.push(search_node);
            }
        }

        (best_score, best_move, moves)
    }
}
