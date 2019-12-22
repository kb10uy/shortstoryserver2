use crate::{
    document::{
        Block, BlockNode, CharacterSet, CharacterType, Document, Element, ElementNode, LineType,
    },
    emitter::Emit,
};
use ansi_term::{Colour, Style};
use lazy_static::lazy_static;
use std::io::{prelude::*, Result};

#[derive(PartialEq)]
enum StackStyle {
    Bold,
    Italic,
    Underlined,
    Striked,
    Blink,
}

#[derive(PartialEq)]
enum AbstractStyle {
    Style(StackStyle),
    Color(Colour),
}

enum NewlineState {
    InUse,
    Ready,
    NewParagraph,
}

/// Emitter for virtual terminal.
pub struct ConsoleEmitter {
    style_stack: Vec<AbstractStyle>,
    newline_state: NewlineState,
}

lazy_static! {
    static ref MALE_COLORS: Vec<Colour> = vec![
        Colour::Fixed(26),
        Colour::Fixed(80),
        Colour::Fixed(74),
        Colour::Fixed(62)
    ];
    static ref FEMALE_COLORS: Vec<Colour> = vec![
        Colour::Fixed(170),
        Colour::Fixed(179),
        Colour::Fixed(209),
        Colour::Fixed(229)
    ];
    static ref MOB_COLORS: Vec<Colour> = vec![
        Colour::Fixed(195),
        Colour::Fixed(230),
        Colour::Fixed(153),
        Colour::Fixed(158)
    ];
}

impl ConsoleEmitter {
    pub fn new() -> ConsoleEmitter {
        ConsoleEmitter {
            style_stack: vec![],
            newline_state: NewlineState::NewParagraph,
        }
    }

    fn current_style(&self) -> ansi_term::Style {
        let mut color = Colour::White;
        let mut styles = vec![];
        for abst in &self.style_stack {
            match abst {
                AbstractStyle::Style(s) => {
                    if !styles.contains(&s) {
                        styles.push(s);
                    }
                }
                AbstractStyle::Color(c) => {
                    color = *c;
                }
            }
        }

        styles.iter().fold(Style::from(color), |r, s| match s {
            StackStyle::Bold => r.bold(),
            StackStyle::Italic => r.italic(),
            StackStyle::Underlined => r.underline(),
            StackStyle::Striked => r.strikethrough(),
            StackStyle::Blink => r.blink(),
        })
    }

