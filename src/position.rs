use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Position(pub i32, pub i32);

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position(self.0 + other.0, self.1 + other.1)
    }
}

impl<'a, 'b> Add<&'b Position> for &'a Position {
    type Output = Position;

    fn add(self, other: &'b Position) -> Position {
        Position(self.0 + other.0, self.1 + other.1)
    }
}

impl Position {
    pub fn from_index(index: i32) -> Result<Self, ()> {
        if index < 0 || index >= 64 {
            return Err(());
        }

        // This might have to be 8 - (index / 8)
        let row = (index / 8) as i32;
        let col = (index % 8) as i32;

        Ok(Position(col, row))
    }

    pub fn to_index(&self) -> usize {
        (self.0 + self.1 * 8) as usize
    }
}


#[test]
fn test_pos_from_index() {
    // This matches what it would be in a bitboard, but I'm not sure if that's what we're needing here.
    assert_eq!(Position::from_index(0).unwrap(), Position(0, 0));
    assert_eq!(Position(0, 0).to_index(), 0);
    assert_eq!(Position::from_index(63).unwrap(), Position(7, 7));
    assert_eq!(Position(7, 7).to_index(), 63);

    assert_eq!(Position::from_index(7).unwrap(), Position(7, 0));
    assert_eq!(Position(7, 0).to_index(), 7);
    assert_eq!(Position::from_index(8).unwrap(), Position(0, 1));
    assert_eq!(Position(0, 1).to_index(), 8);

    assert_eq!(Position::from_index(56).unwrap(), Position(0, 7));
    assert_eq!(Position::from_index(55).unwrap(), Position(7, 6));

    // Error States
    assert_eq!(Position::from_index(-1), Err(()));
    assert_eq!(Position::from_index(64), Err(()));
}
