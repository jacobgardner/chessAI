use position::Position;

pub fn is_within_bounds(position: &Position) -> bool {
    return !(position.0 < 0 || position.0 > 7 || position.1 < 0 || position.1 > 7);
}
