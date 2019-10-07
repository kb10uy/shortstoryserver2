use crate::{
    document::{Block, BlockNode, CharacterSet, CharacterType, Document, Element, ElementNode},
    emitter::Emit,
};
use ansi_term::{Colour, Style};
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

/// Emitter for virtual terminal.
pub struct ConsoleEmitter {
    style_stack: Vec<AbstractStyle>,
    fresh_newline: bool,
}

impl ConsoleEmitter {
    pub fn new() -> ConsoleEmitter {
        ConsoleEmitter {
            style_stack: vec![],
            fresh_newline: true,
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

    fn confirm_newline(&mut self, writer: &mut impl Write) -> Result<()> {
        if !self.fresh_newline {
            self.fresh_newline = true;
            writeln!(writer)?;
        }
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
                self.confirm_newline(writer)?;
                writeln!(writer)?;

                self.style_stack.push(AbstractStyle::Color(Colour::Yellow));
                self.emit_element(writer, characters, &ElementNode::Text("###### "))?;
                self.emit_elements(writer, characters, &block.children)?;
                self.emit_element(writer, characters, &ElementNode::Text(" ######"))?;
                self.style_stack.pop();

                self.fresh_newline = true;
                writeln!(writer)
            }
            Block::Subsection => {
                self.confirm_newline(writer)?;
                writeln!(writer)?;

                self.style_stack.push(AbstractStyle::Color(Colour::Yellow));
                self.emit_element(writer, characters, &ElementNode::Text("====== "))?;
                self.emit_elements(writer, characters, &block.children)?;
                self.emit_element(writer, characters, &ElementNode::Text(" ======"))?;
                self.style_stack.pop();

                self.fresh_newline = true;
                writeln!(writer)
            }
            Block::Horizontal => {
                self.confirm_newline(writer)?;
                self.fresh_newline = true;
                writeln!(writer, "--------------------------------------------------------------------------------")
            }
            Block::Paragraph => {
                self.confirm_newline(writer)?;
                writeln!(writer)?;
                self.emit_elements(writer, characters, &block.children)?;
                self.confirm_newline(writer)
            }
            Block::Quotation => {
                self.confirm_newline(writer)?;
                self.style_stack
                    .push(AbstractStyle::Color(Colour::RGB(127, 127, 127)));
                self.emit_elements(writer, characters, &block.children)?;
                self.style_stack.pop();

                self.fresh_newline = true;
                writeln!(writer)
            }
            Block::UnorderedList => {
                self.confirm_newline(writer)?;
                self.emit_elements(writer, characters, &block.children)
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
                self.fresh_newline = false;
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
                    self.fresh_newline = true;
                    writeln!(writer)
                }
                Element::Newline => {
                    self.fresh_newline = true;
                    writeln!(writer)
                }
                Element::Line(id) => {
                    self.confirm_newline(writer)?;
                    self.style_stack.push(AbstractStyle::Color(Colour::Green));
                    self.emit_element(writer, characters, &ElementNode::Text(&id))?;
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
