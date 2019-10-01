use std::{error::Error as StdError, fmt};

/// Represents semantic error in parsing S3WF2.
#[derive(Debug)]
pub enum SemanticErrorKind {
    UndefinedCharacter(String),
    DuplicateCharacter(String),
    Nonsurrounding,
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
            SemanticErrorKind::Nonsurrounding => write!(f, "Non-surrounding block detected"),
        }
    }
}

/// Represents any error in parsing S3WF2.
#[derive(Debug)]
pub enum ErrorKind {
    TooManyTagOpening,
    TooManyTagClosing,
    InvalidParenPair,
    InvalidBlockPair,
    NotEnoughParameters { given: usize, needed: usize },
    UnknownCommand(String),
    UnknownElement(String),
    Semantic(SemanticErrorKind),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::TooManyTagOpening => write!(f, "Too many tag opening '[' appear"),
            ErrorKind::TooManyTagClosing => write!(f, "Too many tag opening ']' appear"),
            ErrorKind::InvalidParenPair => write!(f, "Invalid brace/bracket pair detected"),
            ErrorKind::InvalidBlockPair => write!(f, "Invalid block pair detected"),
            ErrorKind::NotEnoughParameters { given, needed } => write!(
                f,
                "Not enough parameters ({} given, {} needed)",
                given, needed
            ),
            ErrorKind::UnknownCommand(cmd) => write!(f, "Unknown command: {}", cmd),
            ErrorKind::UnknownElement(elm) => write!(f, "Unknown element: {}", elm),
            ErrorKind::Semantic(kind) => write!(f, "Semantic error ({})", kind),
        }
    }
}

/// Represents an error of S3WF2 format parsing.
#[derive(Debug)]
pub struct Error {
    pub(crate) line_number: usize,
    pub(crate) kind: ErrorKind,
}

impl Error {
    /// The line number this error occured.
    pub fn line(&self) -> usize {
        self.line_number
    }

    /// The error reason.
    pub fn reason(&self) -> &ErrorKind {
        &self.kind
    }
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
