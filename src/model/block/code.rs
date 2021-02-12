use crate::model::block::{BlockContent, Caption, HasCaption};
use crate::model::block::{HasLabel, Label};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A pre-formatted block of text, no formatting should be done on the inner content.
///
#[derive(Clone, Debug)]
pub struct Formatted {
    label: Option<Label>,
    inner: String,
}

///
/// A block of code, syntax highlighting may be used, especially if `language` is specified.
///
#[derive(Clone, Debug)]
pub struct CodeBlock {
    label: Option<Label>,
    code: String,
    language: Option<String>,
    caption: Option<Caption>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<String> for Formatted {
    fn from(inner: String) -> Self {
        Self { label: None, inner }
    }
}

impl From<&str> for Formatted {
    fn from(inner: &str) -> Self {
        Self {
            label: None,
            inner: inner.to_string(),
        }
    }
}

label_impl!(Formatted);

block_impls!(Formatted);

inner_impl!(Formatted, String, inner);

// ------------------------------------------------------------------------------------------------

impl From<String> for CodeBlock {
    fn from(code: String) -> Self {
        Self::from(code.as_str())
    }
}

impl From<&str> for CodeBlock {
    fn from(code: &str) -> Self {
        Self {
            label: None,
            code: code.to_string(),
            language: None,
            caption: None,
        }
    }
}

label_impl!(CodeBlock);

block_impls!(CodeBlock);

has_captioned_impls!(CodeBlock);

impl CodeBlock {
    /// Construct a new code block from the provided string ands language name.
    pub fn with_language(code: &str, language: &str) -> Self {
        Self {
            label: None,
            code: code.to_string(),
            language: Some(language.to_string()),
            caption: None,
        }
    }

    /// Return the inner code value as a string.
    pub fn code(&self) -> &String {
        &self.code
    }

    /// Set the code value for this block.
    pub fn set_code(&mut self, code: &str) -> &mut Self {
        self.code = code.to_string();
        self
    }

    /// Return the inner language name as a string, if present.
    pub fn language(&self) -> &Option<String> {
        &self.language
    }

    /// Set the language for this code block.
    pub fn set_language(&mut self, language: &str) -> &mut Self {
        self.language = Some(language.to_string());
        self
    }

    /// Set the language for this code block to None.
    pub fn unset_language(&mut self) -> &mut Self {
        self.language = None;
        self
    }
}
