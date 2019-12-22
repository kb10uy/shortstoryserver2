use crate::{
    document::{Block, BlockNode, CharacterSet, Document, Element, ElementNode, LineType},
    error::{Error, ErrorKind, SemanticErrorKind},
};
use lazy_static::lazy_static;
use regex::Regex;

const ASCII_WHITESPACES: &[char] = &[' ', '\t', '\r', '\n'];

lazy_static! {
    static ref REGEX_LINE_HEAD: Regex = Regex::new(r"^(:|/|@)([A-Za-z0-9_]+)(\s+(.*))?$").unwrap();
    static ref REGEX_SPACES: Regex = Regex::new(r"\s+").unwrap();
    static ref REGEX_CHARACTER_ID: Regex = Regex::new(r"^[A-Za-z0-9_]+$").unwrap();
    static ref REGEX_COLORCODE: Regex = Regex::new(r"^#([A-Fa-f0-9]{3,6})$").unwrap();
    // MEMO: regex crate does not support (?=expr) syntax, so use capture groups instead
    static ref REGEX_ELEMENT_LINE: Regex = Regex::new(r"\[(@?[A-Za-z0-9_]+)([\[\]{}]|\s+)|[\]{}]").unwrap();
}

/// Separated trimming implementations.
pub struct Trimmer;
impl Trimmer {
    /// Never trims.
    pub fn never(line: &str) -> &str {
        line
    }

    /// Trims only ASCII whitespaces.
    pub fn ascii_only(line: &str) -> &str {
        line.trim_matches(ASCII_WHITESPACES)
    }

    /// Trims all whitespaces.
    pub fn unicode(line: &str) -> &str {
        line.trim()
    }
}

/// Judges whether the parser should insert [br] element
/// ad the end of each line.
pub enum AutoNewline {
    /// Never inserts.
    Never,

    /// Always inserts.
    Always,
}

/// Represents misc. configuration for the document.
pub struct ParserState {
    pub trimming_function: fn(&str) -> &str,
    pub auto_newline: AutoNewline,
    pub block_line_type: LineType,
}

impl ParserState {
    /// Creates a new instance.
    pub fn new() -> ParserState {
        ParserState {
            trimming_function: Trimmer::unicode,
            auto_newline: AutoNewline::Never,
            block_line_type: LineType::NameShownBlock,
        }
    }
}

impl Default for ParserState {
    fn default() -> ParserState {
        ParserState::new()
    }
}

/// S3WF2 parser state.
#[derive(Default)]
pub struct Parser;

