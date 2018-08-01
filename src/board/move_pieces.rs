use super::{Board, PieceType, Player, PIECE_COUNT};
use bitboard::{ROW_2, ROW_7};

use num;
use std::ops::{Generator, GeneratorState};

impl Board {
    pub fn generate_moves(&self, player: Player) -> impl Generator<Yield = Board, Return = ()> {
        let root_board = self.clone();

        move || {
            let player_mask = root_board.players[player as usize];
            let all_pieces = root_board.players[0] | root_board.players[1];

            for i in 0..PIECE_COUNT {
                let mut piecetype_mask = root_board.pieces[i as usize] & player_mask;

                let piece_type: PieceType = num::FromPrimitive::from_usize(i).unwrap();

                // We could do a tiny optimization here by just tracking
                //  when index == 64
                while piecetype_mask > 0 {
                    let index = piecetype_mask.trailing_zeros();
                    // println!("{} 0b{:0>64b}", index, piecetype_mask);

                    let piece_mask = 1 << index;

                    piecetype_mask &= !piece_mask;

                    match piece_type {
                        PieceType::Pawn => {
                            let mut available_moves: u64 = match player {
                                // I'm not too worred about this overflowing (vertical moves only)
                                //  because if it gets to the end it turns into a queen
                                Player::White => {
                                    (if piece_mask & ROW_2 > 0 {
                                        1 << (index + 16)
                                    } else {
                                        0
                                    }) | (1 << (index + 8))
                                }
                                Player::Black => {
                                    (if piece_mask & ROW_7 > 0 {
                                        1 << (index - 16)
                                    } else {
                                        0
                                    }) | (1 << (index - 8))
                                }
                            } & !all_pieces;

                            while available_moves > 0 {
                                let new_move = available_moves.trailing_zeros();
                                let new_move_mask = 1 << new_move;

                                let mut board = root_board.clone();
                                board.pieces[i] |= new_move_mask;
                                board.pieces[i] &= !piece_mask;
                                board.players[player as usize] |= new_move_mask;
                                board.players[player as usize] &= !piece_mask;

                                yield board;

                                available_moves &= !new_move_mask;
                            }

                            // println!("0b{:0>64b}", available_moves);
                        }
                        _ => {}
                    };
                }

                // yield root_board.clone();
            }
        }
    }
}

const PAWN_TEST: &'static str = "
    xxxrxxxx
    xxPxxxxx
    xxxxPxxx
    xxxxxxxx
    xnxxxxxx
    nxxxxxxx
    xPxxxPxx
    xxxxxxxx
    ";

#[test]
fn test_generate_moves() {
    let board = Board::from(PAWN_TEST).unwrap();

    let mut generator = board.generate_moves(Player::White);

    loop {
        let new_board = match unsafe { generator.resume() } {
            GeneratorState::Yielded(board) => board,
            GeneratorState::Complete(_) => break,
        };

        println!("{}", new_board);
    }
}
