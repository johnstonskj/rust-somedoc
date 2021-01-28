/*!
This module is the root of a set of types that represent *block* content; that is, content that
stands on it's own such as a complete paragraph.
*/

use crate::model::inline::Image;
use crate::model::HasInnerContent;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The set of supported block content types.
///
#[derive(Clone, Debug)]
pub enum BlockContent {
    /// A comment; this may be written into markup but not included in a rendered version.
    Comment(String),
    /// A section heading.
    Heading(Heading),
    /// A block containing an image only.
    Image(Image),
    /// Block formatted math formula.
    MathBlock(MathBlock),
    /// An ordered, or unordered, and possibly nested list.
    List(List),
    /// A definition list.
    DefinitionList(DefinitionList),
    /// Pre-formatted output[
    Formatted(Formatted),
    /// A code block with the intent that this will be syntax highlighted.
    CodeBlock(CodeBlock),
    /// A paragraph of text or other inline content.
    Paragraph(Paragraph),
    /// A block quote, these may be nested.
    Quote(Quote),
    /// A table with columns and rows.
    Table(Table),
    /// A visual break between themes.
    ThematicBreak,
}

///
/// This trait is implemented by any type that contains, as content, a list of block content instances.
/// Implementers of this trait may be themselves either block content objects.
///
/// Note that the `add_` methods all return `&mut Self` and so calls to these may be chained.
///
pub trait HasBlockContent: Default + HasInnerContent<BlockContent> {
    /// Create a new block content container from the provided content item.
    fn from(inner: BlockContent) -> Self {
        let mut new_self = Self::default();
        new_self.add_content(inner).unwrap();
        new_self
    }

    /// Create a new block content container from the provided `String` content item.
    fn comment(inner: String) -> Self {
        let mut new_self = Self::default();
        new_self.add_comment_str(&inner);
        new_self
    }

    /// Create a new block content container from the provided `Heading` content item.
    fn heading(inner: Heading) -> Self {
        let mut new_self = Self::default();
        new_self.add_heading(inner);
        new_self
    }

    /// Create a new block content container from the provided `Image` content item.
    fn image(inner: Image) -> Self {
        let mut new_self = Self::default();
        new_self.add_image(inner);
        new_self
    }

    /// Create a new block content container from the provided `List` content item.
    fn list(inner: List) -> Self {
        let mut new_self = Self::default();
        new_self.add_list(inner);
        new_self
    }

    /// Create a new block content container from the provided `DefinitionList` content item.
    fn definition_list(inner: DefinitionList) -> Self {
        let mut new_self = Self::default();
        new_self.add_definition_list(inner);
        new_self
    }

    /// Create a new block content container from the provided `Formatted` content item.
    fn formatted(inner: Formatted) -> Self {
        let mut new_self = Self::default();
        new_self.add_formatted(inner);
        new_self
    }

    /// Create a new block content container from the provided `CodeBlock` content item.
    fn code_block(inner: CodeBlock) -> Self {
        let mut new_self = Self::default();
        new_self.add_code_block(inner);
        new_self
    }

    /// Create a new block content container from the provided `Paragraph` content item.
    fn paragraph(inner: Paragraph) -> Self {
        let mut new_self = Self::default();
        new_self.add_paragraph(inner);
        new_self
    }

    /// Create a new block content container from the provided `Quote` content item.
    fn block_quote(inner: Quote) -> Self {
        let mut new_self = Self::default();
        new_self.add_block_quote(inner);
        new_self
    }

    /// Create a new block content container from the provided `Table` content item.
    fn table(inner: Table) -> Self {
        let mut new_self = Self::default();
        new_self.add_table(inner);
        new_self
    }

    /// Create a new block content container with a thematic break.
    fn thematic_break() -> Self {
        let mut new_self = Self::default();
        new_self.add_thematic_break();
        new_self
    }

    // --------------------------------------------------------------------------------------------

    /// Add the provided `Comment` to this container's inner content.
    fn add_comment_str(&mut self, inner: &str) -> &mut Self {
        self.add_content(BlockContent::Comment(inner.to_string()))
            .unwrap();
        self
    }

    /// Add the provided `Heading` to this container's inner content.
    fn add_heading(&mut self, inner: Heading) -> &mut Self {
        self.add_content(inner.into()).unwrap();
        self
    }

    /// Add the provided `Image` to this container's inner content.
    fn add_image(&mut self, inner: Image) -> &mut Self {
        self.add_content(inner.into()).unwrap();
        self
    }

    /// Add the provided `List` to this container's inner content.
    fn add_list(&mut self, inner: List) -> &mut Self {
        self.add_content(inner.into()).unwrap();
        self
    }

    /// Add the provided `DefinitionList` to this container's inner content.
    fn add_definition_list(&mut self, inner: DefinitionList) -> &mut Self {
        self.add_content(inner.into()).unwrap();
        self
    }

    /// Add the provided `Formatted` to this container's inner content.
    fn add_formatted(&mut self, inner: Formatted) -> &mut Self {
        self.add_content(inner.into()).unwrap();
        self
    }

    /// Add the provided `CodeBlock` to this container's inner content.
    fn add_code_block(&mut self, inner: CodeBlock) -> &mut Self {
        self.add_content(inner.into()).unwrap();
        self
    }

    /// Add the provided `Paragraph` to this container's inner content.
    fn add_paragraph(&mut self, inner: Paragraph) -> &mut Self {
        self.add_content(inner.into()).unwrap();
        self
    }

    /// Add the provided `Quote` to this container's inner content.
    fn add_block_quote(&mut self, inner: Quote) -> &mut Self {
        self.add_content(inner.into()).unwrap();
        self
    }

    /// Add the provided `Table` to this container's inner content.
    fn add_table(&mut self, inner: Table) -> &mut Self {
        self.add_content(inner.into()).unwrap();
        self
    }

    /// Add a `ThematicBreak` to this container's inner content.
    fn add_thematic_break(&mut self) -> &mut Self {
        self.add_content(BlockContent::ThematicBreak).unwrap();
        self
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod caption;
pub use caption::{Caption, Captioned};

#[doc(hidden)]
pub mod code;
pub use code::{CodeBlock, Formatted};

#[doc(hidden)]
pub mod heading;
pub use heading::{Heading, HeadingLevel};

#[doc(hidden)]
pub mod list;
pub use list::{Item, List, ListItem, ListKind};

#[doc(hidden)]
pub mod definition_list;
pub use definition_list::{Definition, DefinitionList, DefinitionListItem, DefinitionPart};

#[doc(hidden)]
pub mod math;
pub use math::MathBlock;

#[doc(hidden)]
pub mod paragraph;
pub use paragraph::{Alignment, Paragraph, ParagraphStyle};

#[doc(hidden)]
pub mod quote;
pub use quote::Quote;

#[doc(hidden)]
pub mod table;
pub use table::{Cell, Column, Row, Table};
