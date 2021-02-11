use crate::error;
use crate::model::inline::InlineContent;
use regex::Regex;
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
pub struct Emoji(String);

///
/// A single character, including some special ones.
///
#[derive(Clone, Debug, PartialEq)]
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
        Self::new(s)
    }
}

impl Deref for Emoji {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Emoji {
    pub fn new(s: &str) -> error::Result<Self> {
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

    pub fn inner(&self) -> &String {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

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
