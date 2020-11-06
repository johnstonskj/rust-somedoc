/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error;
use crate::model::inline::InlineContent;
use crate::model::{Style, Styled};

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
pub struct Text {
    inner: String,
    styles: Vec<TextStyle>,
}

// TODO: super/sub script <https://pandoc.org/MANUAL.html#superscripts-and-subscripts>
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
        Self {
            inner: Default::default(),
            styles: Default::default(),
        }
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Self::new(&s)
    }
}

impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

inline_impls!(Text);

impl Styled<TextStyle> for Text {
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

impl Text {
    pub fn new(inner: &str) -> Self {
        Self::new_with_style(inner, Default::default())
    }

    pub fn new_with_style(inner: &str, style: TextStyle) -> Self {
        Self {
            inner: inner.to_string(),
            styles: vec![style],
        }
    }

    pub fn plain(inner: &str) -> Self {
        Self::new_with_style(inner, TextStyle::Plain)
    }

    pub fn italic(inner: &str) -> Self {
        Self::new_with_style(inner, TextStyle::Italic)
    }

    pub fn slanted(inner: &str) -> Self {
        Self::new_with_style(inner, TextStyle::Slanted)
    }

    pub fn quote(inner: &str) -> Self {
        Self::new_with_style(inner, TextStyle::Quote)
    }

    pub fn light(inner: &str) -> Self {
        Self::new_with_style(inner, TextStyle::Light)
    }

    pub fn bold(inner: &str) -> Self {
        Self::new_with_style(inner, TextStyle::Bold)
    }

    pub fn mono(inner: &str) -> Self {
        Self::new_with_style(inner, TextStyle::Mono)
    }

    pub fn code(inner: &str) -> Self {
        Self::new_with_style(inner, TextStyle::Code)
    }

    pub fn strikethrough(inner: &str) -> Self {
        Self::new_with_style(inner, TextStyle::Strikethrough)
    }

    pub fn underline(inner: &str) -> Self {
        Self::new_with_style(inner, TextStyle::Underline)
    }

    pub fn small_caps(inner: &str) -> Self {
        Self::new_with_style(inner, TextStyle::SmallCaps)
    }

    pub fn superscript(inner: &str) -> Self {
        Self::new_with_style(inner, TextStyle::Superscript)
    }

    pub fn subscript(inner: &str) -> Self {
        Self::new_with_style(inner, TextStyle::Subscript)
    }

    // --------------------------------------------------------------------------------------------

    pub fn set_text(&mut self, text: &str) {
        self.inner = text.to_string();
    }

    pub fn add_style(&mut self, style: TextStyle) {
        self.styles.push(style);
    }

    // --------------------------------------------------------------------------------------------

    pub fn text(&self) -> &String {
        &self.inner
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
