use crate::error::SemanticErrorKind;
use std::collections::BTreeMap;

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

    /// Returns CSS class name for character.
    pub fn get_class(&self, id: &str) -> Option<String> {
        let character = self.characters.get(id)?;

        Some(match character {
            CharacterType::Male(n, _) => format!("male-{}", n % self.preset_max),
            CharacterType::Female(n, _) => format!("female-{}", n % self.preset_max),
            CharacterType::Mob(n, _) => format!("mob-{}", n % self.preset_max),
            CharacterType::Custom(_, _) => unimplemented!(),
        })
    }
}

/// Represents block element.
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
}

/// Represents inline element.
pub enum Element {
    /// new line
    Newline,

    /// bold text
    Bold,

    /// italic text
    Italic,

    /// dotted (傍点) text
    Dotted,

    /// underlined text
    Underlined,

    /// deleted text
    Deleted,

    /// link
    Link(String),

    /// text with ruby
    Ruby(Box<Element>),

    /// line, speech
    Line(String),
}

pub enum BlockNode<'a> {
    Singular(Block),
    Surrounded {
        kind: Block,
        children: Vec<ElementNode<'a>>,
    },
}

pub enum ElementNode<'a> {
    Text(&'a str),
    Singular(Element),
    Surrounded {
        kind: Element,
        children: Vec<ElementNode<'a>>,
    },
}

/// Represents whole S3WF2 document.
pub struct Document<'a> {
    pub(crate) characters: CharacterSet,
    pub(crate) blocks: Vec<BlockNode<'a>>,
}
