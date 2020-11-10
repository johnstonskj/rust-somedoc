use crate::error;
use crate::model::block::BlockContent;
use crate::model::inline::{HasInlineContent, InlineContent, Text};
use crate::model::HasInnerContent;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The level of the heading denoting it's hierarchy.
///
#[derive(Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum HeadingLevel {
    Title = 0,
    Section,
    SubSection,
    SubSubSection,
    SubSubSubSection,
    SubSubSubSubSection,
    Paragraph,
}

///
/// A heading consists of a level and text. The level is indicated by `HeadingLevel` and the text
/// is an inner content list of `InlineContent` values.
///
#[derive(Clone, Debug)]
pub struct Heading {
    level: HeadingLevel,
    inner: Vec<InlineContent>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for HeadingLevel {
    fn default() -> Self {
        Self::Section
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Heading {
    fn default() -> Self {
        Self {
            level: Default::default(),
            inner: Default::default(),
        }
    }
}

block_impls!(Heading);

has_inline_impls!(Heading);

impl Heading {
    pub fn new(inner: &str, kind: HeadingLevel) -> Self {
        Self {
            level: kind,
            inner: vec![Text::from(inner).into()],
        }
    }

    pub fn title(inner: &str) -> Self {
        Self::new(inner, HeadingLevel::Title)
    }

    pub fn section(inner: &str) -> Self {
        Self::new(inner, HeadingLevel::Section)
    }

    pub fn sub_section(inner: &str) -> Self {
        Self::new(inner, HeadingLevel::SubSection)
    }

    pub fn sub_sub_section(inner: &str) -> Self {
        Self::new(inner, HeadingLevel::SubSubSection)
    }

    pub fn sub_sub_sub_section(inner: &str) -> Self {
        Self::new(inner, HeadingLevel::SubSubSubSection)
    }

    pub fn sub_sub_sub_sub_section(inner: &str) -> Self {
        Self::new(inner, HeadingLevel::SubSubSubSubSection)
    }

    pub fn paragraph(inner: &str) -> Self {
        Self::new(inner, HeadingLevel::Paragraph)
    }

    // --------------------------------------------------------------------------------------------

    pub fn level(&self) -> &HeadingLevel {
        &self.level
    }

    pub fn level_as_u8(&self) -> u8 {
        self.level.clone() as u8
    }
}
