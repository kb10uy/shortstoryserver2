//! Includes abstract syntax node structs.

use crate::error::{ErrorKind, SemanticErrorKind};
use std::{collections::BTreeMap, fmt, str::FromStr};

/// Indicates the type of characters in document.
#[derive(Debug, PartialEq, Eq)]
pub enum CharacterType {
    /// Preset male character, with index and reference ID
    Male(usize, String),

    /// Preset female character, with index and reference ID
    Female(usize, String),

    /// Preset mob character, with index and reference ID
    Mob(usize, String),

    /// Customized color character, with colorcode (3 or 6 digit hexiadecimal, without `#`) and reference ID
    Custom(String, String),
}

impl CharacterType {
    /// Returns the name which should be displayed.
    pub fn display_name(&self) -> &str {
        match self {
            CharacterType::Male(_, name) => &name,
            CharacterType::Female(_, name) => &name,
            CharacterType::Mob(_, name) => &name,
            CharacterType::Custom(_, name) => &name,
        }
    }
}

impl fmt::Display for CharacterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CharacterType::Male(index, name) => write!(f, "{} (Male, #{})", name, index),
            CharacterType::Female(index, name) => write!(f, "{} (Female, #{})", name, index),
            CharacterType::Mob(index, name) => write!(f, "{} (Mob, #{})", name, index),
            CharacterType::Custom(color, name) => write!(f, "{} (Custom, #{})", name, color),
        }
    }
}

/// Contains characters metadata.
#[derive(Default)]
pub struct CharacterSet {
    used_male: usize,
    used_female: usize,
    used_mob: usize,
    characters: BTreeMap<String, CharacterType>,
}

impl CharacterSet {
    /// Creates a new instance.
    pub fn new() -> CharacterSet {
        CharacterSet {
            used_male: 0,
            used_female: 0,
            used_mob: 0,
            characters: BTreeMap::new(),
        }
    }

    /// Adds a male character.
    ///
    /// # Parameters
    /// * `id` - The reference ID
    /// * `name` - The name displayed in document
    pub fn add_male(&mut self, id: &str, name: &str) -> Result<(), SemanticErrorKind> {
        if self.characters.contains_key(id) {
            Err(SemanticErrorKind::DuplicateCharacter(id.to_string()))
        } else {
            self.used_male += 1;
            self.characters.insert(
                id.to_string(),
                CharacterType::Male(self.used_male, name.to_string()),
            );
            Ok(())
        }
    }

    /// Adds a female character.
    ///
    /// # Parameters
    /// * `id` - The reference ID
    /// * `name` - The name displayed in document
    pub fn add_female(&mut self, id: &str, name: &str) -> Result<(), SemanticErrorKind> {
        if self.characters.contains_key(id) {
            Err(SemanticErrorKind::DuplicateCharacter(id.to_string()))
        } else {
            self.used_female += 1;
            self.characters.insert(
                id.to_string(),
                CharacterType::Female(self.used_female, name.to_string()),
            );
            Ok(())
        }
    }

    /// Adds a mob character.
    ///
    /// # Parameters
    /// * `id` - The reference ID
    /// * `name` - The name displayed in document
    pub fn add_mob(&mut self, id: &str, name: &str) -> Result<(), SemanticErrorKind> {
        if self.characters.contains_key(id) {
            Err(SemanticErrorKind::DuplicateCharacter(id.to_string()))
        } else {
            self.used_mob += 1;
            self.characters.insert(
                id.to_string(),
                CharacterType::Mob(self.used_mob, name.to_string()),
            );
            Ok(())
        }
    }

    /// Adds a custom color character.
    ///
    /// # Parameters
    /// * `id` - The reference ID
    /// * `name` - The name displayed in document
    /// * `color` - A colorcode, which consists of 3 or 6 hexadecimal digits
    pub fn add_custom(
        &mut self,
        id: &str,
        name: &str,
        color: &str,
    ) -> Result<(), SemanticErrorKind> {
        if self.characters.contains_key(id) {
            Err(SemanticErrorKind::DuplicateCharacter(id.to_string()))
        } else {
            self.characters.insert(
                id.to_string(),
                CharacterType::Custom(color.to_string(), name.to_string()),
            );
            Ok(())
        }
    }

    /// Gets the character type related to the ID.
    pub fn get(&self, id: &str) -> Option<&CharacterType> {
        self.characters.get(id)
    }

    /// Returns character iterator.
    /// The order is guaranteed to be consistent.
    pub fn characters(&self) -> impl Iterator<Item = (&String, &CharacterType)> {
        self.characters.iter()
    }
}

impl fmt::Display for CharacterSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Characters [")?;
        for (id, character) in self.characters.iter() {
            write!(f, "\"{}\" => {}, ", id, character)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

/// Represents block element.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Block {
    /// Horizontal line
    Horizontal,

    /// A paragraph
    Paragraph,

    /// Section title
    Section,

    /// Subsection title
    Subsection,

    /// Block quotation
    Quotation,

    /// Unordered list
    UnorderedList,
}

impl FromStr for Block {
    type Err = ErrorKind;

    fn from_str(s: &str) -> Result<Block, ErrorKind> {
        match s {
            "para" => Ok(Block::Paragraph),
            "sec" => Ok(Block::Section),
            "subsec" => Ok(Block::Subsection),
            "quote" => Ok(Block::Quotation),
            "hori" => Ok(Block::Horizontal),
            "list" => Ok(Block::UnorderedList),
            _ => Err(ErrorKind::UnknownElement(s.to_string())),
        }
    }
}

/// Represents the sub-type of `Element::Line`.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum LineType {
    /// Shown in a dedicated line with name.
    NameShownBlock,

    /// Shown in a dedicated line without name.
    NameHiddenBlock,

    /// Shown in a inline element.
    Inline,
}

