use crate::error;
use crate::model::block::{BlockContent, HasBlockContent};
use crate::model::HasInnerContent;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Represents a block quote, note that these may be nested.
///
#[derive(Clone, Debug)]
pub struct Quote {
    content: Vec<BlockContent>,
}

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

has_block_impls!(Quote);
