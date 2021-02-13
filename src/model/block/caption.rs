#[cfg(feature = "fmt_json")]
use serde::{Deserialize, Serialize};
use std::ops::Deref;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A `Caption` instance holds simple plain, un-styled, text, it is intended to be included with
/// blocks that are commonly labeled such as tables, images, etc.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "fmt_json", derive(Serialize, Deserialize))]
pub struct Caption(String);

///
/// Implemented by values that support a caption.
///
pub trait HasCaption {
    /// Returns `true` if a caption is set, else `false`.
    fn has_caption(&self) -> bool {
        self.caption().is_some()
    }

    /// Return the caption, if present.
    fn caption(&self) -> &Option<Caption>;

    /// Set the caption to the provided value.
    fn set_caption(&mut self, caption: Caption) -> &mut Self;

    /// Set the caption to `None`.
    fn unset_caption(&mut self) -> &mut Self;
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Caption {
    fn default() -> Self {
        Self(String::new())
    }
}

impl From<String> for Caption {
    fn from(inner: String) -> Self {
        Self::from(inner.as_str())
    }
}

impl From<&str> for Caption {
    fn from(inner: &str) -> Self {
        Self(inner.to_string())
    }
}

impl Deref for Caption {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

inner_impl!(Caption, String);
