#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

pub mod board;
pub mod piece;

// use board::Board;

fn main() {
    println!("Hello, world!");

    // let b = board::Board::new();

    let x = Some(1u8);

    if let Some(y) = x {
        println!("{:?}", y);
    }
}
