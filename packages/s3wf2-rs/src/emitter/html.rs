use crate::{
    document::{Block, BlockNode, CharacterSet, CharacterType, Document, Element, ElementNode},
    emitter::Emit,
};

/// Represents error kinds in HtmlEmitter.
pub enum HtmlEmitterError {
    UndefinedCharacter(String),
    InvalidParameter(&'static str),
}

/// Represents an anchor in HTML formatted document.
pub struct HtmlAnchor {
    /// Title
    pub title: String,

    /// Anchor id attribute
    pub id: String,
}

/// The HTML emitter.
pub struct HtmlEmitter {
    max_reserved_characters: usize,
}

impl HtmlEmitter {
    pub fn new(max_characters: usize) -> HtmlEmitter {
        HtmlEmitter {
            max_reserved_characters: max_characters,
        }
    }

    /// Returns the class name for a character.
    fn get_character_class(&self, character: &CharacterType) -> String {
        match character {
            CharacterType::Male(i, _) => format!("male-{}", i % self.max_reserved_characters),
            CharacterType::Female(i, _) => format!("female-{}", i % self.max_reserved_characters),
            CharacterType::Mob(i, _) => format!("mob-{}", i % self.max_reserved_characters),
            CharacterType::Custom(color, _) => format!("custom-{}", color),
        }
    }

    /// Returns <style> element which contains custom character styles.
    fn emit_character_styles(&self, characters: &CharacterSet) -> String {
        let mut result = String::new();
        let custom_colors = characters.characters().filter_map(|(_, t)| match t {
            CharacterType::Custom(color, _) => Some(color),
            _ => None,
        });

        result.push_str("<style>\n");
        for color in custom_colors {
            result.push_str(&format!(".custom-{0} {{ color: #{0}; }}\n", color));
        }
        result.push_str("</style>\n");

        result
    }

    /// Finds some characters needs to escape in string.
    fn find_escape_chars(text: &str) -> Option<(usize, char)> {
        text.chars().enumerate().find_map(|(i, ch)| match ch {
            '<' | '>' | '&' | '"' => Some((i, ch)),
            _ => None,
        })
    }

    /// Writes simple ElementNode into String.
    fn write_simple_surrounded_element(
        &self,
        target: &mut String,
        tag: &str,
        classes: Option<&str>,
        characters: &CharacterSet,
        children: &[ElementNode],
    ) -> Result<(), HtmlEmitterError> {
        target.push('<');
        target.push_str(tag);
        if let Some(cl) = classes {
            target.push_str(" class=\"");
            target.push_str(cl);
            target.push('"');
        }
        target.push('>');

        for child in children {
            self.write_element(target, characters, child)?;
        }

        target.push_str("</");
        target.push_str(tag);
        target.push('>');

        Ok(())
    }

