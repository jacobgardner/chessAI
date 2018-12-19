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

pub const WHITE_ROOK_TEST: &'static str = "
    Rxxnxxxx
    xxxxxxrx
    xxxxxxxx
    xxxRxxxx
    xxxxRxxp
    xxxxxxxx
    xrxxxxxx
    xxxRxxxR
";

pub const BLACK_ROOK_TEST: &'static str = "
    rxxxNxxx
    xxxxxxxx
    xxxxxrxx
    xxxxxxxx
    xxrxxxxN
    xxxxxxxx
    xxxxrxxx
    xxxxNxrr
";

pub const WHITE_KNIGHT_TEST: &'static str = "
    Nxxnxxxx
    xxxxxxrx
    xxxxxxxx
    xxxNxxxx
    xxxxNxxp
    xxxxxxxx
    xrxxxxxx
    xxxNxxxN
";

pub const BLACK_KNIGHT_TEST: &'static str = "
    nxxxNxxx
    xxxxxxxx
    xxxxxnxx
    xxxxxxxx
    xxnxxxxN
    xxxxxxxx
    xxxxnxxx
    xxxxNxnn
";

