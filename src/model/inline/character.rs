/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error;
use crate::model::inline::InlineContent;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Emoji(String);

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
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl FromStr for Emoji {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl Emoji {
    pub fn new(s: &str) -> error::Result<Self> {
        if s.chars().all(char::is_alphabetic) {
            Ok(Self(s.to_string()))
        } else {
            Err(error::ErrorKind::IllegalCharacter.into())
        }
    }

    pub fn name(&self) -> &String {
        &self.0
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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