    /// Writes ElementNode content in HTML format into String.
    fn write_element(
        &self,
        target: &mut String,
        characters: &CharacterSet,
        element: &ElementNode,
    ) -> Result<(), HtmlEmitterError> {
        match element {
            ElementNode::Text(text) => {
                let mut rest = &text[..];
                while let Some((head, ch)) = HtmlEmitter::find_escape_chars(rest) {
                    target.push_str(&rest[..head]);
                    match ch {
                        '<' => target.push_str("&lt;"),
                        '>' => target.push_str("&gt;"),
                        '&' => target.push_str("&amp;"),
                        '"' => target.push_str("&quot;"),
                        _ => unreachable!("Undefined escaping char"),
                    }
                    rest = &rest[(head + 1)..];
                }
                target.push_str(rest);
            }
            ElementNode::Surrounded {
                kind,
                parameters,
                children,
            } => match kind {
                Element::Line(id) => {
                    let name = characters
                        .get(id)
                        .ok_or_else(|| HtmlEmitterError::UndefinedCharacter(id.to_string()))?;
                    self.write_simple_surrounded_element(
                        target,
                        "span",
                        Some(&self.get_character_class(name)),
                        characters,
                        children,
                    )?;
                }
                Element::Bold => self.write_simple_surrounded_element(
                    target, "strong", None, characters, children,
                )?,
                Element::Italic => self.write_simple_surrounded_element(
                    target, "i", None, characters, children,
                )?,
                Element::Underlined => self.write_simple_surrounded_element(
                    target, "span", Some("underline"), characters, children,
                )?,
                Element::Deleted => self.write_simple_surrounded_element(
                    target, "del", None, characters, children,
                )?,
                Element::Dotted => self.write_simple_surrounded_element(
                    target, "span", Some("dots"), characters, children,
                )?,
                Element::Monospaced => self.write_simple_surrounded_element(
                    target, "code", None, characters, children,
                )?,
                Element::Item => self.write_simple_surrounded_element(
                    target, "strong", None, characters, children,
                )?,
                Element::Link => {
                    let href = parameters.first().map(|p| {
                        let children = p.unwrap_parameter();
                        if let Some(ElementNode::Text(url)) = children.first() {
                            Ok(url)
                        } else {
                            Err(HtmlEmitterError::InvalidParameter("Only a plain text content is valid for link target"))
                        }
                    }).ok_or(HtmlEmitterError::InvalidParameter("Only a plain text content is valid for link target"))??;
                    target.push_str("<a href=\"");
                    target.push_str(href);
                    target.push_str("\">");
                    for child in children {
                        self.write_element(target, characters, child)?;
                    }
                    target.push_str("</a>");
                }
                Element::Ruby => {
                    target.push_str("<ruby>");
                    for child in children {
                        self.write_element(target, characters, child)?;
                    }
                    target.push_str("<rp>(</rp><rt>");
                    if let Some(first) = parameters.first() {
                        self.write_element(target, characters, first)?;
                    }
                    target.push_str("</rt><rp>)</rp></ruby>");
                }
                Element::Newline => {
                    target.push_str("<br>\n");
                },
                Element::Parameter => {
                    for child in children {
                        self.write_element(target, characters, child)?;
                    }
                }
            },
        }

        Ok(())
    }

    /// Writes BlockNode content in HTML format into String.
    fn write_block(
        &self,
        target: &mut String,
        characters: &CharacterSet,
        (i, block): (usize, &BlockNode),
    ) -> Result<(), HtmlEmitterError> {
        match block.kind {
            Block::Paragraph => {
                target.push_str("<p>\n");
                for child in &block.children {
                    self.write_element(target, characters, child)?;
                }
                target.push('\n');
                target.push_str("</p>\n");
            }
            Block::Quotation => {
                target.push_str("<blockquote>\n");
                for child in &block.children {
                    self.write_element(target, characters, child)?;
                }
                target.push('\n');
                target.push_str("</blockquote>\n");
            }
            Block::UnorderedList => {
                target.push_str("<blockquote>\n");
                for child in &block.children {
                    if let ElementNode::Surrounded {
                        kind: Element::Item,
                        ..
                    } = child
                    {
                        self.write_element(target, characters, child)?;
                    }
                }
                target.push('\n');
                target.push_str("</blockquote>\n");
            }
            Block::Section => {
                target.push_str(&format!("<h2 id=\"{}\">", format!("section-{}", i)));
                for child in &block.children {
                    self.write_element(target, characters, child)?;
                }
                target.push_str("</h2>\n");
            }
            Block::Subsection => {
                target.push_str(&format!("<h3 id=\"{}\">", format!("section-{}", i)));
                for child in &block.children {
                    self.write_element(target, characters, child)?;
                }
                target.push_str("</h3>\n");
            }
            Block::Horizontal => {
                target.push_str("<hr>\n");
            }
        }
        Ok(())
    }
}

impl<'a> Emit<'a> for HtmlEmitter {
    type Output = Result<String, HtmlEmitterError>;
    type Anchors = Vec<HtmlAnchor>;

    fn emit(&self, document: &Document<'a>) -> Self::Output {
        let mut result = String::with_capacity(1 << 16);
        result.push_str(&self.emit_character_styles(&document.characters));
        for block in document.blocks.iter().enumerate() {
            self.write_block(&mut result, &document.characters, block)?;
        }

        Ok(result)
    }

    fn section_anchors(&self, document: &Document<'a>) -> Self::Anchors {
        document
            .blocks
            .iter()
            .enumerate()
            .filter_map(|(i, b)| match b.kind {
                Block::Section | Block::Subsection => Some(HtmlAnchor {
                    id: format!("section-{}", i),
                    title: format!(""),
                }),
                _ => None,
            })
            .collect()
    }
}
