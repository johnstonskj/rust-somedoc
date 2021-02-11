use crate::model::block::Label;
use crate::model::inline::{InlineContent, Text};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The target types used by the `target` field of `HyperLink`.
///
#[derive(Clone, Debug, PartialEq)]
pub enum HyperLinkTarget {
    External(String),
    Internal(Label),
}

///
/// A link to another document, or an intra-document reference.
///
#[derive(Clone, Debug)]
pub struct HyperLink {
    target: HyperLinkTarget,
    caption: Option<Text>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

inline_impls!(HyperLink);

impl From<Label> for HyperLink {
    fn from(a: Label) -> Self {
        Self::internal(a)
    }
}

impl HyperLink {
    pub fn external(target: &str) -> Self {
        Self::new_external(target, None)
    }

    pub fn external_with_caption(target: &str, caption: Text) -> Self {
        Self::new_external(target, Some(caption))
    }

    pub fn external_with_caption_str(target: &str, caption: &str) -> Self {
        Self::new_external(target, Some(caption.into()))
    }

    pub fn internal(target: Label) -> Self {
        Self::new_internal(target, None)
    }

    pub fn internal_with_caption(target: Label, caption: Text) -> Self {
        Self::new_internal(target, Some(caption))
    }

    pub fn internal_with_caption_str(target: Label, caption: &str) -> Self {
        Self::new_internal(target, Some(caption.into()))
    }

    fn new_external(target: &str, caption: Option<Text>) -> Self {
        Self {
            target: HyperLinkTarget::External(target.to_string()),
            caption,
        }
    }

    fn new_internal(target: Label, caption: Option<Text>) -> Self {
        Self {
            target: HyperLinkTarget::Internal(target),
            caption,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_internal(&self) -> bool {
        match &self.target {
            HyperLinkTarget::Internal(_) => true,
            _ => false,
        }
    }

    pub fn is_external(&self) -> bool {
        match &self.target {
            HyperLinkTarget::External(_) => true,
            _ => false,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn target(&self) -> &HyperLinkTarget {
        &self.target
    }

    pub fn has_caption(&self) -> bool {
        self.caption.is_some()
    }

    pub fn caption(&self) -> &Option<Text> {
        &self.caption
    }
}
