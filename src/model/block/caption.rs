use std::ops::Deref;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A `Caption` instance holds simple plain, un-styled, text, it is intended to be included with
/// blocks that are commonly labeled such as tables, images, etc.
///
#[derive(Clone, Debug)]
pub struct Caption(String);

pub trait Captioned {
    fn has_caption(&self) -> bool {
        self.caption().is_some()
    }

    fn caption(&self) -> &Option<Caption>;

    fn set_caption(&mut self, caption: Caption);

    fn unset_caption(&mut self);
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Caption {
    fn default() -> Self {
        Self(String::new())
    }
}

impl From<String> for Caption {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Caption {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Deref for Caption {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Caption {
    /// Return a reference to the inner string.
    pub fn inner(&self) -> &String {
        &self.0
    }

    /// Return the inner string.
    pub fn into_inner(self) -> String {
        self.0
    }
}
