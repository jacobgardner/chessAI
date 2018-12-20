use crate::chess::Move;
use crate::chess::PieceType;
use crate::chess::RankFile;

pub const WHITE_PAWN_TEST: &str = "
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

pub const BLACK_PAWN_TEST: &str = "
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

pub const WHITE_ROOK_TEST: &str = "
    Rxxnxxxx
    xxxxxxrx
    xxxxxxxx
    xxxRxxxx
    xxxxRxxp
    xxxxxxxx
    xrxxxxxx
    xxxRxxxR
";

pub const BLACK_ROOK_TEST: &str = "
    rxxxNxxx
    xxxxxxxx
    xxxxxrxx
    xxxxxxxx
    xxrxxxxN
    xxxxxxxx
    xxxxrxxx
    xxxxNxrr
";

pub const WHITE_KNIGHT_TEST: &str = "
    Nxxnxxxx
    xxxxxxrx
    xxxxxxxx
    xxxNxxxx
    xxxxNxxp
    xxxxxxxx
    xrxxxxxx
    xxxNxxxN
";

pub const BLACK_KNIGHT_TEST: &str = "
    nxxxNxxx
    xxxxxxxx
    xxxxxnxx
    xxxxxxxx
    xxnxxxxN
    xxxxxxxx
    xxxxnxxx
    xxxxNxnn
";

pub const WHITE_BISHOP_TEST: &str = "
    Bxxnxxxx
    xxxxxxrx
    xxxxxxxx
    xxxBxxxx
    xxxxBxxp
    xrxxxxxx
    xrxxxxxx
    xxxBxxxB
";

pub const BLACK_BISHOP_TEST: &str = "
    bxxxNxxx
    xxxxxxxx
    xxxxxbxx
    xxxxxxxx
    xxbxxxxN
    xxxxxRxx
    xxxxbxxx
    xxxxNxbb
";

pub const WHITE_QUEEN_TEST: &str = "
    Qxxnxxxx
    xxxxxxrx
    xxxxxxxx
    xxxQxxxx
    xxxxQxxp
    xrxxxxxx
    xrxxxxxx
    xxxQxxxQ
";

pub const BLACK_QUEEN_TEST: &str = "
    qxxxNxxx
    xxxxxxxx
    xxxxxqxx
    xxxxxxxx
    xxqxxxxN
    xxxxxRxx
    xxxxqxxx
    xxxxNxqq
";
