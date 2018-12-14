use crate::chess::Move;
use crate::chess::PieceType;
use crate::chess::RankFile;

pub const WHITE_PAWN_TEST: &'static str = "
    xxxrxxxx
    xxPxxxxx
    xxxxxxxx
    xxxxPpxx
    xnxnxxxx
    nxPxxxxn
    xPxxxPxP
    xxxxxxxx
    ";

pub const WHITE_EN_PASSANT: Move = Move {
    piece_type: PieceType::Pawn,
    from: RankFile::F7,
    to: RankFile::F5,
};

pub const BLACK_PAWN_TEST: &'static str = "
    xxxxxxxx
    pxxxxxpx
    NxxxxNxN
    Nxpxxxxx
    xxxPpxxx
    xxxxxxxx
    xxxxpNxx
    xxxNxxxx
    ";

pub const BLACK_EN_PASSANT: Move = Move {
    piece_type: PieceType::Pawn,
    from: RankFile::D2,
    to: RankFile::D4,
};
