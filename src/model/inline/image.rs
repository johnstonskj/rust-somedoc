use crate::model::block::BlockContent;
use crate::model::inline::InlineContent;
#[cfg(feature = "fmt_json")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// An image, identified by the linked content.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "fmt_json", derive(Serialize, Deserialize))]
pub struct Image {
    path_or_label: String,
    alt_text: Option<String>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<String> for Image {
    fn from(inner: String) -> Self {
        Self::new(&inner)
    }
}

impl From<&str> for Image {
    fn from(inner: &str) -> Self {
        Self::new(inner)
    }
}

impl Into<BlockContent> for Image {
    fn into(self) -> BlockContent {
        BlockContent::ImageBlock(self.into())
    }
}

inline_impls!(Image);

inner_impl!(Image, String, path_or_label);

impl Image {
    /// Construct a new image from the provided path (URL) or label (internal anchor).
    ///
    /// This will panic if `path_or_label` is empty.
    pub fn new(path_or_label: &str) -> Self {
        assert!(!path_or_label.is_empty());
        Self {
            path_or_label: path_or_label.to_string(),
            alt_text: None,
        }
    }

    /// Construct a new image from the provided path (URL) or label (internal anchor) with an
    /// additional `alt_text`.
    ///
    /// This will panic if either `path_or_label` or `alt_text` is empty.
    pub fn with_alt_text(path_or_label: &str, alt_text: &str) -> Self {
        assert!(!path_or_label.is_empty());
        assert!(!alt_text.is_empty());
        Self {
            path_or_label: path_or_label.to_string(),
            alt_text: Some(alt_text.to_string()),
        }
    }

    /// Returns `true` if this Image has an alternative text value, else `false`.
    pub fn has_alt_text(&self) -> bool {
        self.alt_text.is_some()
    }

    /// Returns the current alternative text value.
    pub fn alt_text(&self) -> &Option<String> {
        &self.alt_text
    }

    /// Sets the current alternative text value.
    pub fn set_alt_text(&mut self, alt_text: &str) {
        self.alt_text = Some(alt_text.to_string())
    }

    /// Sets the current alternative text value to `None`.
    pub fn unset_alt_text(&mut self) {
        self.alt_text = None
    }
}
