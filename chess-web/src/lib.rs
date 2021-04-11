use wasm_bindgen::prelude::*;

use js_sys::Function;

use chess_lib::chess;

#[wasm_bindgen]
pub struct Board {
    board: chess::Board,
}

#[wasm_bindgen]
pub struct Piece {
    piece: chess::Piece,
    rank_file: chess::RankFile,
}

#[wasm_bindgen]
impl Board {
    pub fn generate_moves(&self) -> Vec<JsValue> {
        self.board
            .generate_moves()
            .map(|b| Board { board: b })
            .map(JsValue::from)
            .collect()
    }

    pub fn get_pieces(&self, fun: &Function) -> Result<(), JsValue> {
        let this = JsValue::NULL;

        for (rank_file, piece) in self.board.iter() {
            let web_piece = Piece { piece, rank_file };

            let x = JsValue::from(web_piece);

            fun.call1(&this, &x)
                .map_err(|_| "Unable to call callback")?;
        }

        Ok(())
    }

    pub fn from(board: &str, is_white: bool) -> Result<Board, JsValue> {
        Ok(Board {
            board: chess::Board::from(
                board,
                if is_white {
                    chess::Player::White
                } else {
                    chess::Player::Black
                },
            )
            .map_err(|_| "Malformed Board")?,
        })
    }
}

#[wasm_bindgen]
impl Piece {
    pub fn rank(&self) -> u8 {
        self.rank_file.rank()
    }

    pub fn file(&self) -> u8 {
        self.rank_file.file()
    }

    #[wasm_bindgen(js_name = type)]
    pub fn piece_type(&self) -> char {
        self.piece.to_char()
    }

    pub fn owner(&self) -> char {
        match self.piece.player {
            chess::Player::White => 'w',
            chess::Player::Black => 'b',
        }
    }
}
