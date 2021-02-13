use crate::model::block::BlockContent;
use crate::model::inline::{HyperLink, InlineContent};
#[cfg(feature = "fmt_json")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// An image, identified by the link content.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "fmt_json", derive(Serialize, Deserialize))]
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