/// Represents inline element.
#[derive(PartialEq, Eq, Debug)]
pub enum Element {
    /// Parameter
    Parameter,

    /// New line
    Newline,

    /// Old text
    Bold,

    /// Italic text
    Italic,

    /// Dotted (傍点) text
    Dotted,

    /// Underlined text
    Underlined,

    /// Deleted text
    Deleted,

    /// Monospace font text
    Monospaced,

    /// Link
    Link,

    /// Text with ruby
    Ruby,

    /// List item
    Item,

    /// Line, speech (the parameter should contain the ID, and whether inline display or not)
    Line(String, LineType),
}

impl FromStr for Element {
    type Err = ErrorKind;

    fn from_str(s: &str) -> Result<Element, ErrorKind> {
        match s {
            "b" => Ok(Element::Bold),
            "i" => Ok(Element::Italic),
            "ul" => Ok(Element::Underlined),
            "st" => Ok(Element::Deleted),
            "dt" => Ok(Element::Dotted),
            "m" => Ok(Element::Monospaced),
            "br" => Ok(Element::Newline),
            "link" => Ok(Element::Link),
            "ruby" => Ok(Element::Ruby),
            "item" => Ok(Element::Item),
            _ => Err(ErrorKind::UnknownElement(s.to_string())),
        }
    }
}

/// Represents a block level node.
#[derive(Debug, PartialEq, Eq)]
pub struct BlockNode<'a> {
    /// Block type
    pub kind: Block,

    /// Child nodes
    pub children: Vec<ElementNode<'a>>,
}

impl<'a> BlockNode<'a> {
    pub fn new(kind: Block) -> BlockNode<'a> {
        BlockNode {
            kind,
            children: vec![],
        }
    }

    /// Checks whether this BlockNode is empty (unused).
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
}

impl<'a> fmt::Display for BlockNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} [", self.kind)?;
        for child in self.children.iter() {
            write!(f, "{}, ", child)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

/// Represents a element level node.
#[derive(Debug, PartialEq, Eq)]
pub enum ElementNode<'a> {
    /// Plain text node
    Text(&'a str),

    /// Surrounded element node
    Surrounded {
        /// Element type
        kind: Element,

        /// Corresponding parameter nodes
        parameters: Vec<ElementNode<'a>>,

        /// Child nodes
        children: Vec<ElementNode<'a>>,
    },
}

impl<'a> ElementNode<'a> {
    /// Creates a surrounded node.
    pub fn new_surrounded(kind: Element) -> ElementNode<'a> {
        ElementNode::Surrounded {
            kind,
            parameters: vec![],
            children: vec![],
        }
    }

    /// Unwraps the node, and take children from parameter.
    pub fn unwrap_parameter(&self) -> &Vec<ElementNode<'a>> {
        match self {
            ElementNode::Surrounded {
                kind: Element::Parameter,
                children,
                ..
            } => children,
            _ => panic!("Failed to unwrap non-parameter node"),
        }
    }
}

impl<'a> fmt::Display for ElementNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElementNode::Text(text) => write!(f, "\"{}\"", text),
            ElementNode::Surrounded { kind, children, .. } => {
                write!(f, "[{:?} ", kind)?;
                for child in children.iter() {
                    write!(f, "{}, ", child)?;
                }
                write!(f, "]")?;
                Ok(())
            }
        }
    }
}

/// Represents whole S3WF2 document.
pub struct Document<'a> {
    pub characters: CharacterSet,
    pub blocks: Vec<BlockNode<'a>>,
}

impl<'a> Document<'a> {
    /// Creates a new instance.
    pub(crate) fn new() -> Document<'a> {
        Document {
            characters: CharacterSet::new(),
            blocks: vec![],
        }
    }
}

impl<'a> fmt::Display for Document<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Document {{")?;
        writeln!(f, "  {}", self.characters)?;
        for block in self.blocks.iter() {
            writeln!(f, "  {}", block)?;
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn character_set_works() {
        let mut characters = CharacterSet::new();
        characters.add_male("kb10uy", "佑").unwrap();
        characters.add_female("natsuki", "夏稀").unwrap();
        characters.add_mob("tomone", "朋音").unwrap();
        characters.add_female("ayano", "文乃").unwrap();

        let mut iter = characters.characters();
        assert_eq!(
            iter.next(),
            Some((
                &"ayano".to_string(),
                &CharacterType::Female(2, "文乃".to_string())
            ))
        );
        assert_eq!(
            iter.next(),
            Some((
                &"kb10uy".to_string(),
                &CharacterType::Male(1, "佑".to_string())
            ))
        );
        assert_eq!(
            iter.next(),
            Some((
                &"natsuki".to_string(),
                &CharacterType::Female(1, "夏稀".to_string())
            ))
        );
        assert_eq!(
            iter.next(),
            Some((
                &"tomone".to_string(),
                &CharacterType::Mob(1, "朋音".to_string())
            ))
        );
        assert_eq!(iter.next(), None);
    }

    #[test]
    #[should_panic(expected = "Failed to unwrap non-parameter node")]
    fn element_node_panics_on_unwrap_non_parameter() {
        let element = ElementNode::new_surrounded(Element::Bold);
        element.unwrap_parameter();
    }
}
