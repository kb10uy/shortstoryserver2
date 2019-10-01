use crate::error::SemanticErrorKind;
use std::{collections::BTreeMap, fmt};

/// Indicates the type of characters in document.
pub enum CharacterType {
    /// Preset male character
    Male(usize, String),

    /// Preset female character
    Female(usize, String),

    /// Preset mob character
    Mob(usize, String),

    /// Customized color character
    Custom(String, String),
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

/// The characters container.
pub struct CharacterSet {
    preset_max: usize,
    used_male: usize,
    used_female: usize,
    used_mob: usize,
    characters: BTreeMap<String, CharacterType>,
}

impl CharacterSet {
    /// Creates new instance.
    pub fn new(preset_max: usize) -> CharacterSet {
        CharacterSet {
            preset_max,
            used_male: 0,
            used_female: 0,
            used_mob: 0,
            characters: BTreeMap::new(),
        }
    }

    /// Adds a male character.
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

    pub fn get(&self, id: &str) -> Option<&CharacterType> {
        self.characters.get(id)
    }

    /// Returns CSS class name for character.
    pub fn get_class(&self, id: &str) -> Option<String> {
        let character = self.characters.get(id)?;

        Some(match character {
            CharacterType::Male(n, _) => format!("male-{}", n % self.preset_max),
            CharacterType::Female(n, _) => format!("female-{}", n % self.preset_max),
            CharacterType::Mob(n, _) => format!("mob-{}", n % self.preset_max),
            CharacterType::Custom(c, _) => format!("custom-{}", c),
        })
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

    /// Link
    Link,

    /// Text with ruby
    Ruby,

    /// List item
    Item,

    /// Line, speech (the parameter should contain the ID)
    Line(String),
}

/// Represents a block level node.
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
    pub(crate) characters: CharacterSet,
    pub(crate) blocks: Vec<BlockNode<'a>>,
}

impl<'a> Document<'a> {
    pub(crate) fn new() -> Document<'a> {
        Document {
            characters: CharacterSet::new(4),
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
