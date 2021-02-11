/*!
One-line description.

More detailed description, with

# Example

*/

use regex::Regex;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Label(String);

///
/// This trait is used to attach a *label* to any element in the model. In HTML terms these are
/// anchors, in markdown these may or may not be supported, in LaTeX they are represented using
/// `\label{}`.
///
pub trait HasLabel {
    ///
    /// Returns `true` if this element has a label, else `false`.
    ///
    fn has_label(&self) -> bool;

    ///
    /// Return the element's label, if present.
    ///
    fn label(&self) -> &Option<Label>;

    ///
    /// Set the current element's label value.
    ///
    fn set_label(&mut self, label: Label);

    ///
    /// Set the current element's label to `None`.
    ///
    fn unset_label(&mut self);
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

lazy_static! {
    static ref RE_LABEL: Regex = Regex::new(r"\p{L}+[\p{L}\p{N}_\-\.]*").unwrap();
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Label {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if RE_LABEL.is_match(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(())
        }
    }
}

impl Deref for Label {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Label {
    pub fn inner(&self) -> &String {
        &self.0
    }

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
