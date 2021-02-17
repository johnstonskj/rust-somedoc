use crate::model::block::caption::HasCaption;
use crate::model::block::{BlockContent, Caption};
use crate::model::block::{HasLabel, Label};
use crate::model::inline::Image;
#[cfg(feature = "fmt_json")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// An image block wraps an inline `Image` so that it forms a stand-alone block within the document.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "fmt_json", derive(Serialize, Deserialize))]
pub struct ImageBlock {
    #[cfg_attr(feature = "fmt_json", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "fmt_json", serde(default))]
    label: Option<Label>,
    image: Image,
    #[cfg_attr(feature = "fmt_json", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "fmt_json", serde(default))]
    caption: Option<Caption>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<String> for ImageBlock {
    fn from(target: String) -> Self {
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
