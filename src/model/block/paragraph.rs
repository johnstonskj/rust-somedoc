use crate::error;
use crate::model::block::{Alignment, BlockContent, HasAlignment, Label};
use crate::model::inline::{HasInlineContent, InlineContent, Span};
use crate::model::{block::HasLabel, HasInnerContent};
#[cfg(feature = "fmt_json")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A paragraph is a bounded block of inline content, usually text.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "fmt_json", derive(Serialize, Deserialize))]
pub struct Paragraph {
    #[cfg_attr(feature = "fmt_json", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "fmt_json", serde(default))]
    label: Option<Label>,
    inner: Vec<InlineContent>,
    alignment: Alignment,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Paragraph {
    fn default() -> Self {
        Self {
            label: None,
            inner: Default::default(),
            alignment: Default::default(),
        }
    }
}

label_impl!(Paragraph);

alignment_impl!(Paragraph);

block_impls!(Paragraph);

has_inline_impls!(Paragraph);

impl Paragraph {
    /// Create a new instance with the string as inline content and the provided alignment.
    pub fn with_alignment(inner: &str, alignment: Alignment) -> Self {
        Self {
            label: None,
            inner: vec![Span::plain_str(inner).into()],
            alignment,
        }
    }
}
