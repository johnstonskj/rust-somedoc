use crate::error;
use crate::model::block::{BlockContent, Label};
use crate::model::inline::{HasInlineContent, InlineContent, Text};
use crate::model::{block::HasLabel, HasInnerContent};
use std::convert::TryFrom;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The level of the heading denoting it's hierarchy.
///
#[derive(Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum HeadingLevel {
    /// Level 1 heading/section (top-level).
    Section = 1,
    /// Level 2 heading/section.
    SubSection,
    /// Level 3 heading/section.
    SubSubSection,
    /// Level 4 heading/section.
    SubSubSubSection,
    /// Level 5 heading/section.
    SubSubSubSubSection,
    /// Level 6 heading/section.
    SubSubSubSubSubSection,
    /// Level 7 heading/section.
    SubSubSubSubSubSubSection,
}

///
/// A heading consists of a level and text. The level is indicated by `HeadingLevel` and the text
/// is an inner content list of `InlineContent` values.
///
#[derive(Clone, Debug)]
pub struct Heading {
    label: Option<Label>,
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
            label: None,
            level: Default::default(),
            inner: Default::default(),
        }
    }
}

impl TryFrom<u8> for HeadingLevel {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(HeadingLevel::Section),
            2 => Ok(HeadingLevel::SubSection),
            3 => Ok(HeadingLevel::SubSubSection),
            4 => Ok(HeadingLevel::SubSubSubSection),
            5 => Ok(HeadingLevel::SubSubSubSubSection),
            6 => Ok(HeadingLevel::SubSubSubSubSubSection),
            7 => Ok(HeadingLevel::SubSubSubSubSubSubSection),
            _ => Err(()),
        }
    }
}

label_impl!(Heading);

block_impls!(Heading);

has_inline_impls!(Heading);

impl Heading {
    /// Create a new heading with the given text and level.
    pub fn new(inner: &str, kind: HeadingLevel) -> Self {
        Self {
            label: None,
            level: kind,
            inner: vec![Text::from(inner).into()],
        }
    }

    /// Create a new heading with the given text and level of `HeadingLevel::Section`.
    pub fn section(inner: &str) -> Self {
        Self::new(inner, HeadingLevel::Section)
    }

    /// Create a new heading with the given text and level of `HeadingLevel::SubSection`.
    pub fn sub_section(inner: &str) -> Self {
        Self::new(inner, HeadingLevel::SubSection)
    }

    /// Create a new heading with the given text and level of `HeadingLevel::SubSubSection`.
    pub fn sub_sub_section(inner: &str) -> Self {
        Self::new(inner, HeadingLevel::SubSubSection)
    }

    /// Create a new heading with the given text and level of `HeadingLevel::SubSubSubSection`.
    pub fn sub_sub_sub_section(inner: &str) -> Self {
        Self::new(inner, HeadingLevel::SubSubSubSection)
    }

    /// Create a new heading with the given text and level of `HeadingLevel::SubSubSubSubSection`.
    pub fn sub_sub_sub_sub_section(inner: &str) -> Self {
        Self::new(inner, HeadingLevel::SubSubSubSubSection)
    }

    /// Create a new heading with the given text and level of `HeadingLevel::SubSubSubSubSubSection`.
    pub fn sub_sub_sub_sub_sub_section(inner: &str) -> Self {
        Self::new(inner, HeadingLevel::SubSubSubSubSubSection)
    }

    /// Create a new heading with the given text and level of `HeadingLevel::SubSubSubSubSubSubSection`.
    pub fn sub_sub_sub_sub_sub_sub_section(inner: &str) -> Self {
        Self::new(inner, HeadingLevel::SubSubSubSubSubSubSection)
    }

    // --------------------------------------------------------------------------------------------

    /// Return the level of this heading.
    pub fn level(&self) -> &HeadingLevel {
        &self.level
    }

    /// Return the level of this heading as a `u8` value.
    pub fn level_as_u8(&self) -> u8 {
        self.level.clone() as u8
    }

    /// Set the level of this heading.
    pub fn set_level(&mut self, level: HeadingLevel) -> &mut Self {
        self.level = level;
        self
    }
}
