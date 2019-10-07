use crate::{document::{Document, Block, BlockNode, Element, ElementNode}, emitter::Emit};
use ansi_term;
use std::io::{prelude::*, Result};

enum AbstractStyle {
    Underline,
    Bold,
    AnsiColor(u8),
    ExtendedColor(u8),
    TrueColor(u8, u8, u8),
}

/// Emitter for virtual terminal.
pub struct ConsoleEmitter {
    style_stack: Vec<AbstractStyle>,
}

impl ConsoleEmitter {
    pub fn new() -> ConsoleEmitter {
        ConsoleEmitter {
            style_stack: vec![],
        }
    }

    fn current_style(&self, writer: &mut impl Write) -> ansi_term::Style {
        ansi_term::Style::from(ansi_term::Colour::Red)
    }

    fn emit_block(&self, writer: &mut impl Write, block: &BlockNode) -> Result<()> {
        match block.kind {
            Block::Section => {
            }
            _ => {}
        }
        Ok(())
    }
}

impl<'a> Emit<'a> for ConsoleEmitter {
    fn emit(&mut self, writer: &mut impl Write, document: &Document<'a>) -> Result<()> {
        Ok(())
    }
}
