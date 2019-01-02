use wasm_bindgen::prelude::*;

#[macro_use]
extern crate seed;
// #[macro_use] extern crate lazy_static;

use chess_lib::chess::{Board, Player, DEFAULT_BOARD};
use seed::prelude::*;
use web_sys::console;

#[derive(Clone)]
struct Model {
    board: Board,
    possible_moves: Vec<Board>,
}

#[derive(Clone)]
enum Msg {}

impl Default for Model {
    fn default() -> Self {
        let board = Board::from(DEFAULT_BOARD, Player::White).unwrap();
        let possible_moves = board.generate_moves().collect();

        Self {
            board,
            possible_moves,
        }
    }
}

fn update(_msg: Msg, model: Model) -> Model {
    model
}

fn view(_state: seed::App<Msg, Model>, model: Model) -> El<Msg> {

    for board in model.possible_moves.iter() {
        console::log_1(&format!("{}", board).into());
    }

    let chess_style = style! {
        "background-color" => "#000000"
    };

    div!(&chess_style,)
}

#[wasm_bindgen]
pub fn mount() {
    seed::run(Model::default(), update, view, "main", None, None);
}
