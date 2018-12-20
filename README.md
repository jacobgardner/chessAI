Chess AI
========

To benchmark, run:

    cargo bench

A new attempt at a Chess AI in Rust.  This attempt will try to achieve the following goals:

- [ ] Using proper bitboards for move generation
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
    - [ ] Queen
    - [ ] King
    - [ ] Castling
- [ ] Using some sort of Arena for the tree
- [ ] Minimax w/ alpha-beta pruning
- [ ] Iterative depth first search
- [ ] Adaptive search depth over the course of the game and how much time is remaining

Stretch Goals:

- [ ] Hooking up to Chess.com API to play against
- [ ] Setting up tournament server for other AI to play against/rank.
- [ ] Using lookups for boards to choose best move without searching (especially at the start)
- [ ] Uisng an evolutionary algorithm to tune parameters
  - [ ] Piece Values
  - [ ] How deep to search at any given time