    fn get_color<'c>(&self, character: Option<&'c CharacterType>) -> (Colour, &'c str) {
        match character {
            Some(CharacterType::Male(i, n)) => (MALE_COLORS[i % MALE_COLORS.len()], n),
            Some(CharacterType::Female(i, n)) => (FEMALE_COLORS[i % FEMALE_COLORS.len()], n),
            Some(CharacterType::Mob(i, n)) => (MOB_COLORS[i % MOB_COLORS.len()], n),
            Some(CharacterType::Custom(cc, n)) => {
                let (r, g, b) = if cc.len() == 3 {
                    let r = u8::from_str_radix(&cc[0..1], 16).unwrap_or(7) * 17;
                    let g = u8::from_str_radix(&cc[1..2], 16).unwrap_or(7) * 17;
                    let b = u8::from_str_radix(&cc[2..3], 16).unwrap_or(7) * 17;
                    (r, g, b)
                } else {
                    let r = u8::from_str_radix(&cc[0..2], 16).unwrap_or(127);
                    let g = u8::from_str_radix(&cc[2..4], 16).unwrap_or(127);
                    let b = u8::from_str_radix(&cc[4..6], 16).unwrap_or(127);
                    (r, g, b)
                };
                (Colour::RGB(r, g, b), n)
            }
            None => (Colour::Fixed(249), "[Undefined]"),
        }
    }

    fn confirm_paragraph(&mut self, writer: &mut impl Write) -> Result<()> {
        match self.newline_state {
            NewlineState::InUse => writeln!(writer, "\n")?,
            NewlineState::Ready => writeln!(writer)?,
            NewlineState::NewParagraph => (),
        }
        self.newline_state = NewlineState::NewParagraph;
        Ok(())
    }

    fn confirm_newline(&mut self, writer: &mut impl Write) -> Result<()> {
        match self.newline_state {
            NewlineState::InUse => writeln!(writer)?,
            NewlineState::Ready => (),
            NewlineState::NewParagraph => (),
        }
        self.newline_state = NewlineState::Ready;
        Ok(())
    }

    fn emit_block(
        &mut self,
        writer: &mut impl Write,
        characters: &CharacterSet,
        block: &BlockNode,
    ) -> Result<()> {
        match block.kind {
            Block::Section => {
                self.confirm_paragraph(writer)?;
                self.style_stack.push(AbstractStyle::Color(Colour::Yellow));
                self.emit_element(writer, characters, &ElementNode::Text("###### "))?;
                self.emit_elements(writer, characters, &block.children)?;
                self.emit_element(writer, characters, &ElementNode::Text(" ######"))?;
                self.style_stack.pop();

                self.confirm_paragraph(writer)
            }
            Block::Subsection => {
                self.confirm_paragraph(writer)?;
                self.style_stack.push(AbstractStyle::Color(Colour::Yellow));
                self.emit_element(writer, characters, &ElementNode::Text("====== "))?;
                self.emit_elements(writer, characters, &block.children)?;
                self.emit_element(writer, characters, &ElementNode::Text(" ======"))?;
                self.style_stack.pop();

                self.confirm_paragraph(writer)
            }
            Block::Horizontal => {
                self.confirm_paragraph(writer)?;
                writeln!(writer, "--------------------------------------------------------------------------------")?;
                self.newline_state = NewlineState::Ready;
                self.confirm_paragraph(writer)
            }
            Block::Paragraph => {
                self.confirm_paragraph(writer)?;
                self.emit_elements(writer, characters, &block.children)?;
                self.confirm_paragraph(writer)
            }
            Block::Quotation => {
                self.confirm_paragraph(writer)?;
                self.confirm_newline(writer)?;
                self.style_stack
                    .push(AbstractStyle::Color(Colour::RGB(127, 127, 127)));
                self.emit_elements(writer, characters, &block.children)?;
                self.style_stack.pop();
                self.confirm_paragraph(writer)
            }
            Block::UnorderedList => {
                self.confirm_paragraph(writer)?;
                self.emit_elements(writer, characters, &block.children)?;
                self.confirm_paragraph(writer)
            }
        }
    }

    fn emit_elements(
        &mut self,
        writer: &mut impl Write,
        characters: &CharacterSet,
        elements: &[ElementNode],
    ) -> Result<()> {
        for element in elements {
            self.emit_element(writer, characters, element)?;
        }
        Ok(())
    }

    fn emit_element(
        &mut self,
        writer: &mut impl Write,
        characters: &CharacterSet,
        element: &ElementNode,
    ) -> Result<()> {
        match element {
            ElementNode::Text(text) => {
                self.newline_state = NewlineState::InUse;
                write!(writer, "{}", self.current_style().paint(*text))
            }
            ElementNode::Surrounded {
                kind,
                children,
                parameters,
            } => match kind {
                Element::Bold => {
                    self.style_stack
                        .push(AbstractStyle::Style(StackStyle::Bold));
                    self.emit_elements(writer, characters, children)?;
                    self.style_stack.pop();
                    Ok(())
                }
                Element::Italic => {
                    self.style_stack
                        .push(AbstractStyle::Style(StackStyle::Italic));
                    self.emit_elements(writer, characters, children)?;
                    self.style_stack.pop();
                    Ok(())
                }
                Element::Underlined => {
                    self.style_stack
                        .push(AbstractStyle::Style(StackStyle::Underlined));
                    self.emit_elements(writer, characters, children)?;
                    self.style_stack.pop();
                    Ok(())
                }
                Element::Deleted => {
                    self.style_stack
                        .push(AbstractStyle::Style(StackStyle::Striked));
                    self.emit_elements(writer, characters, children)?;
                    self.style_stack.pop();
                    Ok(())
                }
                Element::Dotted => {
                    self.style_stack
                        .push(AbstractStyle::Style(StackStyle::Blink));
                    self.emit_elements(writer, characters, children)?;
                    self.style_stack.pop();
                    Ok(())
                }
                Element::Ruby | Element::Link => {
                    self.emit_elements(writer, characters, children)?;
                    self.emit_element(writer, characters, &ElementNode::Text("("))?;
                    self.emit_elements(writer, characters, parameters)?;
                    self.emit_element(writer, characters, &ElementNode::Text(")"))
                }
                Element::Item => {
                    self.confirm_newline(writer)?;
                    self.emit_element(writer, characters, &ElementNode::Text("・"))?;
                    self.emit_elements(writer, characters, children)?;
                    self.confirm_newline(writer)
                }
                Element::Newline => writeln!(writer),
                Element::Line(id, LineType::Inline) => {
                    let ctype = characters.get(id);
                    let (color, _) = self.get_color(ctype);
                    self.style_stack.push(AbstractStyle::Color(color));
                    self.emit_elements(writer, characters, children)?;
                    self.style_stack.pop();
                    Ok(())
                }
                Element::Line(id, line_type) => {
                    let (color, name) = self.get_color(characters.get(id));
                    self.confirm_newline(writer)?;
                    self.style_stack.push(AbstractStyle::Color(color));
                    if *line_type == LineType::NameShownBlock {
                        self.emit_element(writer, characters, &ElementNode::Text(name))?;
                    }
                    self.emit_elements(writer, characters, children)?;
                    self.style_stack.pop();
                    self.confirm_newline(writer)
                }
                Element::Monospaced | Element::Parameter => {
                    self.emit_elements(writer, characters, children)
                }
            },
        }
    }
}

impl<'a> Emit<'a> for ConsoleEmitter {
    fn emit(&mut self, writer: &mut impl Write, document: &Document<'a>) -> Result<()> {
        for block in &document.blocks {
            self.emit_block(writer, &document.characters, block)?;
        }
        Ok(())
    }
}
