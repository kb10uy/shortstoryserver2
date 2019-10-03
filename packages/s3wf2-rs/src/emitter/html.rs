use crate::{
    document::{Block, BlockNode, CharacterSet, CharacterType, Document, Element, ElementNode},
    emitter::Emit,
};

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

    fn emit_character_styles(&self, characters: &CharacterSet) -> String {
        let mut result = String::new();
        let custom_colors = characters.characters().filter_map(|(_, t)| match t {
            CharacterType::Custom(color, _) => Some(color),
            _ => None,
        });

        result.push_str("<style>\n");
        for color in custom_colors {
            result.push_str(&format!("  .custom-{0} {{ color: #{0}; }}\n", color));
        }
        result.push_str("<style>\n");

        result
    }

    fn find_escape_chars(text: &str) -> Option<(usize, char)> {
        text.chars().enumerate().find_map(|(i, ch)| match ch {
            '<' | '>' | '&' | '"' => Some((i, ch)),
            _ => None,
        })
    }

    fn write_simple_surrounded_element(
        &self,
        target: &mut String,
        tag: &str,
        classes: Option<&str>,
        children: &[ElementNode],
    ) {
        target.push('<');
        target.push_str(tag);
        if let Some(cl) = classes {
            target.push_str(" class=\"");
            target.push_str(cl);
            target.push('"');
        }
        target.push('>');

        for child in children {
            self.write_element(target, child);
        }

        target.push_str("</");
        target.push_str(tag);
        target.push('>');
    }

    fn write_element(&self, target: &mut String, element: &ElementNode) {
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
                    target.push_str("<span class="
                }
                Element::Bold => {
                    self.write_simple_surrounded_element(target, "strong", None, children)
                }
                _ => unreachable!("Unexpected element found"),
            },
        }
    }

    fn write_block(&self, target: &mut String, (i, block): (usize, &BlockNode)) {
        match block.kind {
            Block::Paragraph => {
                target.push_str("<p>\n");
                for child in &block.children {
                    self.write_element(target, child);
                }
                target.push('\n');
                target.push_str("</p>\n");
            }
            Block::Quotation => {
                target.push_str("<blockquote>\n");
                for child in &block.children {
                    self.write_element(target, child);
                }
                target.push('\n');
                target.push_str("</blockquote>\n");
            }
            Block::UnorderedList => {
                target.push_str("<blockquote>\n");
                for child in &block.children {
                    if let ElementNode::Surrounded { kind: Element::Item, .. } = child {
                        self.write_element(target, child);
                    }
                }
                target.push('\n');
                target.push_str("</blockquote>\n");

            }
            Block::Section => {
                target.push_str(&format!("<h2 id=\"{}\">", format!("section-{}", i)));
                for child in &block.children {
                    self.write_element(target, child);
                }
                target.push_str("</h2>\n");
            }
            Block::Subsection => {
                target.push_str(&format!("<h3 id=\"{}\">", format!("section-{}", i)));
                for child in &block.children {
                    self.write_element(target, child);
                }
                target.push_str("</h3>\n");
            }
            Block::Horizontal => {
                target.push_str("<hr>\n");
            }
        }
    }
}

impl<'a> Emit<'a> for HtmlEmitter {
    type Output = String;
    type Anchors = Vec<HtmlAnchor>;

    fn emit(&self, document: &Document<'a>) -> Self::Output {
        let mut result = String::with_capacity(1 << 16);
        result.push_str(&self.emit_character_styles(&document.characters));
        for block in document.blocks.iter().enumerate() {
            self.write_block(&mut result, block);
        }

        result
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
