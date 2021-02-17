/*!
This module is the root of a set of types that represent *inline* content; that is, content that
does not stand on it's own. In general the `Text` type is used to represent plain text, and the
`Span` type is used to represent a styled group of inline content.
*/

use crate::model::HasInnerContent;
#[cfg(feature = "fmt_json")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The set of supported inline content types.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "fmt_json", derive(Serialize, Deserialize))]
pub enum InlineContent {
    /// A link which may be to an internal document location or an external document.
    HyperLink(HyperLink),
    /// An image, referenced via a `HyperLink`.
    Image(Image),
    /// Plain text.
    Text(Text),
    /// Inline formatted math formula.
    Math(Math),
    /// A single character.
    Character(Character),
    /// A break within a set of inline content.
    LineBreak,
    /// A span contains other inline content and may also be styled.
    Span(Span),
}

// TODO: math <https://pandoc.org/MANUAL.html#math>

///
/// This trait is implemented by any type that contains, as content, a list of inline content instances.
/// Implementers of this trait may be themselves either block or inline content objects.
///
/// Note that the `add_` methods all return `&mut Self` and so calls to these may be chained.
///
pub trait HasInlineContent: Default + HasInnerContent<InlineContent> {
    /// Create a new inline content container from the provided `HyperLink` content item.
    fn link(inner: HyperLink) -> Self {
        let mut new_self = Self::default();
        let _ = new_self.add_link(inner);
        new_self
    }

    /// Create a new inline content container from the provided `Image` content item.
    fn image(inner: Image) -> Self {
        let mut new_self = Self::default();
        let _ = new_self.add_image(inner);
        new_self
    }

    /// Create a new inline content container from the provided `Text` content item.
    fn text(inner: Text) -> Self {
        let mut new_self = Self::default();
        let _ = new_self.add_text(inner);
        new_self
    }

    /// Create a new inline content container from the provided string (into `Text`) content item.
    fn text_str(inner: &str) -> Self {
        let mut new_self = Self::default();
        let _ = new_self.add_text(inner.into());
        new_self
    }

    /// Create a new inline content container from the provided `Character` content item.
    fn character(inner: Character) -> Self {
        let mut new_self = Self::default();
        let _ = new_self.add_character(inner);
        new_self
    }

    /// Create a new inline content container with a line break.
    fn line_break() -> Self {
        let mut new_self = Self::default();
        let _ = new_self.add_line_break();
        new_self
    }

    /// Create a new inline content container from the provided `Span` content item.
    fn span(span: Span) -> Self {
        let mut new_self = Self::default();
        let _ = new_self.add_span(span);
        new_self
    }

    // --------------------------------------------------------------------------------------------

    /// Add the provided `HyperLink` to this container's inner content.
    fn add_link(&mut self, inner: HyperLink) -> &mut Self {
        let _ = self.add_content(inner.into()).unwrap();
        self
    }

    /// Add the provided `Image` to this container's inner content.
    fn add_image(&mut self, inner: Image) -> &mut Self {
        let _ = self.add_content(inner.into()).unwrap();
        self
    }

    /// Add the provided `Text` to this container's inner content.
    fn add_text(&mut self, inner: Text) -> &mut Self {
        let _ = self.add_content(inner.into()).unwrap();
        self
    }

    /// Add the provided string (as `Text`) to this container's inner content.
    fn add_text_str(&mut self, inner: &str) -> &mut Self {
        let t: Text = inner.into();
        self.add_content(t.into()).unwrap();
        self
    }

    /// Add the provided `Character` to this container's inner content.
    fn add_character(&mut self, inner: Character) -> &mut Self {
        self.add_content(inner.into()).unwrap();
        self
    }

    /// Add a single space to this container's inner content.
    fn add_space(&mut self) -> &mut Self {
        self.add_content(Character::Space.into()).unwrap();
        self
    }

    /// Add a single non-breaking space to this container's inner content.
    fn add_non_breaking_space(&mut self) -> &mut Self {
        self.add_content(Character::NonBreakSpace.into()).unwrap();
        self
    }

    /// Add a `LineBreak` to this container's inner content.
    fn add_line_break(&mut self) -> &mut Self {
        self.add_content(InlineContent::LineBreak).unwrap();
        self
    }

    /// Add the provided `Span` to this container's inner content.
    fn add_span(&mut self, inner: Span) -> &mut Self {
        self.add_content(InlineContent::Span(inner)).unwrap();
        self
    }

    // --------------------------------------------------------------------------------------------

    /// Create a new inline content container as a `Span`, with `TextStyle::Plain` applied to the
    /// provided content.
    fn plain(inner: InlineContent) -> Self {
        Self::span(Span::inner_with_style(inner, SpanStyle::Plain))
    }

    /// Create a new inline content container as a `Span`, with `TextStyle::Plain` applied to the
    /// provided string as a `Text` instance.
    fn plain_str(inner: &str) -> Self {
        Self::span(Span::with_style(inner, SpanStyle::Plain))
    }

    /// Create a new inline content container as a `Span`, with `TextStyle::Italic` applied to the
    /// provided content.
    fn italic(inner: InlineContent) -> Self {
        Self::span(Span::inner_with_style(inner, SpanStyle::Italic))
    }

    /// Create a new inline content container as a `Span`, with `TextStyle::Italic` applied to the
    /// provided string as a `Text` instance.
    fn italic_str(inner: &str) -> Self {
        Self::span(Span::with_style(inner, SpanStyle::Italic))
    }

    /// Create a new inline content container as a `Span`, with `TextStyle::Bold` applied to the
    /// provided content.
    fn bold(inner: InlineContent) -> Self {
        Self::span(Span::inner_with_style(inner, SpanStyle::Bold))
    }

