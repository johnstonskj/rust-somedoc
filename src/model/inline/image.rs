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
    fn from(link: HyperLink) -> Self {
        Self(link)
    }
}

impl Into<BlockContent> for Image {
    fn into(self) -> BlockContent {
        BlockContent::ImageBlock(self.into())
    }
}

inline_impls!(Image);

inner_impl!(Image, HyperLink);
