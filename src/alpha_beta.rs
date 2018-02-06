// API ->
//  * Give root node<T>
//  * T: CanGenerateMoves
//  * T:
//  * Give search depth
//  *

// So we AlphaBeta to take a type that implements CanGenerateMoves
// pub trait StateType<State, ScoreType>: CanGenerateMoves<Item=State> + ScoreState<ScoreType> {}

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    Minimizer,
    Maximizer,
}

pub struct AlphaBeta<
    'a,
    ScoreType,
    State: CanGenerateMoves<'a, ScoreType = ScoreType, Item = State>,
> {
    pub state: State,
    score: Option<ScoreType>,
    alpha: Option<ScoreType>,
    beta: Option<ScoreType>,
    children: Vec<AlphaBeta<'a, ScoreType, State>>,
    mode: Mode,
    iterator: Option<Box<Iterator<Item = State> + 'a>>,
}

pub trait CanGenerateMoves<'a> {
    type Item;
    type ScoreType;
    type MoveIter: Iterator<Item = Self::Item> + 'a;

    fn generate_moves(&'a self) -> Self::MoveIter;
    fn score(&self) -> Self::ScoreType;
}

// impl<S, T: CanGenerateMoves<S>> AlphaBeta<S, T> {

// }

impl<'a, ScoreType, State: CanGenerateMoves<'a, ScoreType = ScoreType, Item = State>>
    AlphaBeta<'a, ScoreType, State> {
    pub fn new(state: State, mode: Mode) -> Self {
        AlphaBeta {
            state: state,
            score: None,
            alpha: None,
            beta: None,
            children: vec![],
            mode: mode,
            iterator: None,
        }
    }

    pub fn generate_to_depth(&'a mut self, depth: u32) {
        let iter = self.state.generate_moves();
        let box_iter = Box::new(iter);
        self.iterator = Some(box_iter);

        if let Some(ref mut iter) = self.iterator {
            iter.next();
        }

    }
}
