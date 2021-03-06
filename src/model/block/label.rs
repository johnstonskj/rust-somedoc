use crate::model::inline::HasInlineContent;
use regex::Regex;
#[cfg(feature = "fmt_json")]
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A `Label` represents a *referencable mark* in a document. A `HyperLink` may reference a
/// `Label` and in some cases external documents may reference a label within documents. In HTML
/// these become IDs/anchors, in LaTeX they are `label` values.
///
/// # Value Space
///
/// Labels must conform to the following rules:
///
/// 1. **may not** be empty,
/// 1. **must** start with a Unicode letter character,
/// 1. **may** then contain any number of Unicode letters, numbers, or the characters underscore `_`,
///    hyphen `-`, period `.`, or colon `:`.
///
/// It is common in LaTeX to prefix labels with a type, for example `"fig:"` for figures, or
/// `"lst:"` for listings. The methods `generate` and `safe_from` allow construction with optional
/// prefixes.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "fmt_json", derive(Serialize, Deserialize))]
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
    fn set_label(&mut self, label: Label) -> &mut Self;

    ///
    /// Set the current element's label to `None`.
    ///
    fn unset_label(&mut self) -> &mut Self;
}

///
/// Trait for model elements that can auto-generate a label for themselves.
///
pub trait AutoLabel: HasLabel {
    ///
    /// Set the current element's label value based on other properties already set in the
    /// model element. For example, if a value has been set for a Heading text then the auto
    /// label will be based on it.
    ///
    /// Note, if no values are present to make the label from, **or** this method is not supported
    /// by an element, no label is generated.
    ///
    #[allow(unused_variables)]
    fn auto_label(&mut self) -> &mut Self;
}

impl<T> AutoLabel for T
where
    T: HasInlineContent + HasLabel,
{
    fn auto_label(&mut self) -> &mut Self {
        self.set_label(Label::safe_from(&self.unformatted_string(), None))
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref RE_LABEL: Regex = Regex::new(r"^\p{L}+[\p{L}\p{N}_\-\.:]*$").unwrap();
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Label {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
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

inner_impl!(Label, String);

impl Label {
    /// Returns `true` if `label` is a valid label value, else `false`.
    pub fn is_valid(label: &str) -> bool {
        RE_LABEL.is_match(label)
    }

    /// Generate a new, random, identifier. The `prefix` if present **must** be a non-empty string
    /// containing **only** Unicode characters. If not specified the prefix will be `"gen"`. This
    /// function will place the colon character as a separator between prefix and label text.
    ///
    /// This function will panic if the prefix is empty, or contains non-Unicode letter characters.
    pub fn generate(prefix: Option<&str>) -> Self {
        let prefix = if let Some(prefix) = prefix {
            assert!(!prefix.is_empty());
            assert!(prefix.chars().all(char::is_alphabetic));
            format!("{}:", prefix)
        } else {
            String::from("gen:")
        };
        Self(format!("{}{}", prefix, blob_uuid::random_blob()))
    }

    /// Generate a new label by replacing any illegal characters in `label` with the underscore `_`
    /// character. This cannot be guaranteed to generate unique labels as both `"hello world"` and
    /// `"hello?world"` will both result in `"hello_world"`. This function will place the colon
    /// character as a separator between prefix and label text if a prefix value is provided.
    ///
    /// This function will panic if the label is empty, or the first character **is not** a Unicode
    /// letter (as the replacement underscore is not a valid start character for labels). Also, if
    /// the prefix has been provided and is empty, or contains non-Unicode letter characters.
    pub fn safe_from(label: &str, prefix: Option<&str>) -> Self {
        assert!(!label.is_empty());
        let mut characters = label.chars();
        let first = characters.next().unwrap();
        assert!(first.is_alphabetic());
        let rest: String = characters
            .map(|c| {
                if c.is_alphanumeric() || c == '_' || c == '-' || c == '.' || c == ':' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        let prefix = if let Some(prefix) = prefix {
            assert!(prefix.chars().all(char::is_alphabetic));
            format!("{}:", prefix)
        } else {
            String::new()
        };
        Self(format!("{}{}{}", prefix, first, rest))
    }

    /// Copy the label value (or `None`) from a labeled element. This allows a reference label, or
    /// link to be created without having to copy the text of the label value. This is even more
    /// useful when used with the `AutoLabel` trait, as shown below.
    ///
    /// # Example
    ///
    /// ```rust
    /// use somedoc::model::block::{AutoLabel, HasLabel, Heading, Label};
    /// use somedoc::model::inline::HyperLink;
    ///
    /// let header = Heading::section("Section One").auto_label().clone();
    ///
    /// let ref_to_section = HyperLink::from(Label::copy_from(&header).unwrap());
    /// ```
    ///
    pub fn copy_from(labeled: &impl HasLabel) -> Option<Self> {
        match labeled.label() {
            None => None,
            Some(label) => Some(Self(label.inner().clone())),
        }
    }
}
