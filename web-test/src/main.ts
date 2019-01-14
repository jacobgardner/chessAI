const DEFAULT_BOARD = `
    rnbkqbnr
    pppppppp
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    PPPPPPPP
    RNBKQBNR
`;

import {Piece} from '../../chess-web/pkg';

import("../../chess-web/pkg").then(chess => {
    const board = chess.Board.from(DEFAULT_BOARD, true);
    // const board = chess.generate_board_from_string(DEFAULT_BOARD, true);

    try {
        chess.Board.from("poop", false);
    } catch (err) {
        console.log("Fuck");
    }

    console.log(board);
    board.get_pieces((piece: Piece) => {
        console.log(piece.rank(), piece.file(), piece.type(), piece.owner())
        piece.free();
    });

    board.free();
});
