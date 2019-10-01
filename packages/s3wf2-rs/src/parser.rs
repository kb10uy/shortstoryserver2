use crate::{
    document::{Block, BlockNode, CharacterSet, Document, Element, ElementNode},
    error::{Error, ErrorKind, SemanticErrorKind},
};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_LINE_HEAD: Regex = Regex::new(r"^(:|/|@)([[:word:]]+)(\s+(.*))?$").unwrap();
    static ref REGEX_SPACES: Regex = Regex::new(r"\s+").unwrap();
    static ref REGEX_CHARACTER_ID: Regex = Regex::new(r"^[[:word:]]+$").unwrap();
    static ref REGEX_COLORCODE: Regex = Regex::new(r"^[[:xdigit:]]{3,6}$").unwrap();
    // MEMO: 後方一致が無いのでキャプチャーグループの終端でもってマッチを終了させる
    static ref REGEX_ELEMENT_LINE: Regex = Regex::new(r"\[(@?[[:word:]]+)([\[\]{}]|\s+)|[\]{}]").unwrap();
}

/// S3WF2 parser state.
pub struct Parser {}

impl<'a> Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    /// Parses text and append the result to held document.
    pub fn parse(&mut self, source: &'a str) -> Result<Document<'a>, Error> {
        let lines = source.lines();
        let mut document = Document::new();
        let mut current_block = BlockNode::new(Block::Paragraph);

        for (index, line) in lines.enumerate() {
            let line_number = index + 1;
            let trimmed_line = line.trim();
            let error_mapper = |kind| Error { kind, line_number };

            let re_result = REGEX_LINE_HEAD.captures(trimmed_line);
            let line_parse_result = if let Some(captures) = re_result {
                let cmd_type = captures.get(1).unwrap().as_str();
                let name = captures.get(2).unwrap().as_str();
                let rest = captures.get(2).map(|m| m.as_str());
                match cmd_type {
                    ":" => self.parse_command(&mut document, name, rest),
                    "/" => {
                        current_block = self
                            .parse_block(&mut document, current_block, name, rest)
                            .map_err(error_mapper)?;
                        Ok(())
                    }
                    "@" => self.parse_line(&document.characters, &mut current_block, name, rest),
                    // Not included in regex, therefore unreachable
                    _ => unreachable!("Unexpected command type"),
                }
            } else {
                self.parse_normal(&mut current_block.children, trimmed_line)
            };

            line_parse_result.map_err(error_mapper)?;
        }

        Ok(document)
    }

    fn parse_command(
        &mut self,
        document: &mut Document,
        name: &'a str,
        rest: Option<&'a str>,
    ) -> Result<(), ErrorKind> {
        let params: Option<Vec<_>> = rest.map(|raw| REGEX_SPACES.split(raw).collect());
        match name {
            "character" => {
                let params = params.ok_or(ErrorKind::NotEnoughParameters {
                    given: 0,
                    needed: 3,
                })?;
                if params.len() < 3 {
                    Err(ErrorKind::NotEnoughParameters {
                        given: params.len(),
                        needed: 3,
                    })
                } else {
                    self.command_character(
                        &mut document.characters,
                        params[0],
                        params[1],
                        params[2],
                    )
                }
            }
            _ => Err(ErrorKind::UnknownCommand(name.to_string())),
        }
    }

    fn parse_block(
        &mut self,
        document: &mut Document<'a>,
        current_block: BlockNode<'a>,
        element: &'a str,
        rest: Option<&'a str>,
    ) -> Result<BlockNode<'a>, ErrorKind> {
        let kind = Parser::parse_block_kind(element)?;
        let previous_kind = current_block.kind;
        if !current_block.is_empty() {
            document.blocks.push(current_block);
        }

        match rest {
            Some(">>>") => Ok(BlockNode::new(kind)),
            Some("<<<") => {
                if previous_kind != kind {
                    Err(ErrorKind::InvalidBlockPair)
                } else {
                    Ok(BlockNode::new(Block::Paragraph))
                }
            }

            Some(content) => {
                let mut block = BlockNode::new(kind);
                self.parse_normal(&mut block.children, content)?;
                document.blocks.push(block);
                Ok(BlockNode::new(Block::Paragraph))
            }
            None => {
                document.blocks.push(BlockNode::new(kind));
                Ok(BlockNode::new(Block::Paragraph))
            }
        }
    }

    fn parse_block_kind(name: &'a str) -> Result<Block, ErrorKind> {
        match name {
            "para" => Ok(Block::Paragraph),
            "sec" => Ok(Block::Section),
            "subsec" => Ok(Block::Subsection),
            "quote" => Ok(Block::Quotation),
            "hori" => Ok(Block::Horizontal),
            "list" => Ok(Block::UnorderedList),
            _ => Err(ErrorKind::UnknownElement(name.to_string())),
        }
    }

    fn parse_line(
        &mut self,
        characters: &CharacterSet,
        parent_block: &mut BlockNode<'a>,
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
            kind: Element::Line(id),
            parameters: vec![],
            children,
        });
        Ok(())
    }

    fn parse_normal(
        &mut self,
        parent: &mut Vec<ElementNode<'a>>,
        line: &'a str,
    ) -> Result<(), ErrorKind> {
        let mut uncommited: Vec<ElementNode<'a>> = vec![];
        let commited = parent;

        let mut rest = line;
        while let Some(captures) = REGEX_ELEMENT_LINE.captures(rest) {
            // before tag separation
            let whole_match = captures.get(0).unwrap();
            let leading_text = ElementNode::Text(&rest[0..whole_match.start()]);
            // TODO: 本当はこれを何回も書きたくない
            if uncommited.is_empty() {
                commited.push(leading_text);
            } else {
                match uncommited.last_mut().unwrap() {
                    ElementNode::Surrounded { children, .. } => children.push(leading_text),
                    ElementNode::Text(_) => unreachable!("Text node must not be pushed as a tag"),
                }
            }

            let next_rest_start = if let Some(tag_start) = captures.get(1) {
                let element = tag_start.as_str();
                if element.starts_with('@') {
                    // line element
                    uncommited.push(ElementNode::new_surrounded(Element::Line(
                        (&element[1..]).to_string(),
                    )));
                } else {
                    // other
                    let kind = Parser::parse_element_kind(element)?;
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
                                        ElementNode::Surrounded { children, .. } => {
                                            children.push(popped)
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
        } else if rest != "" {
            commited.push(ElementNode::Text(rest));
        }
        Ok(())
    }

    fn parse_element_kind(name: &'a str) -> Result<Element, ErrorKind> {
        match name {
            "b" => Ok(Element::Bold),
            "i" => Ok(Element::Italic),
            "ul" => Ok(Element::Underlined),
            "st" => Ok(Element::Deleted),
            "dt" => Ok(Element::Dotted),
            "br" => Ok(Element::Newline),
            "link" => Ok(Element::Link),
            "ruby" => Ok(Element::Ruby),
            "item" => Ok(Element::Item),
            _ => Err(ErrorKind::UnknownElement(name.to_string())),
        }
    }

    // command functions ------------------------------------------------------

    fn command_character(
        &mut self,
        character_set: &mut CharacterSet,
        kind: &'a str,
        id: &'a str,
        name: &'a str,
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
                if !REGEX_COLORCODE.is_match(colorcode) {
                    Err(ErrorKind::Semantic(SemanticErrorKind::UndefinedCharacter(
                        format!("{} (invalid colorcode {})", id, colorcode),
                    )))
                } else {
                    match colorcode.len() {
                        3 | 6 => character_set
                            .add_custom(id, name, colorcode)
                            .map_err(ErrorKind::Semantic),
                        _ => Err(ErrorKind::Semantic(SemanticErrorKind::UndefinedCharacter(
                            format!("{} (invalid colorcode {})", id, colorcode),
                        ))),
                    }
                }
            }
        }
    }
}
