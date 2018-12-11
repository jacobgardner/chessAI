use std::fmt::{self, Display, Formatter};


// TODO: Do we need to use failure?

#[derive(Debug, PartialEq)]
pub enum InvalidStringReason {
    IncorrectLength,
    NonAsciiChars,
}

impl Display for InvalidStringReason {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            InvalidStringReason::IncorrectLength => write!(
                formatter,
                "Expected string to have exactly 64 non-space characters"
            ),
            InvalidStringReason::NonAsciiChars => {
                write!(formatter, "Detected 1 or more non-ascii characters")
            }
        }
    }
}

#[derive(Debug, Fail, PartialEq)]
pub enum BoardError {
    #[fail(display = "invalid player id: {}", player_id)]
    InvalidPlayer { player_id: u8 },

    #[fail(display = "invalid piece id: {}", piece_id)]
    InvalidPiece { piece_id: u8 },

    #[fail(display = "Bit found on player mask, but no board masks")]
    MalformedBoard,

    #[fail(display = "Rank/File exceeded board limits: {} {}", rank, file)]
    OutOfBounds { rank: u8, file: u8 },

    #[fail(display = "Malformed string for board: {}", _0)]
    InvalidString(InvalidStringReason),
}
