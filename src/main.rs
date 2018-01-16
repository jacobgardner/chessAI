#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]


mod board;

use board::Board;

fn main() {
    println!("Hello, world!");

    let b = Board::new();

    let x = Some(1u8);

    if let Some(y) = x {
        println!("{:?}", y);
    }

}