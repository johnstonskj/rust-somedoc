use crate::model::block::caption::Captioned;
use crate::model::block::{BlockContent, Caption};
use crate::model::inline::Math;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// .
///
#[derive(Clone, Debug)]
pub struct MathBlock {
    math: Math,
    caption: Option<Caption>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

block_impls!(MathBlock);

has_captioned_impls!(MathBlock);

impl MathBlock {
    pub fn new(math: Math) -> Self {
        Self {
            math,
            caption: None,
        }
    }

    pub fn new_with_caption(math: Math, caption: Caption) -> Self {
        Self {
            math,
            caption: Some(caption),
        }
    }

    pub fn new_with_caption_str(math: Math, caption: &str) -> Self {
        Self::new_with_caption(math, caption.into())
    }

    pub fn inner(&self) -> &Math {
        &self.math
    }
}
