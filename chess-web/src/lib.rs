use wasm_bindgen::prelude::*;
use js_sys::Array;

use chess_lib::chess::{Board, Piece, Player};

#[wasm_bindgen]
pub struct WebBoard {
    board: Board,
}

#[wasm_bindgen]
pub struct WebPiece {
    piece: Piece,
}

#[wasm_bindgen]
impl WebBoard {
    pub fn get_pieces(&self) -> Array {
        let arr = Array::new();

        for piece in self.board.iter() {
            // arr.push()
        }

        arr
    }
}

#[wasm_bindgen]
pub fn generate_board_from_string(board: &str, is_white: bool) -> Result<WebBoard, JsValue> {
    Ok(WebBoard {
        board: Board::from(
            board,
            if is_white {
                Player::White
            } else {
                Player::Black
            },
        )
        .map_err(|_| "Malformed Board")?,
    })
}
