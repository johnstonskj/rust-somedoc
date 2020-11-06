/*!
One-line description.

More detailed description, with

# Example

*/

use crate::model::inline::anchor::Anchor;
use crate::model::inline::InlineContent;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum HyperLinkTarget {
    External(String),
    Internal(Anchor),
}

#[derive(Clone, Debug)]
pub struct HyperLink {
    target: HyperLinkTarget,
    alt_text: Option<String>,
    title: Option<String>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

inline_impls!(HyperLink);

impl From<String> for HyperLink {
    fn from(s: String) -> Self {
        Self::external(&s)
    }
}

impl From<&str> for HyperLink {
    fn from(s: &str) -> Self {
        Self::external(s)
    }
}

impl From<Anchor> for HyperLink {
    fn from(a: Anchor) -> Self {
        Self::internal(a)
    }
}

impl HyperLink {
    pub fn external(target: &str) -> Self {
        Self::new_external(target, None, None)
    }

    pub fn external_with_label(target: &str, alt_text: &str) -> Self {
        Self::new_external(target, Some(alt_text), None)
    }

    pub fn external_with_label_and_title(target: &str, alt_text: &str, title: &str) -> Self {
        Self::new_external(target, Some(alt_text), Some(title))
    }

    pub fn internal(target: Anchor) -> Self {
        Self::new_internal(target, None, None)
    }

    pub fn internal_with_label(target: Anchor, alt_text: &str) -> Self {
        Self::new_internal(target, Some(alt_text), None)
    }

    pub fn internal_with_label_and_title(target: Anchor, alt_text: &str, title: &str) -> Self {
        Self::new_internal(target, Some(alt_text), Some(title))
    }

    fn new_external(target: &str, alt_text: Option<&str>, title: Option<&str>) -> Self {
        Self {
            target: HyperLinkTarget::External(target.to_string()),
            alt_text: alt_text.map(str::to_string),
            title: title.map(str::to_string),
        }
    }

    fn new_internal(target: Anchor, alt_text: Option<&str>, title: Option<&str>) -> Self {
        Self {
            target: HyperLinkTarget::Internal(target),
            alt_text: alt_text.map(str::to_string),
            title: title.map(str::to_string),
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

    pub fn target(&self) -> &HyperLinkTarget {
        &self.target
    }

    pub fn has_alt_text(&self) -> bool {
        self.alt_text.is_some()
    }

    pub fn alt_text(&self) -> &Option<String> {
        &self.alt_text
    }

    pub fn has_title(&self) -> bool {
        self.title.is_some()
    }

    pub fn title(&self) -> &Option<String> {
        &self.title
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
