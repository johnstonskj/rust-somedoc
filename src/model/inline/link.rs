use crate::model::block::{Caption, HasCaption, Label};
use crate::model::inline::InlineContent;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The target types used by the `target` field of `HyperLink`.
///
#[derive(Clone, Debug, PartialEq)]
pub enum HyperLinkTarget {
    /// The target is an external reference, i.e. URL.
    External(String),
    /// The target is an internal reference, a `Label` on some element.
    Internal(Label),
}

///
/// A link to another document, or an intra-document reference.
///
#[derive(Clone, Debug)]
pub struct HyperLink {
    target: HyperLinkTarget,
    caption: Option<Caption>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<Label> for HyperLink {
    fn from(v: Label) -> Self {
        Self::internal(v)
    }
}

impl From<HyperLinkTarget> for HyperLink {
    fn from(v: HyperLinkTarget) -> Self {
        Self {
            target: v,
            caption: None,
        }
    }
}

inline_impls!(HyperLink);

has_captioned_impls!(HyperLink);

impl HyperLink {
    /// Create a new `Link` with `target` an external location.
    pub fn external(target: &str) -> Self {
        Self::new_external(target, None)
    }

    /// Create a new `Link` with `target` an external location, and associated caption.
    pub fn external_with_caption(target: &str, caption: Caption) -> Self {
        Self::new_external(target, Some(caption))
    }

    /// Create a new `Link` with `target` an external location, and associated caption string.
    pub fn external_with_caption_str(target: &str, caption: &str) -> Self {
        Self::new_external(target, Some(caption.into()))
    }

    /// Create a new `Link` with `target` an internal location.
    pub fn internal(target: Label) -> Self {
        Self::new_internal(target, None)
    }

    /// Create a new `Link` with `target` an internal location, and associated caption.
    pub fn internal_with_caption(target: Label, caption: Caption) -> Self {
        Self::new_internal(target, Some(caption))
    }

    /// Create a new `Link` with `target` an internal location, and associated caption string.
    pub fn internal_with_caption_str(target: Label, caption: &str) -> Self {
        Self::new_internal(target, Some(caption.into()))
    }

    fn new_external(target: &str, caption: Option<Caption>) -> Self {
        Self {
            target: HyperLinkTarget::External(target.to_string()),
            caption,
        }
    }

    fn new_internal(target: Label, caption: Option<Caption>) -> Self {
        Self {
            target: HyperLinkTarget::Internal(target),
            caption,
        }
    }

    // --------------------------------------------------------------------------------------------

    /// Is the target of this link internal?
    pub fn is_internal(&self) -> bool {
        matches!(&self.target, HyperLinkTarget::Internal(_))
    }

    /// Is the target of this link external?
    pub fn is_external(&self) -> bool {
        matches!(&self.target, HyperLinkTarget::External(_))
    }

    // --------------------------------------------------------------------------------------------

    /// Return the target of this link.
    pub fn target(&self) -> &HyperLinkTarget {
        &self.target
    }
}
