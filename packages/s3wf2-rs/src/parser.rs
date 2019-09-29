use crate::{
    document::{BlockNode, CharacterSet, CharacterType, Document},
    error::{Error, ErrorKind, SemanticErrorKind},
};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_LINE_HEAD: Regex = Regex::new(r"^(:|\/|@)([[:word:]]+)(\s+(.*))?$").unwrap();
    static ref REGEX_SPACES: Regex = Regex::new(r"\s+").unwrap();
    static ref REGEX_CHARACTER_ID: Regex = Regex::new(r"^[[:word:]]+$").unwrap();
    static ref REGEX_COLORCODE: Regex = Regex::new(r"^[[:xdigit:]]{3,6}$").unwrap();
    // MEMO: 後方一致が無いのでキャプチャーグループの終端でもってマッチを終了させる
    static ref REGEX_ELEMENT_LINE: Regex = Regex::new(r"\[(@?[[:word:]]+)[\]{\s]|[\]{}]").unwrap();
}

/// S3WF2 parser state.
pub struct Parser<'a> {
    characters: CharacterSet,
    blocks: Vec<BlockNode<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Parser<'a> {
        Parser {
            blocks: vec![],
            characters: CharacterSet::new(4),
        }
    }

    /// Parses text and append the result to held document.
    pub fn parse(&mut self, source: &'a str) -> Result<(), Error> {
        let lines = source.lines();

        for (index, line) in lines.enumerate() {
            let line_number = index + 1;
            let trimmed_line = line.trim();

            let re_result = REGEX_LINE_HEAD.captures(trimmed_line);
            let line_parse_result = if let Some(captures) = re_result {
                let cmd_type = captures.get(1).unwrap().as_str();
                let name = captures.get(2).unwrap().as_str();
                let rest = captures.get(2).map(|m| m.as_str());
                self.parse_command_line(cmd_type, name, rest)
            } else {
                self.parse_element_line(trimmed_line)
            };

            line_parse_result.map_err(|kind| Error { kind, line_number })?;
        }

        Ok(())
    }

    fn parse_command_line(
        &mut self,
        ctype: &'a str,
        name: &'a str,
        rest: Option<&'a str>,
    ) -> Result<(), ErrorKind> {
        match ctype {
            ":" => self.parse_meta_command(name, rest),
            "/" => self.parse_meta_block(name, rest),
            "@" => self.parse_meta_line(name, rest),
            // Not included in regex, therefore unreachable
            _ => unreachable!("Unexpected command type"),
        }
    }

    fn parse_element_line(&mut self, line: &'a str) -> Result<(), ErrorKind> {
        let rest = line;
        loop {
            let tag_captures = REGEX_ELEMENT_LINE.captures(line);
        }
        Ok(())
    }

    fn parse_meta_command(&mut self, name: &'a str, rest: Option<&'a str>) -> Result<(), ErrorKind> {
        let params: Option<Vec<_>> = rest.map(|raw| REGEX_SPACES.split(raw).collect());
        match name {
            "character" => {
                let params = params.ok_or(ErrorKind::NotEnoughParameters { given: 0, needed: 3})?;
                if params.len() < 3 {
                    Err(ErrorKind::NotEnoughParameters {
                        given: params.len(),
                        needed: 3,
                    })
                } else {
                    self.command_character(params[0], params[1], params[2])
                }
            }
            _ => Err(ErrorKind::UnknownCommand(name.to_string())),
        }
    }

    fn parse_meta_block(&mut self, element: &'a str, rest: Option<&'a str>) -> Result<(), ErrorKind> {
        Ok(())
    }

    fn parse_meta_line(&mut self, element: &'a str, rest: Option<&'a str>) -> Result<(), ErrorKind> {
        Ok(())
    }

    // command functions ------------------------------------------------------

    fn command_character(&mut self, kind: &'a str, id: &'a str, name: &'a str) -> Result<(), ErrorKind> {
        if !REGEX_CHARACTER_ID.is_match(id) {
            return Err(ErrorKind::Semantic(SemanticErrorKind::UndefinedCharacter(format!("{} (invalid ID)", id))));
        }

        match kind {
            "male" => self.characters.add_male(id, name).map_err(|kind| ErrorKind::Semantic(kind)),
            "female" => self.characters.add_female(id, name).map_err(|kind| ErrorKind::Semantic(kind)),
            "mob" => self.characters.add_mob(id, name).map_err(|kind| ErrorKind::Semantic(kind)),
            colorcode => {
                if !REGEX_COLORCODE.is_match(colorcode) {
                    Err(ErrorKind::Semantic(SemanticErrorKind::UndefinedCharacter(format!("{} (invalid colorcode {})", id, colorcode))))
                } else {
                    // TODO: とっとと実装しろ
                    unimplemented!()
                }
            },
        }
    }
}
