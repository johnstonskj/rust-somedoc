use crate::model::block::{BlockContent, Caption, Captioned};
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

label_impl!(Formatted);

block_impls!(Formatted);

impl Formatted {
    pub fn new(inner: &str) -> Self {
        Self {
            label: None,
            inner: inner.to_string(),
        }
    }

    pub fn inner(&self) -> &String {
        &self.inner
    }

    pub fn into_inner(self) -> String {
        self.inner
    }
}

// ------------------------------------------------------------------------------------------------

label_impl!(CodeBlock);

block_impls!(CodeBlock);

has_captioned_impls!(CodeBlock);

impl CodeBlock {
    pub fn new(code: &str) -> Self {
        Self {
            label: None,
            code: code.to_string(),
            language: None,
            caption: None,
        }
    }

    pub fn new_with_language(code: &str, language: &str) -> Self {
        Self {
            label: None,
            code: code.to_string(),
            language: Some(language.to_string()),
            caption: None,
        }
    }

    pub fn code(&self) -> &String {
        &self.code
    }

    pub fn language(&self) -> &Option<String> {
        &self.language
    }
}
