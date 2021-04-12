use variant_count::VariantCount;

#[derive(Debug, PartialEq, FromPrimitive, Copy, Clone, VariantCount)]
pub enum Player {
    Black = 0,
    White = 1,
}

impl From<char> for Player {
    fn from(chr: char) -> Player {
        if chr.is_lowercase() {
            Player::Black
        } else {
            Player::White
        }
    }
}

#[test]
fn test_player_from_str() {
    assert_eq!(Player::from('Q'), Player::White);
    assert_eq!(Player::from('K'), Player::White);
    assert_eq!(Player::from('N'), Player::White);
    assert_eq!(Player::from('B'), Player::White);
    assert_eq!(Player::from('P'), Player::White);
    assert_eq!(Player::from('R'), Player::White);
    assert_eq!(Player::from('X'), Player::White);

    assert_eq!(Player::from('q'), Player::Black);
    assert_eq!(Player::from('k'), Player::Black);
    assert_eq!(Player::from('n'), Player::Black);
    assert_eq!(Player::from('b'), Player::Black);
    assert_eq!(Player::from('p'), Player::Black);
    assert_eq!(Player::from('r'), Player::Black);
    assert_eq!(Player::from('x'), Player::Black);
}
