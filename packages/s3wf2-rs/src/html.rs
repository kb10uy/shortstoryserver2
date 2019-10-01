use std::borrow::Borrow;
use std::io::{Result as IoResult, Write};

/// Can generate HTML string.
trait EmitHtml {
    /// Output HTML to String.
    fn emit_html(&self) -> String;

    /// Output HTML to Write object.
    fn write_html(&self, writer: &mut impl Write) -> IoResult<()>;
}

impl<T: Borrow<str>> EmitHtml for T {
    fn emit_html(&self) -> String {
        let original: &str = self.borrow();

        original
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("&", "&amp;")
            .replace("'", "&quot;")
    }

    fn write_html(&self, writer: &mut impl Write) -> IoResult<()> {
        for ch in self.borrow().chars() {
            match ch {
                '>' => write!(writer, "&gt;")?,
                '<' => write!(writer, "&lt;")?,
                '&' => write!(writer, "&amp;")?,
                '\'' => write!(writer, "&apos;")?,
                '"' => write!(writer, "&quot;")?,
                other => write!(writer, "{}", other)?,
            }
        }
        Ok(())
    }
}

enum Node<'a> {
    Text(&'a str),
    Singular {
        tag: String,
        classes: Vec<String>,
    },
    Surrounded {
        tag: String,
        classes: Vec<String>,
        children: Vec<Node<'a>>,
    },
}

enum BlockNode<'a> {
    Singular {
        tag: String,
        classes: Vec<String>,
    },
    Surrounded {
        tag: String,
        classes: Vec<String>,
        children: Vec<Node<'a>>,
    },
}
