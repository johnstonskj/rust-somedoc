use crate::error;
use crate::model::block::{BlockContent, HasBlockContent, Label};
use crate::model::{block::HasLabel, HasInnerContent};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Represents a block quote, note that these may be nested.
///
#[derive(Clone, Debug)]
pub struct Quote {
    label: Option<Label>,
    content: Vec<BlockContent>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Quote {
    fn default() -> Self {
        Self {
            label: None,
            content: Default::default(),
        }
    }
}

impl From<BlockContent> for Quote {
    fn from(v: BlockContent) -> Self {
        Self {
            label: None,
            content: vec![v],
        }
    }
}

label_impl!(Quote);

block_impls!(Quote);

has_block_impls!(Quote);
