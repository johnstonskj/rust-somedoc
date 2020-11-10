/*!
The document model here comprises a document type with nested blocks comprised of blocks and inline
content.

The model contains the following structure.

1. A Document which contains a list of [`BlockContent`](block/enum.BlockContent.html):
   1. Some block have no content of their own, such as `BlockContent::ThematicBreak`.
   1. Some block content is a basic type, such as `BlockContent::Comment` which contains a
      `String`.
   1. Some block content contains other block content, such as `BlockContent::Quote`.
   1. Most blocks contain a list of `InlineContent`.
      1. Some inline have no content of it's own, such as `InlineContent::LineBreak`.
      1. Some inline content contains a basic type, such as `InlineContent::Character` which contains a
         `char`.
      1. Some inline content contains other inline content, such as `InlineContent::Span`.
      1. Most inline types contain a single structured type.
1. A Document may also have associated metadata which may, or may not, be interpreted by a writer.
*/

use crate::error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This trait should be implemented by any type, whether block or inline, that includes.
/// [`InlineContent`](inline/enum.InlineContent.html). This allows for common treatment of such
/// types in writers and similar use cases.
///
pub trait HasInnerContent<T> {
    ///
    /// Returns `true` if the inner list of content is not empty, else `false`.
    ///
    fn has_inner(&self) -> bool {
        !self.inner().is_empty()
    }

    ///
    /// Return a reference to the inner value.
    ///
    fn inner(&self) -> &Vec<T>;

    ///
    /// Return the inner value, consuming `self`.
    ///
    fn into_inner(self) -> Vec<T>;

    ///
    /// Return a mutable reference to the inner list of content.
    ///
    fn inner_mut(&mut self) -> &mut Vec<T>;

    ///
    /// Add `content` to the inner list of content. If there is some consistency condition that
    /// would be broken by this addition an error will be returned.
    ///
    fn add_content(&mut self, content: T) -> error::Result<()>;
}

///
/// A marker trait denoting that a type, most likely an enum, should be treated as a syle by the
/// type [`HasStyles`](trait.HasStyles.html). All styles **must** support `Default` to denote the
/// *un-styled* case.
///
pub trait Style: Default {}

///
/// This trait should be implemented by any type, whether block or inline, that can be styled. The
/// type parameter `T` denotes the style information to apply.
///
pub trait HasStyles<T: Style> {
    ///
    /// Returns `true` if the list of applied styles is not empty, else `false`.
    ///
    fn has_styles(&self) -> bool {
        self.styles().is_empty()
    }

    ///
    /// Return a reference to the list of applied styles.
    ///
    fn styles(&self) -> &Vec<T>;

    ///
    /// Return a mutable reference to the list of applied styles.
    ///
    fn styles_mut(&mut self) -> &mut Vec<T>;

    ///
    /// Add `content` to the list of applied styles. If there is some consistency condition that
    /// would be broken by this addition an error will be returned.
    ///
    fn add_style(&mut self, style: T) -> error::Result<()>;
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod document;
pub use document::Document;

pub mod inline;

pub mod block;

//pub mod visitor;
