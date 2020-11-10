use crate::error;
use crate::model::inline::{HyperLink, InlineContent};
use std::fmt::{Display, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// An `Anchor` represents a referencable location within a document.
///
#[derive(Clone, Debug, PartialEq)]
pub struct Anchor(String);

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Anchor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

inline_impls!(Anchor);

impl Anchor {
    /// Construct a new Anchor from the provided string value. This value **must** not be empty.
    pub fn new(value: &str) -> error::Result<Self> {
        if value.is_empty() {
            Err(error::ErrorKind::MustNotBeEmpty.into())
        } else {
            Ok(Self(value.to_string()))
        }
    }

    /// Create a new `HyperLink` value that refers to this `Anchor`.
    pub fn to_ref(&self) -> HyperLink {
        HyperLink::internal(self.clone())
    }

    /// Return a reference to the inner string.
    pub fn inner(&self) -> &String {
        &self.0
    }

    /// Return the inner string.
    pub fn into_inner(self) -> String {
        self.0
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
