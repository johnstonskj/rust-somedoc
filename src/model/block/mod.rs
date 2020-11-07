/*!
One-line description.

More detailed description, with

# Example

*/

use crate::model::inline::image::Image;
use crate::model::ComplexContent;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum BlockContent {
    Comment(String),
    Heading(Heading),
    Image(Image),
    List(List),
    DefinitionList(DefinitionList),
    Formatted(Formatted),
    CodeBlock(CodeBlock),
    Paragraph(Paragraph),
    Quote(Quote),
    Table(Table),
    ThematicBreak,
}

pub trait HasBlockContent: Default + ComplexContent<BlockContent> {
    fn heading(inner: Heading) -> Self {
        let mut new_self = Self::default();
        new_self.add_heading(inner);
        new_self
    }

    fn image(inner: Image) -> Self {
        let mut new_self = Self::default();
        new_self.add_image(inner);
        new_self
    }

    fn list(inner: List) -> Self {
        let mut new_self = Self::default();
        new_self.add_list(inner);
        new_self
    }

    fn definition_list(inner: DefinitionList) -> Self {
        let mut new_self = Self::default();
        new_self.add_definition_list(inner);
        new_self
    }

    fn formatted(inner: Formatted) -> Self {
        let mut new_self = Self::default();
        new_self.add_formatted(inner);
        new_self
    }

    fn code_block(inner: CodeBlock) -> Self {
        let mut new_self = Self::default();
        new_self.add_code_block(inner);
        new_self
    }

    fn paragraph(inner: Paragraph) -> Self {
        let mut new_self = Self::default();
        new_self.add_paragraph(inner);
        new_self
    }

    fn block_quote(inner: Quote) -> Self {
        let mut new_self = Self::default();
        new_self.add_block_quote(inner);
        new_self
    }

    fn table(inner: Table) -> Self {
        let mut new_self = Self::default();
        new_self.add_table(inner);
        new_self
    }

    fn thematic_break() -> Self {
        let mut new_self = Self::default();
        new_self.add_thematic_break();
        new_self
    }

    fn add_heading(&mut self, inner: Heading) {
        self.add_content(inner.into()).unwrap()
    }

    fn add_image(&mut self, inner: Image) {
        self.add_content(inner.into()).unwrap()
    }

    fn add_list(&mut self, inner: List) {
        self.add_content(inner.into()).unwrap()
    }

    fn add_definition_list(&mut self, inner: DefinitionList) {
        self.add_content(inner.into()).unwrap()
    }

    fn add_formatted(&mut self, inner: Formatted) {
        self.add_content(inner.into()).unwrap()
    }

    fn add_code_block(&mut self, inner: CodeBlock) {
        self.add_content(inner.into()).unwrap()
    }

    fn add_paragraph(&mut self, inner: Paragraph) {
        self.add_content(inner.into()).unwrap()
    }

    fn add_block_quote(&mut self, inner: Quote) {
        self.add_content(inner.into()).unwrap()
    }

    fn add_table(&mut self, inner: Table) {
        self.add_content(inner.into()).unwrap()
    }

    fn add_thematic_break(&mut self) {
        self.add_content(BlockContent::ThematicBreak).unwrap()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod code;
pub use code::{CodeBlock, Formatted};

pub mod heading;
pub use heading::{Heading, HeadingKind};

pub mod list;
pub use list::{Item, List, ListItem, ListKind};

pub mod definition_list;
pub use definition_list::{Definition, DefinitionList, DefinitionListItem, DefinitionPart};

pub mod paragraph;
pub use paragraph::{Alignment, Paragraph, ParagraphStyle};

pub mod quote;
pub use quote::Quote;

pub mod table;
pub use table::{Cell, Column, Row, Table};
