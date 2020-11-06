/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error;
use crate::model::blocks::BlockContent;
use crate::model::inline::{HasInlineContent, InlineContent, Text};
use crate::model::ComplexContent;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum HeadingKind {
    Title,
    Subtitle,
    Chapter,
    Heading(u8),
}

#[derive(Clone, Debug)]
pub struct Heading {
    kind: HeadingKind,
    inner: Vec<InlineContent>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for HeadingKind {
    fn default() -> Self {
        Self::Heading(1)
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Heading {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            inner: Default::default(),
        }
    }
}

block_impls!(Heading);

has_inline_impls!(Heading);

impl Heading {
    pub fn new(inner: &str, kind: HeadingKind) -> Self {
        Self {
            kind,
            inner: vec![Text::new(inner).into()],
        }
    }

    pub fn title(inner: &str) -> Self {
        Self::new(inner, HeadingKind::Title)
    }

    pub fn subtitle(inner: &str) -> Self {
        Self::new(inner, HeadingKind::Subtitle)
    }

    pub fn chapter(inner: &str) -> Self {
        Self::new(inner, HeadingKind::Chapter)
    }

    pub fn heading_1(inner: &str) -> Self {
        Self::new(inner, HeadingKind::Heading(1))
    }

    pub fn heading_2(inner: &str) -> Self {
        Self::new(inner, HeadingKind::Heading(2))
    }

    pub fn heading_3(inner: &str) -> Self {
        Self::new(inner, HeadingKind::Heading(3))
    }

    pub fn heading_4(inner: &str) -> Self {
        Self::new(inner, HeadingKind::Heading(4))
    }

    pub fn heading_5(inner: &str) -> Self {
        Self::new(inner, HeadingKind::Heading(5))
    }

    pub fn heading_6(inner: &str) -> Self {
        Self::new(inner, HeadingKind::Heading(6))
    }

    // --------------------------------------------------------------------------------------------

    pub fn kind(&self) -> &HeadingKind {
        &self.kind
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
