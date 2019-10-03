pub mod html;

use crate::document::Document;

/// The trait which converts `Document` into other formats.
pub trait Emit<'a> {
    /// The main output type (e.g. `String`).
    type Output;

    /// The type which represents anchors to any position in document.
    type Anchors;

    /// Emits formatted document.
    fn emit(&self, document: &Document<'a>) -> Self::Output;

    /// Returns an Iterator which returns anchors to sections.
    fn section_anchors(&self, document: &Document<'a>) -> Self::Anchors;
}
