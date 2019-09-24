use crate::document::{CharacterSet, CharacterType, Document, DocumentError, BlockNode};
use regex::Regex;

pub enum ParseError {
    TooManyOpen,
    TooManyClose,
    UnknownCommand,
    UnknownElement,
    UnknownCharacter,
    InDocument(DocumentError),
}

pub struct Parser<'a> {
    re_line_head: Regex,

    characters: CharacterSet,
    blocks: Vec<BlockNode<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Parser<'a> {
        Parser {
            re_line_head: Regex::new(r"^(:|\/|@)(\w+)(\s+(.*))?$").unwrap(),
            blocks: vec![],
            characters: CharacterSet::new(4),
        }
    }

    /// Parses text and append the result to held document.
    pub fn parse(&self, source: &'a str) -> Result<(), ParseError> {
        let lines = source.lines();

        for (index, line) in lines.enumerate() {
            let line_number = index + 1;
            let trimmed_line = line.trim();

            let re_result = self.re_line_head.captures(trimmed_line);
            if let Some(captures) = re_result {

            } else {

            }
        }

        Ok(())
    }

    fn parse_element_line(&self, line: &'a str) -> Result<(), ParseError> {
        Ok(())
    }
}