    /// Create a new inline content container as a `Span`, with `TextStyle::Bold` applied to the
    /// provided string as a `Text` instance.
    fn bold_str(inner: &str) -> Self {
        Self::span(Span::with_style(inner, SpanStyle::Bold))
    }

    /// Create a new inline content container as a `Span`, with `TextStyle::Mono` applied to the
    /// provided content.
    fn mono(inner: InlineContent) -> Self {
        Self::span(Span::inner_with_style(inner, SpanStyle::Mono))
    }

    /// Create a new inline content container as a `Span`, with `TextStyle::Mono` applied to the
    /// provided string as a `Text` instance.
    fn mono_str(inner: &str) -> Self {
        Self::span(Span::with_style(inner, SpanStyle::Mono))
    }

    /// Create a new inline content container as a `Span`, with `TextStyle::Code` applied to the
    /// provided content.
    fn code(inner: InlineContent) -> Self {
        Self::span(Span::inner_with_style(inner, SpanStyle::Code))
    }

    /// Create a new inline content container as a `Span`, with `TextStyle::Code` applied to the
    /// provided string as a `Text` instance.
    fn code_str(inner: &str) -> Self {
        Self::span(Span::with_style(inner, SpanStyle::Code))
    }

    /// Create a new inline content container as a `Span`, with `TextStyle::Strikethrough` applied to the
    /// provided content.
    fn strikethrough(inner: InlineContent) -> Self {
        Self::span(Span::inner_with_style(inner, SpanStyle::Strikethrough))
    }

    /// Create a new inline content container as a `Span`, with `TextStyle::Strikethrough` applied to the
    /// provided string as a `Text` instance.
    fn strikethrough_str(inner: &str) -> Self {
        Self::span(Span::with_style(inner, SpanStyle::Strikethrough))
    }

    /// Create a new inline content container as a `Span`, with `TextStyle::Underline` applied to the
    /// provided content.
    fn underline(inner: InlineContent) -> Self {
        Self::span(Span::inner_with_style(inner, SpanStyle::Underline))
    }

    /// Create a new inline content container as a `Span`, with `TextStyle::Underline` applied to the
    /// provided string as a `Text` instance.
    fn underline_str(inner: &str) -> Self {
        Self::span(Span::with_style(inner, SpanStyle::Underline))
    }

    /// Create a new inline content container as a `Span`, with `TextStyle::SmallCaps` applied to the
    /// provided content.
    fn small_caps(inner: InlineContent) -> Self {
        Self::span(Span::inner_with_style(inner, SpanStyle::SmallCaps))
    }

    /// Create a new inline content container as a `Span`, with `TextStyle::SmallCaps` applied to the
    /// provided string as a `Text` instance.
    fn small_caps_str(inner: &str) -> Self {
        Self::span(Span::with_style(inner, SpanStyle::SmallCaps))
    }

    /// Create a new inline content container as a `Span`, with `TextStyle::Superscript` applied to the
    /// provided content.
    fn superscript(inner: InlineContent) -> Self {
        Self::span(Span::inner_with_style(inner, SpanStyle::Superscript))
    }

    /// Create a new inline content container as a `Span`, with `TextStyle::Superscript` applied to the
    /// provided string as a `Text` instance.
    fn superscript_str(inner: &str) -> Self {
        Self::span(Span::with_style(inner, SpanStyle::Superscript))
    }

    /// Create a new inline content container as a `Span`, with `TextStyle::Subscript` applied to the
    /// provided content.
    fn subscript(inner: InlineContent) -> Self {
        Self::span(Span::inner_with_style(inner, SpanStyle::Subscript))
    }

    /// Create a new inline content container as a `Span`, with `TextStyle::Subscript` applied to the
    /// provided string as a `Text` instance.
    fn subscript_str(inner: &str) -> Self {
        Self::span(Span::with_style(inner, SpanStyle::Subscript))
    }

    // --------------------------------------------------------------------------------------------

    /// Return a string that is simply the `Text` instances (recursively) concatenated. This removes
    /// any style information and non-text instances such as images or links.
    fn unformatted_string(&self) -> String {
        let mut s = String::new();
        let _ = unformat(&mut s, self.inner());
        s
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn unformat(s: &mut String, content: &[InlineContent]) -> String {
    for item in content {
        match item {
            // TODO: all local refs need alt text.
            InlineContent::Text(value) => s.push_str(value.inner()),
            InlineContent::Character(value) => match value {
                Character::Space => s.push(' '),
                Character::NonBreakSpace => s.push(' '),
                Character::Hyphen => s.push('-'),
                Character::EmDash => s.push_str("---"),
                Character::EnDash => s.push_str("--"),
                Character::Emoji(e) => s.push_str(e.inner()),
                Character::Other(c) => s.push(*c),
            },
            InlineContent::LineBreak => s.push('\n'),
            InlineContent::Span(value) => {
                let s2 = unformat(s, value.inner());
                s.push_str(&s2)
            }
            _ => {}
        }
    }
    s.clone()
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod character;
pub use character::{Character, Emoji};

#[cfg(feature = "emoji_names")]
#[allow(missing_docs)]
pub mod emoji_names;

#[doc(hidden)]
pub mod image;
pub use image::Image;

#[doc(hidden)]
pub mod link;
pub use link::{HyperLink, HyperLinkTarget};

#[doc(hidden)]
pub mod math;
pub use math::Math;

#[doc(hidden)]
pub mod text;
pub use text::{Span, SpanStyle, Text};
