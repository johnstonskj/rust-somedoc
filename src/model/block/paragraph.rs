use crate::error;
use crate::model::block::BlockContent;
use crate::model::inline::{HasInlineContent, InlineContent, Span};
use crate::model::{HasInnerContent, HasStyles, Style};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The text alignment for this paragraph, used in ParagraphStyle.
///
#[derive(Clone, Debug, PartialEq)]
pub enum Alignment {
    Left,
    Right,
    Centered,
    Justified,
}

// TODO: line blocks <https://pandoc.org/MANUAL.html#line-blocks>

///
/// The styles for the paragraph.
///
#[derive(Clone, Debug, PartialEq)]
pub enum ParagraphStyle {
    Plain,
    Abstract,
    Aligned(Alignment),
}

///
/// A paragraph is a bounded block of inline content, usually text.
///
#[derive(Clone, Debug)]
pub struct Paragraph {
    inner: Vec<InlineContent>,
    styles: Vec<ParagraphStyle>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Alignment {
    fn default() -> Self {
        Self::Left
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for ParagraphStyle {
    fn default() -> Self {
        Self::Plain
    }
}

impl Style for ParagraphStyle {}

impl Default for Paragraph {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            styles: Default::default(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

block_impls!(Paragraph);

has_inline_impls!(Paragraph);

has_styles_impls!(Paragraph, ParagraphStyle);

impl Paragraph {
    pub fn new(inner: &str, style: ParagraphStyle) -> Self {
        Self {
            inner: vec![Span::plain_str(inner).into()],
            styles: vec![style],
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn set_left_aligned(&mut self) {
        self.add_style(ParagraphStyle::Aligned(Alignment::Left))
            .unwrap()
    }

    pub fn set_right_aligned(&mut self) {
        self.add_style(ParagraphStyle::Aligned(Alignment::Right))
            .unwrap()
    }

    pub fn set_ragged_left(&mut self) {
        self.add_style(ParagraphStyle::Aligned(Alignment::Right))
            .unwrap()
    }

    pub fn set_ragged_right(&mut self) {
        self.add_style(ParagraphStyle::Aligned(Alignment::Left))
            .unwrap()
    }

    pub fn set_centered(&mut self) {
        self.add_style(ParagraphStyle::Aligned(Alignment::Centered))
            .unwrap()
    }

    pub fn set_justified(&mut self) {
        self.add_style(ParagraphStyle::Aligned(Alignment::Justified))
            .unwrap()
    }
}
