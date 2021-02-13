use crate::model::block::caption::HasCaption;
use crate::model::block::{BlockContent, Caption};
use crate::model::block::{HasLabel, Label};
use crate::model::inline::Math;
#[cfg(feature = "fmt_json")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A math block wraps an inline `Math` so that it forms a stand-alone block within the document.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "fmt_json", derive(Serialize, Deserialize))]
pub struct MathBlock {
    #[cfg_attr(feature = "fmt_json", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "fmt_json", serde(default))]
    label: Option<Label>,
    math: Math,
    #[cfg_attr(feature = "fmt_json", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "fmt_json", serde(default))]
    caption: Option<Caption>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

label_impl!(MathBlock);

block_impls!(MathBlock);

has_captioned_impls!(MathBlock);

impl From<Math> for MathBlock {
    fn from(inner: Math) -> Self {
        Self {
            label: None,
            math: inner,
            caption: None,
        }
    }
}

impl MathBlock {
    /// Create a new instance from the inline `Math` instance and a `Caption`.
    pub fn with_caption(math: Math, caption: Caption) -> Self {
        Self {
            label: None,
            math,
            caption: Some(caption),
        }
    }

    /// Create a new instance from the inline `Math` instance and a caption string.
    pub fn with_caption_str(math: Math, caption: &str) -> Self {
        Self::with_caption(math, caption.into())
    }

    /// Return a reference to the inner `Math` instance.
    pub fn inner(&self) -> &Math {
        &self.math
    }
}
