use position::Position;
use std::ops::Range;

pub fn is_within_bounds(position: &Position) -> bool {
    !(position.0 < 0 || position.0 > 7 || position.1 < 0 || position.1 > 7)
}


pub fn exclusive_range(x0: i32, x1: i32) -> Range<i32> {
    if x0 < x1 {
        x0 + 1 .. x1
    } else {
        x1 + 1 .. x0
    }
}
