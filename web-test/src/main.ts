const DEFAULT_BOARD = `
    rnbkqbnr
    pppppppp
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    xxxxxxxx
    PPPPPPPP
    RNBKQBNR
`;

import type * as ChessType from "../../chess-web/pkg";

let cachedValues: any;
let chess: typeof ChessType;

enum Owner {
  White = "w",
  Black = "b",
}

enum PieceType {
  King = "k",
  Queen = "q",
  Bishop = "b",
  Knight = "n",
  Rook = "r",
  Pawn = "p",
}

class Piece {
  constructor(
    public owner: Owner,
    public rank: number,
    public file: number,
    public type: PieceType
  ) {}

  toUnicode(): string {
    switch (this.type) {
      case PieceType.King:
        return this.owner === Owner.White ? "♔" : "♚";
      case PieceType.Queen:
        return this.owner === Owner.White ? "♕" : "♛";
      case PieceType.Rook:
        return this.owner === Owner.White ? "♖" : "♜";
      case PieceType.Bishop:
        return this.owner === Owner.White ? "♗" : "♝";
      case PieceType.Knight:
        return this.owner === Owner.White ? "♘" : "♞";
      case PieceType.Pawn:
        return this.owner === Owner.White ? "♙" : "♟";
    }
  }
}

class Board {
  static from(
    board: string | ChessType.Board,
    isCurrentPlayerWhite = true
  ): Board {
    let boardRs: ChessType.Board;
    if (typeof board === "string") {
      boardRs = chess.Board.from(board, isCurrentPlayerWhite);
    } else {
      boardRs = board;
    }

    const pieces: Piece[] = [];

    boardRs.get_pieces((piece: ChessType.Piece) => {
      pieces.push(
        new Piece(
          piece.owner() as Owner,
          piece.rank(),
          piece.file(),
          piece.type().toLowerCase() as PieceType
        )
      );

      piece.free();
    });

    console.log("Freeing");

    boardRs.free();

    return new Board(pieces, isCurrentPlayerWhite);
  }

  public boardString: string;

  private constructor(
    public pieces: Piece[],
    public isCurrentPlayerWhite = true
  ) {
    this.boardString = "";

    for (let rank = 7; rank >= 0; rank -= 1) {
      for (let file = 0; file < 8; file += 1) {
        const piece = this.getAt(rank, file);

        if (piece) {
          const type = piece.type;
          this.boardString +=
            piece.owner === Owner.White
              ? type.toUpperCase()
              : type.toLowerCase();
        } else {
          this.boardString += ".";
        }
      }

      this.boardString += "\n";
    }
  }

  getAt(rank: number, file: number): Piece | undefined {
    const piece = this.pieces.find(
      (piece) => piece.rank === rank && piece.file === file
    );

    return piece;
  }

  generate_boards() {
    const boards = chess.Board.from(
      this.boardString,
      this.isCurrentPlayerWhite
    ).generate_moves();

    return boards.map((board) => Board.from(board, !this.isCurrentPlayerWhite));
  }
}

export async function build(): Promise<{ Board: typeof Board }> {
  if (cachedValues) {
    return cachedValues;
  }

  chess = await import("../../chess-web/pkg");

  const rVal = {
    Board,
  };

  cachedValues = rVal;

  return rVal;
}

const PIECE_LOOKUP = {};

function depictionFromBoard(board: Board): HTMLElement {
  const container = document.createElement("div");
  container.className = "board";

  for (let rank = 7; rank >= 0; rank -= 1) {
    for (let file = 0; file < 8; file += 1) {
      const space = document.createElement("div");
      const spaceType = (rank + file) % 2 === 1 ? "odd" : "even";
      space.className = `board-space board-space_${spaceType}`;

      const piece = board.getAt(rank, file);

      if (piece) {
        space.innerText = piece.toUnicode();
      }

      container.appendChild(space);
    }
  }

  return container;
}

build().then((api) => {
  const board = api.Board.from(DEFAULT_BOARD, true);
  const boardContainer = depictionFromBoard(board);
  document.body.appendChild(boardContainer);
  console.log(board.pieces);

  for (const b of board.generate_boards()) {
    const boardContainer = depictionFromBoard(b);
    document.body.appendChild(boardContainer);
  }
});
