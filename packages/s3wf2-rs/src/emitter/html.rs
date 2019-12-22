use std::{
    error::Error as StdError,
    fmt,
    io::{prelude::*, Error, ErrorKind, Result as IoResult},
};

use crate::{
    document::{
        Block, BlockNode, CharacterSet, CharacterType, Document, Element, ElementNode, LineType,
    },
    emitter::{Emit, ExtractIndices},
};

// Error ----------------------------------------------------------------------

/// Represents error kinds in HtmlEmitter.
#[derive(Debug)]
pub enum HtmlEmitterError {
    /// Undefined character ID appeared.
    UndefinedCharacter(String),

    /// Invalid parameter value detected.
    InvalidParameter(&'static str),
}

impl fmt::Display for HtmlEmitterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HtmlEmitterError::UndefinedCharacter(character) => {
                write!(f, "Undefined character: {}", character)
            }
            HtmlEmitterError::InvalidParameter(e) => write!(f, "Invalid parameter: {}", e),
        }
    }
}

impl StdError for HtmlEmitterError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

// Emitter --------------------------------------------------------------------

/// The HTML emitter.
pub struct HtmlEmitter {
    max_reserved_characters: usize,
}

impl HtmlEmitter {
    /// Creates a new instance.
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
    fn write_character_styles(
        &self,
        writer: &mut impl Write,
        characters: &CharacterSet,
    ) -> IoResult<()> {
        let custom_colors = characters.characters().filter_map(|(_, t)| match t {
            CharacterType::Custom(color, _) => Some(color),
            _ => None,
        });

        writeln!(writer, "<style>")?;
        for color in custom_colors {
            writeln!(writer, ".custom-{0} {{ color: #{0};", color)?;
        }
        writeln!(writer, "</style>")?;

        Ok(())
    }

    /// Finds some characters needs to escape in string.
    fn find_escape_chars(text: &str) -> Option<(usize, char)> {
        text.char_indices().find_map(|(i, ch)| match ch {
            '<' | '>' | '&' | '"' => Some((i, ch)),
            _ => None,
        })
    }

    /// Writes simple ElementNode into String.
    fn write_simple_surrounded_element(
        &self,
        writer: &mut impl Write,
        tag: &str,
        classes: Option<&str>,
        characters: &CharacterSet,
        children: &[ElementNode],
    ) -> IoResult<()> {
        write!(writer, "<{}", tag)?;
        if let Some(cl) = classes {
            write!(writer, " class=\"{}\"", cl)?;
        }
        write!(writer, ">")?;

        for child in children {
            self.write_element(writer, characters, child)?;
        }

        write!(writer, "</{}>", tag)?;

        Ok(())
    }

    /// Writes ElementNode content in HTML format into String.
    fn write_element(
        &self,
        writer: &mut impl Write,
        characters: &CharacterSet,
        element: &ElementNode,
    ) -> IoResult<()> {
        match element {
            ElementNode::Text(text) => {
                let mut rest = &text[..];
                while let Some((head, ch)) = HtmlEmitter::find_escape_chars(rest) {
                    writer.write_all(&rest[..head].as_bytes())?;
                    match ch {
                        '<' => writer.write_all(b"&lt;")?,
                        '>' => writer.write_all(b"&gt;")?,
                        '&' => writer.write_all(b"&amp;")?,
                        '"' => writer.write_all(b"&quot;")?,
                        _ => unreachable!("Undefined escaping char"),
                    }
                    rest = &rest[(head + 1)..];
                }
                writer.write_all(&rest.as_bytes())?;
            }
            ElementNode::Surrounded {
                kind,
                parameters,
                children,
            } => match kind {
                Element::Line(id, line_type) => {
                    let name = characters.get(id).ok_or_else(|| {
                        Error::new(
                            ErrorKind::NotFound,
                            HtmlEmitterError::UndefinedCharacter(id.to_string()),
                        )
                    })?;

                    match line_type {
                        LineType::Inline => {
                            let classes = format!("inline line {}", self.get_character_class(name));
                            self.write_simple_surrounded_element(
                                writer,
                                "span",
                                Some(&classes),
                                characters,
                                children,
                            )?;
                        }
                        _ => {
                            write!(
                                writer,
                                "\n<span class=\"line {}\">",
                                self.get_character_class(name)
                            )?;
                            if *line_type == LineType::NameShownBlock {
                                self.write_element(
                                    writer,
                                    characters,
                                    &ElementNode::Text(name.display_name()),
                                )?;
                            }
                            for element in children {
                                self.write_element(writer, characters, element)?;
                            }

                            writeln!(writer, "</span>")?;
                        }
                    }
                }
                Element::Bold => self.write_simple_surrounded_element(
                    writer, "strong", None, characters, children,
                )?,
                Element::Italic => {
                    self.write_simple_surrounded_element(writer, "i", None, characters, children)?
                }
                Element::Underlined => self.write_simple_surrounded_element(
                    writer,
                    "span",
                    Some("underline"),
                    characters,
                    children,
                )?,
                Element::Deleted => {
                    self.write_simple_surrounded_element(writer, "del", None, characters, children)?
                }
                Element::Dotted => self.write_simple_surrounded_element(
                    writer,
                    "span",
                    Some("dots"),
                    characters,
                    children,
                )?,
                Element::Monospaced => self
                    .write_simple_surrounded_element(writer, "code", None, characters, children)?,
                Element::Item => {
                    self.write_simple_surrounded_element(writer, "li", None, characters, children)?;
                    writeln!(writer)?;
                }
                Element::Link => {
                    let href = parameters
                        .first()
                        .map(|p| {
                            let children = p.unwrap_parameter();
                            if let Some(ElementNode::Text(url)) = children.first() {
                                Ok(url)
                            } else {
                                Err(Error::new(
                                    ErrorKind::InvalidData,
                                    HtmlEmitterError::InvalidParameter(
                                        "Only a plain text content is valid for link target",
                                    ),
                                ))
                            }
                        })
                        .ok_or_else(|| {
                            Error::new(
                                ErrorKind::Other,
                                HtmlEmitterError::InvalidParameter("Link URL needed"),
                            )
                        })??;
                    write!(writer, "<a href=\"")?;
                    self.write_element(writer, characters, &ElementNode::Text(href))?;
                    write!(writer, "\">")?;
                    for child in children {
                        self.write_element(writer, characters, child)?;
                    }
                    writer.write_all(b"</a>")?;
                }
                Element::Ruby => {
                    writer.write_all(b"<ruby>")?;
                    for child in children {
                        self.write_element(writer, characters, child)?;
                    }
                    writer.write_all(b"<rp>(</rp><rt>")?;
                    if let Some(first) = parameters.first() {
                        self.write_element(writer, characters, first)?;
                    }
                    writer.write_all(b"</rt><rp>)</rp></ruby>")?;
                }
                Element::Newline => {
                    writer.write_all(b"<br>\n")?;
                }
                Element::Parameter => {
                    for child in children {
                        self.write_element(writer, characters, child)?;
                    }
                }
            },
        }

        Ok(())
    }

