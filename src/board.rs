struct Piece;

pub struct Board {
    pieces: Vec<Piece>,
        // pieces: Vec<Piece>;
}

impl Board {
    pub fn new() -> Board {
        Board { pieces: vec![] }
    }
}

#[test]
fn test_board_init() {
    let b = Board::new();
}
