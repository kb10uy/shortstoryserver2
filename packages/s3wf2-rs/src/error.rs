use std::{error::Error as StdError, fmt};

/// Represents semantic error in parsing S3WF2.
#[derive(Debug)]
pub enum SemanticErrorKind {
    /// Undefined character ID appeared.
    UndefinedCharacter(String),

    /// Detected duplicate character ID.
    DuplicateCharacter(String),

    /// Surrounding tag is not supported for the `Element`.
    Nonsurrounding,

    /// Invalid parameter value was specified.
    InvalidParameter(String),
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
            SemanticErrorKind::InvalidParameter(reason) => {
                write!(f, "Invalid parameter ({})", reason)
            }
        }
    }
}

/// Represents any error in parsing S3WF2.
#[derive(Debug)]
pub enum ErrorKind {
    /// Tag opening appears too much.
    TooManyTagOpening,

    /// Tag closing appears too much.
    TooManyTagClosing,

    /// Wrong (bracket/brace) pair detected (like `{ ]` and `[ }`).
    InvalidParenPair,

    /// Wrong surrounding block pair detected. Maybe blank line exists in block.
    InvalidBlockPair,

    /// Parameters are insufficient.
    NotEnoughParameters { given: usize, needed: usize },

    /// Unknown command used.
    UnknownCommand(String),

    /// Unknown element (`BlockNode` or `ElementNode`) used.
    UnknownElement(String),

    /// Semantic error.
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
    /// The line number at which this error occurred.
    pub(crate) line_number: usize,

    /// Error type.
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