    /// Writes BlockNode content in HTML format into String.
    fn write_block(
        &self,
        writer: &mut impl Write,
        characters: &CharacterSet,
        (i, block): (usize, &BlockNode),
    ) -> IoResult<()> {
        match block.kind {
            Block::Paragraph => {
                writeln!(writer, "<p>")?;
                for child in &block.children {
                    self.write_element(writer, characters, child)?;
                }
                writeln!(writer, "\n</p>")?;
            }
            Block::Quotation => {
                writeln!(writer, "<blockquote>")?;
                for child in &block.children {
                    self.write_element(writer, characters, child)?;
                }
                writeln!(writer, "\n</blockquote>")?;
            }
            Block::UnorderedList => {
                writeln!(writer, "<ul>")?;
                for child in &block.children {
                    if let ElementNode::Surrounded {
                        kind: Element::Item,
                        ..
                    } = child
                    {
                        self.write_element(writer, characters, child)?;
                    }
                }
                writeln!(writer, "\n</ul>")?;
            }
            Block::Section => {
                write!(writer, "<h2 id=\"section-{}\">", i)?;
                for child in &block.children {
                    self.write_element(writer, characters, child)?;
                }
                writeln!(writer, "</h2>")?;
            }
            Block::Subsection => {
                write!(writer, "<h3 id=\"section-{}\">", i)?;
                for child in &block.children {
                    self.write_element(writer, characters, child)?;
                }
                writeln!(writer, "</h3>")?;
            }
            Block::Horizontal => {
                writeln!(writer, "<hr>")?;
            }
        }
        Ok(())
    }
}

impl<'a> Emit<'a> for HtmlEmitter {
    fn emit(&mut self, writer: &mut impl Write, document: &Document<'a>) -> IoResult<()> {
        self.write_character_styles(writer, &document.characters)?;
        for block in document.blocks.iter().enumerate() {
            self.write_block(writer, &document.characters, block)?;
        }

        Ok(())
    }
}

// Index extractor

/// Represents an anchor in HTML formatted document.
pub struct HtmlAnchor {
    /// Title
    pub title: String,

    /// Anchor id attribute
    pub id: String,
}

/// Iterator adaptor which iterates HtmlAnchor.
pub struct HtmlAnchorIter<'d, 's>(&'d [BlockNode<'s>], usize);

impl Iterator for HtmlAnchorIter<'_, '_> {
    type Item = HtmlAnchor;

    fn next(&mut self) -> Option<Self::Item> {
        while self.1 < self.0.len() {
            let target = &self.0[self.1];
            match target.kind {
                Block::Section | Block::Subsection => {
                    return Some(HtmlAnchor {
                        id: format!("section-{}", self.1),
                        // TODO: タグなしタイトルをつける
                        title: format!(""),
                    });
                }
                _ => (),
            }
            self.1 += 1;
        }

        None
    }
}

impl<'d, 's: 'd> ExtractIndices<'d, 's> for HtmlEmitter {
    type IndexItemIter = HtmlAnchorIter<'d, 's>;
    type IndexItem = HtmlAnchor;

    fn indices(&self, document: &'d Document<'s>) -> Self::IndexItemIter {
        HtmlAnchorIter(&document.blocks, 0)
    }
}
