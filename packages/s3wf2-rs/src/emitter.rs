pub mod html;

#[cfg(feature = "cli")]
pub mod console;

use crate::document::Document;
use std::io::{prelude::*, Result};

/// The trait which converts `Document` into other formats.
pub trait Emit<'a> {
    /// Emits formatted document.
    ///
    /// # Parameters
    /// * `writer` - The `Write` object into which you want to write formatted text.
    fn emit(&mut self, writer: &mut impl Write, document: &Document<'a>) -> Result<()>;
}

/// The trait which extracts indices in document.
pub trait ExtractIndices<'d, 's: 'd> {
    /// Index item type.
    type IndexItem;

    /// The Iterator type which iterates IndexItem.
    type IndexItemIter: Iterator<Item = Self::IndexItem>;

    /// Returns an iterator which lists the index items.
    fn indices(&self, document: &'d Document<'s>) -> Self::IndexItemIter;
}
