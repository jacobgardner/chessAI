
// API ->
//  * Give root node<T>
//  * T: CanGenerateMoves
//  * T:
//  * Give search depth
//  *

pub struct AlphaBeta<T: CanGenerateMoves<T>> {
    board: T,
}

pub trait MoveIterator<T> {

}

pub trait CanGenerateMoves<T> {
    fn generate_moves(&self) -> T;
}

impl<T: CanGenerateMoves<T>> AlphaBeta<T> {

}
