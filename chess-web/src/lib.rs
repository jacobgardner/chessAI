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
enum Msg {
    NextBoard,
}

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

fn update(msg: Msg, mut model: Model) -> Model {
    match msg {
        Msg::NextBoard => model.board = model.board.generate_moves().nth(0).unwrap(),
    }

    model
}

fn view(_state: seed::App<Msg, Model>, model: Model) -> El<Msg> {
    // for board in model.possible_moves.iter() {
    console::log_1(&format!("{}", model.board).into());
    // }

    let board_style = style! {
        "background-color" => "#FFFFFF"
    };

    let rank_style = style! {
        "display" => "flex";
        "flex-direction" => "row";
    };

    let space_style = |space_id: u8| {
        style! {
            "width" => "100px";
            "height" => "100px";
            "display" => "flex";
            "justify-content" => "center";
            "align-items" => "center";
            "font-size" => "60px";
            "background-color" => {
                if space_id % 2 == 0 {
                    "white"
                } else {
                    "teal"
                }
            };
        }
    };

    div!(
        div!(
            &board_style,
            (0..8)
                .map(|r| div!(
                    &rank_style,
                    (0..8)
                        .map(|f| div!(
                            space_style(r + f),
                            if let Some(piece) = model.board.piece_at(7 - r, f).unwrap() {
                                piece.to_char().to_string()
                            } else {
                                " ".to_owned()
                            }
                        ))
                        .collect::<Vec<_>>()
                ))
                .collect::<Vec<_>>()
        ),
        button!(simple_ev("click", Msg::NextBoard), "next")
    )
}

#[wasm_bindgen]
pub fn mount() {
    seed::run(Model::default(), update, view, "main", None, None);
}
