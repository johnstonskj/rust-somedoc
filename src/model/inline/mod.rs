/*!
One-line description.

More detailed description, with

# Example

*/

use crate::model::ComplexContent;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum InlineContent {
    Anchor(Anchor),
    HyperLink(HyperLink),
    Image(Image),
    Text(Text),
    Character(Character),
    LineBreak,
}

pub trait HasInlineContent: Default + ComplexContent<InlineContent> {
    fn from(inner: InlineContent) -> Self {
        let mut new_self = Self::default();
        new_self.add_content(inner).unwrap();
        new_self
    }

    fn from_all(inner: Vec<InlineContent>) -> Self {
        let mut new_self = Self::default();
        let mut inner = inner;
        for item in inner.drain(..) {
            new_self.add_content(item).unwrap();
        }
        new_self
    }

    fn anchor(inner: Anchor) -> Self {
        let mut new_self = Self::default();
        new_self.add_anchor(inner);
        new_self
    }

    fn link(inner: HyperLink) -> Self {
        let mut new_self = Self::default();
        new_self.add_link(inner);
        new_self
    }

    fn image(inner: Image) -> Self {
        let mut new_self = Self::default();
        new_self.add_image(inner);
        new_self
    }

    fn text(inner: Text) -> Self {
        let mut new_self = Self::default();
        new_self.add_text(inner);
        new_self
    }

    fn text_str(inner: &str) -> Self {
        let mut new_self = Self::default();
        new_self.add_text(inner.into());
        new_self
    }

    fn character(inner: Character) -> Self {
        let mut new_self = Self::default();
        new_self.add_character(inner);
        new_self
    }

    fn line_break() -> Self {
        let mut new_self = Self::default();
        new_self.add_line_break();
        new_self
    }

    fn add_anchor(&mut self, inner: Anchor) {
        self.add_content(inner.into()).unwrap()
    }

    fn add_link(&mut self, inner: HyperLink) {
        self.add_content(inner.into()).unwrap()
    }

    fn add_image(&mut self, inner: Image) {
        self.add_content(inner.into()).unwrap()
    }

    fn add_text(&mut self, inner: Text) {
        self.add_content(inner.into()).unwrap()
    }

    fn add_text_str(&mut self, inner: &str) {
        let t: Text = inner.into();
        self.add_content(t.into()).unwrap()
    }

    fn add_character(&mut self, inner: Character) {
        self.add_content(inner.into()).unwrap()
    }

    fn add_line_break(&mut self) {
        self.add_content(InlineContent::LineBreak).unwrap()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! inline_impls {
    ($name:ident) => {
        impl Into<InlineContent> for $name {
            fn into(self) -> InlineContent {
                InlineContent::$name(self)
            }
        }
    };
}

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

pub mod character;
pub use character::Character;

pub mod anchor;
pub use anchor::Anchor;

pub mod link;
pub use link::{HyperLink, HyperLinkTarget};

pub mod image;
pub use image::Image;

pub mod text;
pub use text::{Text, TextStyle};
