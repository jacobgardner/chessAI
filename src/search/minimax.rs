use super::Searchable;
use super::NodeRole;
use super::NodeRole::*;
use super::Score;


pub struct SearchNode<State: Searchable<State, ScoreType>, ScoreType: Score> {
    pub state: State,
    pub score: ScoreType,
    iterator: Option<Box<Iterator<Item = State>>>,
    children: Vec<SearchNode<State, ScoreType>>,
}



impl<State: Searchable<State, ScoreType>, ScoreType: Score> SearchNode<State, ScoreType> {
    pub fn new(state: State) -> SearchNode<State, ScoreType> {
        SearchNode {
            score: state.score(),
            iterator: Some(state.generate_moves()),
            state: state,
            children: vec![]
        }
    }

    pub fn search(&mut self, search_depth: usize, role: &NodeRole) -> ScoreType {
        let mut best_score = match *role {
            Minimizer => ScoreType::min_default(),
            Maximizer => ScoreType::max_default(),
        };

        for child in &mut self.children {
            if search_depth == 0 {
                best_score = match *role {
                    Minimizer => if child.score < best_score {
                        child.score
                    } else {
                        best_score
                    },
                    Maximizer => if child.score > best_score {
                        child.score
                    } else {
                        best_score
                    },
                }
            } else {
                best_score = child.search(search_depth - 1, &role.flip());
            }
        }

        if let Some(ref mut iter_box) = self.iterator {
            for child in iter_box {
                let mut search_node = SearchNode::new(child);
                if search_depth == 0 {
                    best_score = match *role {
                        Minimizer => if search_node.score < best_score {
                            search_node.score
                        } else { best_score },
                        Maximizer => if search_node.score > best_score {
                            search_node.score
                        } else {
                            best_score
                        }
                    }
                } else {
                    best_score = search_node.search(search_depth - 1, &role.flip())
                }

                self.children.push(search_node);
            }
        }
        best_score
    }
}
