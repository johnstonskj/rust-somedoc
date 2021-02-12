use crate::model::block::caption::HasCaption;
use crate::model::block::{BlockContent, Caption};
use crate::model::block::{HasLabel, Label};
use crate::model::inline::{HyperLink, Image};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// An image block wraps an inline `Image` so that it forms a stand-alone block within the document.
///
#[derive(Clone, Debug)]
pub struct ImageBlock {
    label: Option<Label>,
    image: Image,
    caption: Option<Caption>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<HyperLink> for ImageBlock {
    fn from(target: HyperLink) -> Self {
        Self::from(Image::from(target))
    }
}

impl From<Image> for ImageBlock {
    fn from(image: Image) -> Self {
        Self {
            label: None,
            image,
            caption: None,
        }
    }
}

label_impl!(ImageBlock);

block_impls!(ImageBlock);

has_captioned_impls!(ImageBlock);

impl ImageBlock {
    /// Create a new image block from the provided `Image` instance and caption String.
    pub fn with_caption(image: Image, caption: Caption) -> Self {
        Self {
            label: None,
            image,
            caption: Some(caption),
        }
    }

    /// Create a new image block from the provided `Image` instance and caption string.
    pub fn with_caption_str(image: Image, caption: &str) -> Self {
        Self::with_caption(image, caption.into())
    }

    /// Return a reference to the inner image value.
    pub fn inner(&self) -> &Image {
        &self.image
    }
}
