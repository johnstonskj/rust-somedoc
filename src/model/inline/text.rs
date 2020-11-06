/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error;
use crate::model::inline::{HasInlineContent, InlineContent};
use crate::model::{ComplexContent, Style, Styled};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum TextStyle {
    Plain,
    Italic,
    Slanted,
    Quote,
    Light,
    Bold,
    Mono,
    Code,
    Strikethrough,
    Underline,
    SmallCaps,
    Superscript,
    Subscript,
}

#[derive(Clone, Debug)]
pub struct Text(String);

#[derive(Clone, Debug)]
pub struct Span {
    inner: Vec<InlineContent>,
    styles: Vec<TextStyle>,
}

// TODO: math <https://pandoc.org/MANUAL.html#math>

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for TextStyle {
    fn default() -> Self {
        Self::Plain
    }
}

impl Style for TextStyle {}

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

inline_impls!(Text);

impl Text {
    pub fn inner(&self) -> &String {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
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

impl Styled<TextStyle> for Span {
    fn styles(&self) -> &Vec<TextStyle> {
        &self.styles
    }

    fn styles_mut(&mut self) -> &mut Vec<TextStyle> {
        &mut self.styles
    }

    fn add_style(&mut self, style: TextStyle) -> error::Result<()> {
        self.styles.push(style);
        Ok(())
    }
}

impl Span {
    pub fn new(inner: &str) -> Self {
        Self::new_with_style(inner, Default::default())
    }

    pub fn new_with_style(inner: &str, style: TextStyle) -> Self {
        Self {
            inner: vec![Text::from(inner).into()],
            styles: vec![style],
        }
    }

    pub fn new_inner(inner: InlineContent) -> Self {
        Self::new_inner_with_style(inner, Default::default())
    }

    pub fn new_inner_with_style(inner: InlineContent, style: TextStyle) -> Self {
        Self {
            inner: vec![inner],
            styles: vec![style],
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
