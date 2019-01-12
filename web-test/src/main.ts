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

import('../../chess-web/pkg').then((chess) => {
    const board = chess.generate_board_from_string(DEFAULT_BOARD, true);

    try {
        chess.generate_board_from_string("poop", false);
    } catch(err) {
        console.log('Fuck');
    }

    console.log(board);
    board.get_pieces();

    board.free();
});
