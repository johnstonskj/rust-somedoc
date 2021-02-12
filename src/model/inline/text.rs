use crate::error;
use crate::model::inline::Character;
use crate::model::inline::{HasInlineContent, InlineContent};
use crate::model::{HasInnerContent, HasStyles, Style};
use std::ops::Deref;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// These represent styles that may be applied to a single `Span`. Note that they will be applied in
/// the order they were added to the span. Writers should simply ignore styles they do not support.
///
#[derive(Clone, Debug, PartialEq)]
pub enum SpanStyle {
    Plain,
    Italic,
    Bold,
    Mono,
    Code,
    Strikethrough,
    Underline,
    SmallCaps,
    Superscript,
    Subscript,
    Sized(Size),
}

///
/// A size modifier for styling a `Span`.
///
#[derive(Clone, Debug, PartialEq)]
pub enum Size {
    Largest,
    Larger,
    Large,
    Normal,
    Small,
    Smaller,
    Smallest,
}

///
/// A span consists of a list of styles to apply to an inner list of inline content.
///
#[derive(Clone, Debug)]
pub struct Span {
    inner: Vec<InlineContent>,
    styles: Vec<SpanStyle>,
}

///
/// A `Text` instance holds simple plain, un-styled, text.
///
#[derive(Clone, Debug)]
pub struct Text(String);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for SpanStyle {
    fn default() -> Self {
        Self::Plain
    }
}

impl Style for SpanStyle {}

// ------------------------------------------------------------------------------------------------

impl Default for Size {
    fn default() -> Self {
        Self::Normal
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Text {
    fn default() -> Self {
        Self(String::new())
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Deref for Text {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

inline_impls!(Text);

inner_impl!(Text, String);

impl Text {
    /// Create a new Text instance from the string.
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Span {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            styles: Default::default(),
        }
    }
}

inline_impls!(Span);

has_inline_impls!(Span);

has_styles_impls!(Span, SpanStyle);

impl Span {
    /// Create a new span, with the provided style, that includes a `Text` instance.
    pub fn with_style(inner: &str, style: SpanStyle) -> Self {
        Self {
            inner: vec![Text::from(inner).into()],
            styles: vec![style],
        }
    }

    /// Create a new span, with the provided style, that includes a `InlineContent` instance.
    pub fn inner_with_style(inner: InlineContent, style: SpanStyle) -> Self {
        Self {
            inner: vec![inner],
            styles: vec![style],
        }
    }

    /// Create a new span containing simply a space character.
    pub fn space() -> Self {
        Self::from(InlineContent::from(Character::Space.into()))
    }
}
