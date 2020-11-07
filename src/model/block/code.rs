/*!
One-line description.

More detailed description, with

# Example

*/

use crate::model::block::BlockContent;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Formatted(String);

#[derive(Clone, Debug)]
pub struct CodeBlock {
    code: String,
    language: Option<String>,
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

block_impls!(Formatted);

impl Formatted {
    pub fn new(text: &str) -> Self {
        Self(text.to_string())
    }

    pub fn inner(&self) -> &String {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

// ------------------------------------------------------------------------------------------------

block_impls!(CodeBlock);

impl CodeBlock {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_string(),
            language: None,
        }
    }

    pub fn new_with_language(code: &str, language: &str) -> Self {
        Self {
            code: code.to_string(),
            language: Some(language.to_string()),
        }
    }

    pub fn code(&self) -> &String {
        &self.code
    }

    pub fn language(&self) -> &Option<String> {
        &self.language
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
