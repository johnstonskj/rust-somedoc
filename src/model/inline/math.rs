use crate::error;
use crate::model::block::BlockContent;
use crate::model::inline::InlineContent;
#[cfg(feature = "fmt_json")]
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Inline, LaTeX formatted representing a single formula.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "fmt_json", derive(Serialize, Deserialize))]
pub struct Math(String);

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

    fn from_str(inner: &str) -> Result<Self, Self::Err> {
        Ok(Self(inner.to_string()))
    }
}

impl Deref for Math {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<BlockContent> for Math {
    fn into(self) -> BlockContent {
        BlockContent::MathBlock(self.into())
    }
}

inline_impls!(Math);

inner_impl!(Math, String);

// ------------------------------------------------------------------------------------------------

#[cfg(feature = "math_builder")]
mod builder {
    use super::Math;
    use std::fmt::{Display, Formatter};

    #[derive(Clone, Debug)]
    pub(crate) struct MathBuilder(Vec<String>);

    impl Default for MathBuilder {
        fn default() -> Self {
            Self(Default::default())
        }
    }

    impl Display for MathBuilder {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0.join(""))
        }
    }

    impl Into<Math> for MathBuilder {
        fn into(self) -> Math {
            Math(self.to_string())
        }
    }

    impl MathBuilder {}
}
