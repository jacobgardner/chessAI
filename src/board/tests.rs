#![cfg(test)]

use super::*;

#[test]
fn test_check() {
    let board = ChessBoard::from_ascii(
        "
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    xxxxxbxx
    xxxxxxxx
    xRxxxxxx
    xxxxxxxx
    ",
    ).unwrap();

    assert_eq!(board.is_capturable(&Position(0, 0), &Black), false);
    assert_eq!(board.is_capturable(&Position(0, 1), &Black), true);
    assert_eq!(board.is_capturable(&Position(7, 1), &Black), true);
    assert_eq!(board.is_capturable(&Position(7, 2), &Black), false);
    assert_eq!(board.is_capturable(&Position(2, 7), &Black), false);
    assert_eq!(board.is_capturable(&Position(1, 7), &Black), true);

    assert_eq!(board.is_capturable(&Position(0, 0), &White), false);
    assert_eq!(board.is_capturable(&Position(0, 1), &White), false);
    assert_eq!(board.is_capturable(&Position(7, 1), &White), true);
    assert_eq!(board.is_capturable(&Position(7, 2), &White), false);
    assert_eq!(board.is_capturable(&Position(2, 7), &White), false);
    assert_eq!(board.is_capturable(&Position(1, 7), &White), true);

    assert_eq!(board.is_capturable(&Position(3, 1), &White), true);
    assert_eq!(board.is_capturable(&Position(1, 3), &White), false);
    assert_eq!(board.is_capturable(&Position(7, 5), &White), true);

    let board = ChessBoard::from_ascii(
        "
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    xxxNxxxx
    xxxxxxxx
    xxxxxxxx
    ",
    ).unwrap();

    assert_eq!(board.is_capturable(&Position(0, 0), &Black), false);
    assert_eq!(board.is_capturable(&Position(0, 2), &Black), false);
    assert_eq!(board.is_capturable(&Position(1, 1), &Black), true);
}

#[test]
fn test_from_ascii() {
    // Test typical board.
    let board = ChessBoard::from_ascii(
        "
    RNBQKBNR
    PPPPPPPP
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    pppppppp
    rnbqkbnr
    ",
    ).unwrap();

    #[rustfmt_skip]
    {
        assert_eq!(board.pieces[0], Some(Piece::new(Rook, White)));
        assert_eq!(board.pieces[7], Some(Piece::new(Rook, White)));
        assert_eq!(board.pieces[1], Some(Piece::new(Knight, White)));
        assert_eq!(board.pieces[6], Some(Piece::new(Knight, White)));
        assert_eq!(board.pieces[2], Some(Piece::new(Bishop, White)));
        assert_eq!(board.pieces[5], Some(Piece::new(Bishop, White)));
        assert_eq!(board.pieces[3], Some(Piece::new(Queen, White)));
        assert_eq!(board.pieces[4], Some(Piece::new(King, White)));
    }

    for i in (8..16).chain(48..56) {
        let owner = if i > 16 { Black } else { White };
        assert_eq!(board.pieces[i], Some(Piece::new(Pawn, owner,)));
    }

    #[rustfmt_skip]
    {
        assert_eq!(board.pieces[56], Some(Piece::new(Rook, Black)));
        assert_eq!(board.pieces[63], Some(Piece::new(Rook, Black)));
        assert_eq!(board.pieces[57], Some(Piece::new(Knight, Black)));
        assert_eq!(board.pieces[62], Some(Piece::new(Knight, Black)));
        assert_eq!(board.pieces[58], Some(Piece::new(Bishop, Black)));
        assert_eq!(board.pieces[61], Some(Piece::new(Bishop, Black)));
        assert_eq!(board.pieces[59], Some(Piece::new(Queen, Black)));
        assert_eq!(board.pieces[60], Some(Piece::new(King, Black)));
    }

    for i in 16..48 {
        assert_eq!(board.pieces[i], None);
    }

    // Too Few spaces
    let board2 = ChessBoard::from_ascii(
        "
    RNBQKBNR
    PPPPPPP
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    pppppppp
    rnbqkbnr
    ",
    );

    assert_eq!(board2, Err(()));

    // Too many spaces
    let board3 = ChessBoard::from_ascii(
        "
    RNBQKBNR
    PPPPPPPPP
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    pppppppp
    rnbqkbnr
    ",
    );

    assert_eq!(board3, Err(()));
}

#[test]
fn test_generate_boards() {
    let board = ChessBoard::from_ascii(
        "
    xxxxxxxx
    xPxxxxPx
    xxxxxxxx
    xpxxxxxP
    xxxpxxxp
    Pxxxxbxx
    xpxxxxxx
    xxxpxxrx
    ",
    ).unwrap();

    println!("WHITE!");

    for sub_board in board.generate_moves(&White) {
        println!("{}", sub_board);
    }

    println!("BLACK!");

    for sub_board in board.generate_moves(&Black) {
        println!("{}", sub_board);
    }
}
