use crate::model::inline::{Anchor, HasInlineContent, InlineContent, Span};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The target types used by the `target` field of `HyperLink`.
///
#[derive(Clone, Debug, PartialEq)]
pub enum HyperLinkTarget {
    External(String),
    Internal(Anchor),
}

///
/// A link to another document, or an intra-document reference.
///
#[derive(Clone, Debug)]
pub struct HyperLink {
    target: HyperLinkTarget,
    alt_text: Option<Span>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

inline_impls!(HyperLink);

impl From<Anchor> for HyperLink {
    fn from(a: Anchor) -> Self {
        Self::internal(a)
    }
}

impl HyperLink {
    pub fn external(target: &str) -> Self {
        Self::new_external(target, None)
    }

    pub fn external_with_label(target: &str, alt_text: Span) -> Self {
        Self::new_external(target, Some(alt_text))
    }

    pub fn external_with_label_str(target: &str, alt_text: &str) -> Self {
        Self::new_external(target, Some(Span::plain_str(alt_text)))
    }

    pub fn internal(target: Anchor) -> Self {
        Self::new_internal(target, None)
    }

    pub fn internal_with_label(target: Anchor, alt_text: Span) -> Self {
        Self::new_internal(target, Some(alt_text))
    }

    pub fn internal_with_label_str(target: Anchor, alt_text: &str) -> Self {
        Self::new_internal(target, Some(Span::plain_str(alt_text)))
    }

    fn new_external(target: &str, alt_text: Option<Span>) -> Self {
        Self {
            target: HyperLinkTarget::External(target.to_string()),
            alt_text,
        }
    }

    fn new_internal(target: Anchor, alt_text: Option<Span>) -> Self {
        Self {
            target: HyperLinkTarget::Internal(target),
            alt_text,
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

    pub fn has_alt_text(&self) -> bool {
        self.alt_text.is_some()
    }

    pub fn alt_text(&self) -> &Option<Span> {
        &self.alt_text
    }
}
