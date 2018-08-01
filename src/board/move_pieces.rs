use super::{Board, PieceType, Player, PIECE_COUNT};
use bitboard::{ROW_2, ROW_7};

use num;
use std::ops::{Generator, GeneratorState};

impl Board {
    pub fn generate_moves(&self, player: Player) -> impl Generator<Yield = Board, Return = ()> {
        let root_board = self.clone();

        move || {
            let player_mask = root_board.players[player as usize];
            let enemy_mask = root_board.players[1 - (player as usize)];
            let all_pieces = player_mask | enemy_mask;

            for i in 0..PIECE_COUNT {
                let mut piecetype_mask = root_board.pieces[i as usize] & player_mask;

                let piece_type: PieceType = num::FromPrimitive::from_usize(i).unwrap();

                // We could do a tiny optimization here by just tracking
                //  when index == 64
                while piecetype_mask > 0 {
                    let index = piecetype_mask.trailing_zeros();
                    let piece_mask = 1 << index;
                    let piece_inverse = !piece_mask;

                    piecetype_mask &= piece_inverse;

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

                            let mut available_captures: u64 = match player {
                                Player::White => {
                                    (if index % 8 != 0 { 1 << (index + 9) } else { 0 })
                                        | (if index % 8 != 7 { 1 << (index + 7) } else { 0 })
                                }
                                Player::Black => 0,
                            } & enemy_mask;

                            while available_moves > 0 {
                                let new_move = available_moves.trailing_zeros();
                                let new_move_mask = 1 << new_move;

                                let mut board = root_board.clone();
                                board.pieces[i] |= new_move_mask;
                                board.pieces[i] &= piece_inverse;
                                board.players[player as usize] |= new_move_mask;
                                board.players[player as usize] &= piece_inverse;

                                yield board;

                                available_moves &= !new_move_mask;
                            }

                            while available_captures > 0 {
                                let new_move = available_captures.trailing_zeros();
                                let new_move_mask = 1 << new_move;
                                let inverse_move = !new_move_mask;

                                let mut board = root_board.clone();
                                board.pieces[i] |= new_move_mask;
                                board.pieces[i] &= piece_inverse;
                                board.players[player as usize] |= new_move_mask;
                                board.players[player as usize] &= piece_inverse;
                                board.players[1 - (player as usize)] &= inverse_move;

                                yield board;
                                available_captures &= inverse_move;

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
    xnxnxxxx
    nxPxxxxx
    xPxxxPxP
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