impl<'a> Parser {
    /// Creates a new instance.
    pub fn new() -> Parser {
        Parser {}
    }

    /// Parses text and append the result to held document.
    ///
    /// # Return value
    /// * `Ok(Document<'a>)` when parse completed successfully
    /// * `Err(Vec<Error>)` when some error detected
    ///     - Each item represents an error in single line
    pub fn parse(&self, source: &'a str) -> Result<Document<'a>, Vec<Error>> {
        let lines = source.lines();
        let mut state = ParserState::new();
        let mut errors = vec![];
        let mut document = Document::new();
        let mut current_block = BlockNode::new(Block::Paragraph);

        for (index, line) in lines.enumerate() {
            let line_number = index + 1;
            let trimmer = state.trimming_function;
            let trimmed_line = trimmer(line);
            if trimmed_line == "" && !current_block.is_empty() {
                document.blocks.push(current_block);
                current_block = BlockNode::new(Block::Paragraph);
                continue;
            }

            let re_result = REGEX_LINE_HEAD.captures(trimmed_line);
            let line_parse_result = if let Some(captures) = re_result {
                let cmd_type = captures.get(1).unwrap().as_str();
                let name = captures.get(2).unwrap().as_str();
                let rest = captures.get(4).map(|m| m.as_str());
                match cmd_type {
                    ":" => self.parse_command(&mut state, &mut document, name, rest),
                    "/" => {
                        let (next, error) =
                            self.parse_block(&mut document, current_block, name, rest);
                        current_block = next;
                        match error {
                            Some(kind) => Err(kind),
                            None => Ok(()),
                        }
                    }
                    "@" => self.parse_line(
                        &document.characters,
                        &mut current_block,
                        state.block_line_type,
                        name,
                        rest,
                    ),
                    // Not included in regex, therefore unreachable
                    _ => unreachable!("Unexpected command type"),
                }
            } else {
                self.parse_normal(&mut current_block.children, trimmed_line)
                    .and_then(|_| match state.auto_newline {
                        AutoNewline::Always if trimmed_line != "" => {
                            self.parse_normal(&mut current_block.children, "[br]")
                        }
                        _ => Ok(()),
                    })
            };

            if let Err(kind) = line_parse_result {
                errors.push(Error { line_number, kind });
            }
        }
        if !current_block.is_empty() {
            document.blocks.push(current_block);
        }

        if errors.is_empty() {
            Ok(document)
        } else {
            Err(errors)
        }
    }

    fn parse_command(
        &self,
        state: &mut ParserState,
        document: &mut Document,
        name: &'a str,
        rest: Option<&'a str>,
    ) -> Result<(), ErrorKind> {
        let params: Option<Vec<_>> = rest.map(|raw| REGEX_SPACES.split(raw).collect());
        match name {
            // :character <type> <id> <name>
            "character" => {
                let params = params.ok_or(ErrorKind::NotEnoughParameters {
                    given: 0,
                    needed: 3,
                })?;
                if params.len() < 3 {
                    return Err(ErrorKind::NotEnoughParameters {
                        given: params.len(),
                        needed: 3,
                    });
                }
                Command::command_character(
                    &mut document.characters,
                    params[0],
                    params[1],
                    params[2],
                )
            }
            "trim" => {
                let params = params.ok_or(ErrorKind::NotEnoughParameters {
                    given: 0,
                    needed: 1,
                })?;
                Command::command_trim(state, params[0])
            }
            "autobr" => {
                let params = params.ok_or(ErrorKind::NotEnoughParameters {
                    given: 0,
                    needed: 1,
                })?;
                Command::command_autobr(state, params[0])
            }
            "linename" => {
                let params = params.ok_or(ErrorKind::NotEnoughParameters {
                    given: 0,
                    needed: 1,
                })?;
                Command::command_linename(state, params[0])
            }
            _ => Err(ErrorKind::UnknownCommand(name.to_string())),
        }
    }

    fn parse_block(
        &self,
        document: &mut Document<'a>,
        current_block: BlockNode<'a>,
        element: &'a str,
        rest: Option<&'a str>,
    ) -> (BlockNode<'a>, Option<ErrorKind>) {
        let kind = match element.parse() {
            Ok(kind) => kind,
            Err(kind) => return (current_block, Some(kind)),
        };
        let previous_kind = current_block.kind;

        match rest {
            Some(">>>") => {
                if !current_block.is_empty() {
                    document.blocks.push(current_block);
                }
                (BlockNode::new(kind), None)
            }
            Some("<<<") => {
                if previous_kind != kind {
                    (current_block, Some(ErrorKind::InvalidBlockPair))
                } else {
                    if !current_block.is_empty() {
                        document.blocks.push(current_block);
                    }
                    (BlockNode::new(Block::Paragraph), None)
                }
            }

            Some(content) => {
                let mut block = BlockNode::new(kind);
                if let Err(kind) = self.parse_normal(&mut block.children, content) {
                    return (current_block, Some(kind));
                }
                if !current_block.is_empty() {
                    document.blocks.push(current_block);
                }
                document.blocks.push(block);
                (BlockNode::new(Block::Paragraph), None)
            }
            None => {
                if !current_block.is_empty() {
                    document.blocks.push(current_block);
                }
                document.blocks.push(BlockNode::new(kind));
                (BlockNode::new(Block::Paragraph), None)
            }
        }
    }

    fn parse_line(
        &self,
        characters: &CharacterSet,
        parent_block: &mut BlockNode<'a>,
        inline: LineType,
        element: &'a str,
        rest: Option<&'a str>,
    ) -> Result<(), ErrorKind> {
        let id = element.to_string();
        if characters.get(element).is_none() {
            return Err(ErrorKind::Semantic(SemanticErrorKind::UndefinedCharacter(
                id,
            )));
        }

        let mut children = vec![];
        self.parse_normal(&mut children, rest.unwrap_or(""))?;
        parent_block.children.push(ElementNode::Surrounded {
            kind: Element::Line(id, inline),
            parameters: vec![],
            children,
        });
        Ok(())
    }

    fn parse_normal(
        &self,
        parent: &mut Vec<ElementNode<'a>>,
        line: &'a str,
    ) -> Result<(), ErrorKind> {
        let mut uncommited: Vec<ElementNode<'a>> = vec![];
        let commited = parent;

        let mut rest = line;
        while let Some(captures) = REGEX_ELEMENT_LINE.captures(rest) {
            // before tag separation
            let whole_match = captures.get(0).unwrap();
            let leading_text = &rest[0..whole_match.start()];
            if leading_text != "" {
                let leading = ElementNode::Text(leading_text);
                // TODO: 本当はこれを何回も書きたくない
                if uncommited.is_empty() {
                    commited.push(leading);
                } else {
                    match uncommited.last_mut().unwrap() {
                        ElementNode::Surrounded { children, .. } => children.push(leading),
                        ElementNode::Text(_) => {
                            unreachable!("Text node must not be pushed as a tag")
                        }
                    }
                }
            }

            let next_rest_start = if let Some(tag_start) = captures.get(1) {
                let element = tag_start.as_str();
                if element.starts_with('@') {
                    // line element
                    uncommited.push(ElementNode::new_surrounded(Element::Line(
                        (&element[1..]).to_string(),
                        LineType::Inline,
                    )));
                } else {
                    // other
                    let kind = element.parse()?;
                    uncommited.push(ElementNode::new_surrounded(kind));
                }
                let ending = captures.get(2).unwrap();
                match ending.as_str() {
                    "[" | "]" | "{" => tag_start.end(),
                    "}" => return Err(ErrorKind::InvalidParenPair),
                    _ => ending.end(),
                }
            } else {
                match whole_match.as_str() {
                    "]" => {
                        let popped = uncommited.pop().ok_or(ErrorKind::TooManyTagClosing)?;
                        match popped {
                            ElementNode::Text(_) => {
                                unreachable!("Text node must not be pushed as a tag")
                            }
                            ElementNode::Surrounded {
                                kind: Element::Parameter,
                                ..
                            } => return Err(ErrorKind::InvalidParenPair),
                            _ => {
                                if uncommited.is_empty() {
                                    commited.push(popped);
                                } else {
                                    match uncommited.last_mut().unwrap() {
                                        ElementNode::Surrounded { children, .. } => {
                                            children.push(popped)
                                        }
                                        ElementNode::Text(_) => {
                                            unreachable!("Text node must not be pushed as a tag")
                                        }
                                    }
                                }
                            }
                        }
                    }
                    "{" => {
                        uncommited.push(ElementNode::new_surrounded(Element::Parameter));
                    }
                    "}" => {
                        let popped = uncommited.pop().ok_or(ErrorKind::TooManyTagClosing)?;
                        match popped {
                            ElementNode::Text(_) => {
                                unreachable!("Text node must not be pushed as a tag")
                            }
                            ElementNode::Surrounded {
                                kind: Element::Parameter,
                                ..
                            } => {
                                if uncommited.is_empty() {
                                    commited.push(popped);
                                } else {
                                    match uncommited.last_mut().unwrap() {
                                        ElementNode::Surrounded { parameters, .. } => {
                                            parameters.push(popped)
                                        }
                                        ElementNode::Text(_) => {
                                            unreachable!("Text node must not be pushed as a tag")
                                        }
                                    }
                                }
                            }
                            _ => return Err(ErrorKind::InvalidParenPair),
                        }
                    }
                    _ => unreachable!("Unexpected paren"),
                }
                whole_match.end()
            };
            rest = &rest[next_rest_start..];
        }

        if !uncommited.is_empty() {
            return Err(ErrorKind::TooManyTagOpening);
        }
        if rest != "" {
            commited.push(ElementNode::Text(rest));
        }
        Ok(())
    }
}

