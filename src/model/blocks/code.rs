/*!
One-line description.

More detailed description, with

# Example

*/

// use ...

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

use crate::model::blocks::BlockContent;

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

block_impls!(CodeBlock);

impl CodeBlock {
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
