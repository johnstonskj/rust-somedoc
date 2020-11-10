use crate::model::block::BlockContent;
use crate::model::inline::{HyperLink, InlineContent};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// An image, identified by the link content.
///
#[derive(Clone, Debug)]
pub struct Image(HyperLink);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<HyperLink> for Image {
    fn from(v: HyperLink) -> Self {
        Self::new(v)
    }
}

inline_impls!(Image);

impl Into<BlockContent> for Image {
    fn into(self) -> BlockContent {
        BlockContent::Image(self)
    }
}

impl Image {
    pub fn new(v: HyperLink) -> Self {
        Self(v)
    }

    pub fn link(&self) -> &HyperLink {
        &self.0
    }

    pub fn into_inner(self) -> HyperLink {
        self.0
    }
}
