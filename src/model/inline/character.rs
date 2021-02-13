use crate::error;
use crate::model::inline::InlineContent;
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
/// The common name for an emoji.
///
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "fmt_json", derive(Serialize, Deserialize))]
pub struct Emoji(String);

///
/// A single character, including some special ones.
///
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "fmt_json", derive(Serialize, Deserialize))]
pub enum Character {
    Space,
    NonBreakSpace,
    Hyphen,
    EmDash,
    EnDash,
    Emoji(Emoji),
    Other(char),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref EMOJI_RE: Regex = Regex::new(r"(^:[a-zA-Z0-9_\-]+:$)|(^[a-zA-Z0-9_\-]+$)").unwrap();
}

impl Display for Emoji {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Emoji {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_empty() {
            if EMOJI_RE.is_match(s) {
                if s.starts_with(':') && s.ends_with(':') {
                    Ok(Self(s.to_string()))
                } else {
                    Ok(Self(format!(":{}:", s)))
                }
            } else {
                Err(error::ErrorKind::IllegalCharacter.into())
            }
        } else {
            Err(error::ErrorKind::MustNotBeEmpty.into())
        }
    }
}

impl Deref for Emoji {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

inner_impl!(Emoji, String);

// ------------------------------------------------------------------------------------------------

impl From<Emoji> for Character {
    fn from(v: Emoji) -> Self {
        Self::Emoji(v)
    }
}

impl From<char> for Character {
    fn from(v: char) -> Self {
        Self::Other(v)
    }
}

inline_impls!(Character);
