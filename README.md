# Chess AI [![Build Status](https://travis-ci.org/jacobgardner/chessAI.svg?branch=master)](https://travis-ci.org/jacobgardner/chessAI)

To benchmark, run:

    cargo bench

A new attempt at a Chess AI in Rust.  This attempt will try to achieve the following goals:

- [x] Using proper bitboards for move generation
    - [x] Pawn
        - [x] Single Move
        - [x] Double Move
        - [x] Normal Captures
        - [x] En Passant
    - [x] Rook
        - [x] Horizontal Moves
        - [x] Vertical Moves
    - [x] Knight
    - [x] Bishop
    - [x] Queen
    - [x] King
    - [x] Castling
- [ ] Web frontend for seeing chess boards
- [ ] Using some sort of Arena for the tree
- [ ] Minimax w/ alpha-beta pruning
- [ ] Iterative depth first search
- [ ] Adaptive search depth over the course of the game and how much time is remaining

Stretch Goals:

- [ ] Hooking up to Chess.com API to play against
- [ ] Setting up tournament server for other AI to play against/rank.
- [ ] Using lookups for boards to choose best move without searching (especially at the start)
- [ ] Using a genetic algorithm to tune parameters
  - [ ] Piece Values
  - [ ] How deep to search at any given time


Possible Optimizations:
- [ ] Using premade boards for a lot of masks.

    NOTE: This actually may end up slower because the frequent lookups may thrash the cache

  - [ ] 64 knight move pattern boards
  - [ ] 64 bishop move pattern boards
  - [ ] 64 rook move pattern boards
  - [ ] 65 king move pattern boards
- [ ] Use opening move database for initial moves
