use position::Position;

pub const ROOK_MOVE: [Position; 4] = [
    Position(-1, 0),
    Position(1, 0),
    Position(0, -1),
    Position(0, 1),
];

pub const BISHOP_MOVE: [Position; 4] = [
    Position(-1, -1),
    Position(1, 1),
    Position(1, -1),
    Position(-1, 1),
];

pub const KNIGHT_MOVE: [Position; 8] = [
    Position(-2, -1),
    Position(-2, 1),
    Position(-1, 2),
    Position(1, 2),
    Position(2, 1),
    Position(2, -1),
    Position(1, -2),
    Position(-1, -2),
];

pub const QUEEN_MOVE: [Position; 8] = [
    Position(-1, 0),
    Position(-1, 1),
    Position(0, 1),
    Position(1, 1),
    Position(1, 0),
    Position(1, -1),
    Position(0, -1),
    Position(-1, -1),
];
