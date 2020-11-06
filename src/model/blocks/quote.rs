/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error;
use crate::model::blocks::{BlockContent, HasBlockContent};
use crate::model::ComplexContent;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Quote {
    content: Vec<BlockContent>,
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

impl Default for Quote {
    fn default() -> Self {
        Self {
            content: Default::default(),
        }
    }
}

impl From<BlockContent> for Quote {
    fn from(v: BlockContent) -> Self {
        Self { content: vec![v] }
    }
}

block_impls!(Quote);

impl ComplexContent<BlockContent> for Quote {
    fn inner(&self) -> &Vec<BlockContent> {
        &self.content
    }

    fn inner_mut(&mut self) -> &mut Vec<BlockContent> {
        &mut self.content
    }

    fn add_content(&mut self, content: BlockContent) -> error::Result<()> {
        self.content.push(content);
        Ok(())
    }
}

impl HasBlockContent for Quote {}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
