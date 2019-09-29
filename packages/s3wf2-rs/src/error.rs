use std::{error::Error as StdError, fmt};

/// Represents semantic error in parsing S3WF2.
#[derive(Debug)]
pub enum SemanticErrorKind {
    UndefinedCharacter(String),
    DuplicateCharacter(String),
}

impl fmt::Display for SemanticErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SemanticErrorKind::UndefinedCharacter(chara) => {
                write!(f, "Undefined character ID: {}", chara)
            }
            SemanticErrorKind::DuplicateCharacter(chara) => {
                write!(f, "Duplicate character ID: {}", chara)
            }
        }
    }
}

/// Represents any error in parsing S3WF2.
#[derive(Debug)]
pub enum ErrorKind {
    TooManyTagOpening,
    TooManyTagClosing,
    NotEnoughParameters { given: usize, needed: usize },
    Nonsurrounding,
    UnknownCommand(String),
    UnknownElement(String),
    Semantic(SemanticErrorKind),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::TooManyTagOpening => write!(f, "Too many tag opening '[' appear"),
            ErrorKind::TooManyTagClosing => write!(f, "Too many tag opening ']' appear"),
            ErrorKind::NotEnoughParameters { given, needed } => write!(
                f,
                "Not enough parameters ({} given, {} needed)",
                given, needed
            ),
            ErrorKind::Nonsurrounding => write!(f, "Non-surrounding block detected"),
            ErrorKind::UnknownCommand(cmd) => write!(f, "Unknown command: {}", cmd),
            ErrorKind::UnknownElement(elm) => write!(f, "Unknown element: {}", elm),
            ErrorKind::Semantic(kind) => write!(f, "Semantic error ({})", kind),
        }
    }
}

/// Represents an error of S3WF2 format parsing.
#[derive(Debug)]
pub struct Error {
    pub line_number: usize,
    pub kind: ErrorKind,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error at line {}: {}", self.line_number, self.kind)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}
