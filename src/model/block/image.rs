use crate::model::block::caption::Captioned;
use crate::model::block::{BlockContent, Caption};
use crate::model::block::{HasLabel, Label};
use crate::model::inline::Image;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
///
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

label_impl!(ImageBlock);

block_impls!(ImageBlock);

has_captioned_impls!(ImageBlock);

impl ImageBlock {
    pub fn new(image: Image) -> Self {
        Self {
            label: None,
            image,
            caption: None,
        }
    }

    pub fn new_with_caption(image: Image, caption: Caption) -> Self {
        Self {
            label: None,
            image,
            caption: Some(caption),
        }
    }

    pub fn new_with_caption_str(image: Image, caption: &str) -> Self {
        Self::new_with_caption(image, caption.into())
    }

    pub fn inner(&self) -> &Image {
        &self.image
    }
}
