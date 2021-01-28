use crate::error;
use std::ops::Deref;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Inline, LaTeX formatted representing a single formula.
///
#[derive(Clone, Debug)]
pub struct Math(String);

#[derive(Clone, Debug)]
pub struct MathBuilder;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Math {
    fn default() -> Self {
        Self(String::new())
    }
}

impl FromStr for Math {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl Deref for Math {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Math {
    ///
    /// Construct a new Math instance from the provided LaTeX string; this will return an error
    /// if the LaTeX is invalid.
    ///
    pub fn new(s: &str) -> error::Result<Self> {
        Ok(Self(s.to_string()))
    }

    pub fn inner(&self) -> &String {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}
