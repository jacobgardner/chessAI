// Score should take into account:
//  * Piece values (King being highest, pawn being lowest)
//  * Positioning?  Should, all other things being equal, we value higher the position of pieces on the board or just
//      let the search decide that?
// *

use std::cmp::{Ordering, PartialOrd};

#[derive(Debug, PartialEq)]
pub struct Score<T> {
    pub piece_values: T,
    pub positioning: T,
}

impl<T: PartialOrd> PartialOrd for Score<T> {
    fn partial_cmp(&self, other: &Score<T>) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.piece_values != other.piece_values {
            self.piece_values.partial_cmp(&other.piece_values)
        } else {
            self.positioning.partial_cmp(&other.positioning)
        }
    }
}

#[test]
fn test_equality() {
    assert_eq!(Score {piece_values: 0, positioning: 0}, Score {piece_values: 0, positioning: 0});
    assert_eq!(Score {piece_values: 1, positioning: 0}, Score {piece_values: 1, positioning: 0});
    assert_ne!(Score {piece_values: 0, positioning: 0}, Score {piece_values: 0, positioning: 5});
}

#[test]
fn test_comparisons() {
    assert_eq!(Score {piece_values: 0, positioning: 10} < Score {piece_values: 1, positioning: 0}, true);
    assert_eq!(Score {piece_values: 1, positioning: 10} > Score {piece_values: 1, positioning: 0}, true);
    assert_eq!(Score {piece_values: 1, positioning: 0} < Score {piece_values: 1, positioning: 10}, true);
}
