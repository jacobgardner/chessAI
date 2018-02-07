pub mod alpha_beta;
pub mod minimax;

pub trait Score: PartialOrd + Copy {
    fn min_default() -> Self;
    fn max_default() -> Self;
}

pub trait CanGenerateMoves {
    type Item;
    type ScoreType;
    type MoveIter: Iterator<Item = Self::Item>;

    fn generate_moves(&self) -> Self::MoveIter;
    fn score(&self) -> Self::ScoreType;
}

pub enum NodeRole {
    Minimizer,
    Maximizer
}

use self::NodeRole::*;

impl NodeRole {
    pub fn flip(&self) -> Self {
        match *self {
            Minimizer => Maximizer,
            Maximizer => Minimizer
        }
    }
}

pub trait Searchable<State, ScoreType: Score> {
    fn score(&self) -> ScoreType;
    fn generate_moves(&self) -> Box<Iterator<Item=State>>;
}
