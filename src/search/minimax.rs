use super::Searchable;
use super::NodeRole;
use super::NodeRole::*;
use super::Score;
// use std::rc::Rc;

pub struct SearchNode<State: Searchable<State, ScoreType>, ScoreType: Score> {
    pub state: State,
    pub score: ScoreType,
    iterator: Option<Box<Iterator<Item = State>>>,
    children: Vec<SearchNode<State, ScoreType>>,
}

// type ScoreTuple: <ScoreType

// fn update_score(role: &NodeRole, best_score, compare) {

// }

impl<'a, State: Searchable<State, ScoreType>, ScoreType: Score> SearchNode<State, ScoreType> {

    // fn update_score(role: &NodeRole, best_score: (&mut ScoreType, &mut State), compare: (&mut ScoreType, &mut State)) {
    //         match *role {
    //             Minimizer => {
    //                 if *best_score.0 < *compare.0 {
    //                     *best_score.0 = *compare.0;
    //                     best_score.1 = compare.1;
    //                 }
    //             }
    //             Maximizer => {
    //                 if *best_score.0 > *compare.0 {
    //                     *best_score.0 = *compare.0;
    //                     best_score.1 = compare.1;
    //                 }
    //             }

    //     }
    // }

    pub fn new(state: State) -> SearchNode<State, ScoreType> {
        SearchNode {
            score: state.score(),
            iterator: Some(state.generate_moves()),
            state: state,
            children: vec![],
        }
    }

    pub fn search(&mut self, search_depth: usize, role: &NodeRole) -> (ScoreType, Option<State>) {
        let mut best_score = match *role {
            Minimizer => ScoreType::min_default(),
            Maximizer => ScoreType::max_default(),
        };

        let mut best_move: Option<State> = None;

        for child in &mut self.children {
            if search_depth == 0 {
                SearchNode::update_score(role, (&mut best_score, &mut best_move), (&child.score, &child.state));
            } else {
                let results = child.search(search_depth - 1, &role.flip());
                if let (score, Some(state)) = results {
                    SearchNode::update_score(role, (&mut best_score, &mut best_move), (&score, &state));
                }
            }
        }

        if let Some(ref mut iter_box) = self.iterator {
            for child in iter_box {
                let mut search_node = SearchNode::new(child);

                if search_depth == 0 {
                    SearchNode::update_score(role, (&mut best_score, &mut best_move), (&search_node.score, &search_node.state));
                    // SearchNode::update_score(role, (&mut best_score, &mut best_move), (&search_node.score, &Some(search_node.state)));
                    // SearchNode::update_score(role, (&mut best_score, &mut best_move), (search_node.score, Some(search_node.state)));
                } else {
                    let results = search_node.search(search_depth - 1, &role.flip());
                    if let (score, Some(state)) = results {
                        SearchNode::update_score(role, (&mut best_score, &mut best_move), (&score, &state));
                    }
               }

                self.children.push(search_node);
            }
        }

        (best_score, best_move)
    }
}
