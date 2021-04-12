use std::fmt::{self, Display, Formatter};

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

#[derive(Debug, PartialEq)]
pub enum BoardError {
    InvalidPlayer { player_id: u8 },
    InvalidPiece { piece_id: u8 },
    MalformedBoard,
    OutOfBounds { rank: u8, file: u8 },
    InvalidString(InvalidStringReason),
}