/// Separated command implementations.
struct Command;
impl Command {
    /// Process `:character` command.
    fn command_character(
        character_set: &mut CharacterSet,
        kind: &str,
        id: &str,
        name: &str,
    ) -> Result<(), ErrorKind> {
        if !REGEX_CHARACTER_ID.is_match(id) {
            return Err(ErrorKind::Semantic(SemanticErrorKind::UndefinedCharacter(
                format!("{} (invalid ID)", id),
            )));
        }

        match kind {
            "male" => character_set
                .add_male(id, name)
                .map_err(ErrorKind::Semantic),
            "female" => character_set
                .add_female(id, name)
                .map_err(ErrorKind::Semantic),
            "mob" => character_set.add_mob(id, name).map_err(ErrorKind::Semantic),
            colorcode => {
                if let Some(captured) = REGEX_COLORCODE.captures(colorcode) {
                    let color = captured.get(1).unwrap().as_str();
                    match color.len() {
                        3 | 6 => character_set
                            .add_custom(id, name, color)
                            .map_err(ErrorKind::Semantic),
                        _ => Err(ErrorKind::Semantic(SemanticErrorKind::UndefinedCharacter(
                            format!("{} (invalid colorcode {})", id, colorcode),
                        ))),
                    }
                } else {
                    Err(ErrorKind::Semantic(SemanticErrorKind::UndefinedCharacter(
                        format!("{} (invalid colorcode {})", id, colorcode),
                    )))
                }
            }
        }
    }

    /// Process `:trim` command.
    fn command_trim(state: &mut ParserState, trim_type: &str) -> Result<(), ErrorKind> {
        state.trimming_function = match trim_type {
            "never" => Trimmer::never,
            "ascii" => Trimmer::ascii_only,
            "unicode" => Trimmer::unicode,
            _ => {
                return Err(ErrorKind::Semantic(SemanticErrorKind::InvalidParameter(
                    format!("Invalid trimming type: {}", trim_type),
                )))
            }
        };

        Ok(())
    }

    /// Process `:autobr` command.
    fn command_autobr(state: &mut ParserState, autobr_type: &str) -> Result<(), ErrorKind> {
        state.auto_newline = match autobr_type {
            "never" => AutoNewline::Never,
            "always" => AutoNewline::Always,
            _ => {
                return Err(ErrorKind::Semantic(SemanticErrorKind::InvalidParameter(
                    format!("Invalid auto-br type: {}", autobr_type),
                )))
            }
        };

        Ok(())
    }

    /// Process `:linename` command.
    fn command_linename(state: &mut ParserState, linename_type: &str) -> Result<(), ErrorKind> {
        state.block_line_type = match linename_type {
            "shown" => LineType::NameShownBlock,
            "hidden" => LineType::NameHiddenBlock,
            _ => {
                return Err(ErrorKind::Semantic(SemanticErrorKind::InvalidParameter(
                    format!("Invalid line name visibility type: {}", linename_type),
                )))
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn trimmer_works() {
        assert_eq!(Trimmer::never("  ABC  "), "  ABC  ");
        assert_eq!(Trimmer::ascii_only(" 　ABC  "), "　ABC");
        assert_eq!(Trimmer::unicode(" 　ABC  "), "ABC");
    }

    #[test]
    fn parser_parses_character_command() {
        let parser = Parser::new();
        let mut document = Document::new();
        let mut state = ParserState::new();

        assert!(parser
            .parse_command(
                &mut state,
                &mut document,
                "character",
                Some("female natsuki 夏稀")
            )
            .is_ok());
        assert!(parser
            .parse_command(
                &mut state,
                &mut document,
                "character",
                Some("female natsuki")
            )
            .is_err());
        assert!(parser
            .parse_command(&mut state, &mut document, "character", Some("female"))
            .is_err());
        assert!(parser
            .parse_command(&mut state, &mut document, "character", None)
            .is_err());
    }

    #[test]
    fn parser_parses_trim_command() {
        let parser = Parser::new();
        let mut document = Document::new();
        let mut state = ParserState::new();

        assert!(parser
            .parse_command(&mut state, &mut document, "trim", Some("never"))
            .is_ok());
        assert!(parser
            .parse_command(&mut state, &mut document, "trim", Some("ascii"))
            .is_ok());
        assert!(parser
            .parse_command(&mut state, &mut document, "trim", Some("unicode"))
            .is_ok());
        assert!(parser
            .parse_command(&mut state, &mut document, "trim", None)
            .is_err());
    }

    #[test]
    fn parser_parses_autobr_command() {
        let parser = Parser::new();
        let mut document = Document::new();
        let mut state = ParserState::new();

        assert!(parser
            .parse_command(&mut state, &mut document, "autobr", Some("never"))
            .is_ok());
        assert!(parser
            .parse_command(&mut state, &mut document, "autobr", Some("always"))
            .is_ok());
        assert!(parser
            .parse_command(&mut state, &mut document, "autobr", None)
            .is_err());
    }

    #[test]
    fn parser_parses_block_element() {
        let parser = Parser::new();
        let mut document = Document::new();
        let current_block = BlockNode::new(Block::Paragraph);

        let (current_block, error) =
            parser.parse_block(&mut document, current_block, "sec", Some("Section"));
        assert!(error.is_none());
        assert_eq!(document.blocks.last().map(|b| b.kind), Some(Block::Section));

        let (current_block, error) =
            parser.parse_block(&mut document, current_block, "subsec", Some("Subsection"));
        assert!(error.is_none());
        assert_eq!(
            document.blocks.last().map(|b| b.kind),
            Some(Block::Subsection)
        );

        let (_, error) = parser.parse_block(&mut document, current_block, "notfound", None);
        assert!(error.is_some());
    }

    #[test]
    fn parser_parses_inline_element() {
        let parser = Parser::new();
        let mut current_block = BlockNode::new(Block::Paragraph);

        assert!(parser.parse_normal(&mut current_block.children, "").is_ok());
        assert!(current_block.children.is_empty());

        assert!(parser
            .parse_normal(&mut current_block.children, "test")
            .is_ok());
        assert_eq!(
            current_block.children.last(),
            Some(&ElementNode::Text("test"))
        );
        assert!(parser
            .parse_normal(&mut current_block.children, "[b hello]")
            .is_ok());
        assert_eq!(
            current_block.children.last(),
            Some(&ElementNode::Surrounded {
                kind: Element::Bold,
                parameters: vec![],
                children: vec![ElementNode::Text("hello")]
            }),
        );
    }
}
