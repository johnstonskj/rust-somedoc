/*!
One-line description.

More detailed description, with

# Example

*/

use crate::model::ComplexContent;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum InlineContent {
    Anchor(Anchor),
    HyperLink(HyperLink),
    Image(Image),
    Text(Text),
    Character(Character),
    LineBreak,
    Span(Span),
}

pub trait HasInlineContent: Default + ComplexContent<InlineContent> {
    fn from(inner: InlineContent) -> Self {
        let mut new_self = Self::default();
        new_self.add_content(inner).unwrap();
        new_self
    }

    fn from_all(inner: Vec<InlineContent>) -> Self {
        let mut new_self = Self::default();
        let mut inner = inner;
        for item in inner.drain(..) {
            new_self.add_content(item).unwrap();
        }
        new_self
    }

    fn anchor(inner: Anchor) -> Self {
        let mut new_self = Self::default();
        new_self.add_anchor(inner);
        new_self
    }

    fn link(inner: HyperLink) -> Self {
        let mut new_self = Self::default();
        new_self.add_link(inner);
        new_self
    }

    fn image(inner: Image) -> Self {
        let mut new_self = Self::default();
        new_self.add_image(inner);
        new_self
    }

    fn text(inner: Text) -> Self {
        let mut new_self = Self::default();
        new_self.add_text(inner);
        new_self
    }

    fn text_str(inner: &str) -> Self {
        let mut new_self = Self::default();
        new_self.add_text(inner.into());
        new_self
    }

    fn character(inner: Character) -> Self {
        let mut new_self = Self::default();
        new_self.add_character(inner);
        new_self
    }

    fn line_break() -> Self {
        let mut new_self = Self::default();
        new_self.add_line_break();
        new_self
    }

    fn span(span: Span) -> Self {
        let mut new_self = Self::default();
        new_self.add_span(span);
        new_self
    }

    // --------------------------------------------------------------------------------------------

    fn add_anchor(&mut self, inner: Anchor) {
        self.add_content(inner.into()).unwrap()
    }

    fn add_link(&mut self, inner: HyperLink) {
        self.add_content(inner.into()).unwrap()
    }

    fn add_image(&mut self, inner: Image) {
        self.add_content(inner.into()).unwrap()
    }

    fn add_text(&mut self, inner: Text) {
        self.add_content(inner.into()).unwrap()
    }

    fn add_text_str(&mut self, inner: &str) {
        let t: Text = inner.into();
        self.add_content(t.into()).unwrap()
    }

    fn add_character(&mut self, inner: Character) {
        self.add_content(inner.into()).unwrap()
    }

    fn add_line_break(&mut self) {
        self.add_content(InlineContent::LineBreak).unwrap()
    }

    fn add_span(&mut self, inner: Span) {
        self.add_content(InlineContent::Span(inner.into())).unwrap()
    }

    // --------------------------------------------------------------------------------------------

    fn plain(inner: InlineContent) -> Self {
        Self::span(Span::new_inner_with_style(inner, TextStyle::Plain))
    }

    fn plain_str(inner: &str) -> Self {
        Self::span(Span::new_with_style(inner, TextStyle::Plain))
    }

    fn italic(inner: InlineContent) -> Self {
        Self::span(Span::new_inner_with_style(inner, TextStyle::Italic))
    }

    fn italic_str(inner: &str) -> Self {
        Self::span(Span::new_with_style(inner, TextStyle::Italic))
    }

    fn slanted(inner: InlineContent) -> Self {
        Self::span(Span::new_inner_with_style(inner, TextStyle::Slanted))
    }

    fn slanted_str(inner: &str) -> Self {
        Self::span(Span::new_with_style(inner, TextStyle::Slanted))
    }

    fn quote(inner: InlineContent) -> Self {
        Self::span(Span::new_inner_with_style(inner, TextStyle::Quote))
    }

    fn quote_str(inner: &str) -> Self {
        Self::span(Span::new_with_style(inner, TextStyle::Quote))
    }

    fn light(inner: InlineContent) -> Self {
        Self::span(Span::new_inner_with_style(inner, TextStyle::Light))
    }

    fn light_str(inner: &str) -> Self {
        Self::span(Span::new_with_style(inner, TextStyle::Light))
    }

    fn bold(inner: InlineContent) -> Self {
        Self::span(Span::new_inner_with_style(inner, TextStyle::Bold))
    }

    fn bold_str(inner: &str) -> Self {
        Self::span(Span::new_with_style(inner, TextStyle::Bold))
    }

    fn mono(inner: InlineContent) -> Self {
        Self::span(Span::new_inner_with_style(inner, TextStyle::Mono))
    }

    fn mono_str(inner: &str) -> Self {
        Self::span(Span::new_with_style(inner, TextStyle::Mono))
    }

    fn code(inner: InlineContent) -> Self {
        Self::span(Span::new_inner_with_style(inner, TextStyle::Code))
    }

    fn code_str(inner: &str) -> Self {
        Self::span(Span::new_with_style(inner, TextStyle::Code))
    }

    fn strikethrough(inner: InlineContent) -> Self {
        Self::span(Span::new_inner_with_style(inner, TextStyle::Strikethrough))
    }

    fn strikethrough_str(inner: &str) -> Self {
        Self::span(Span::new_with_style(inner, TextStyle::Strikethrough))
    }

    fn underline(inner: InlineContent) -> Self {
        Self::span(Span::new_inner_with_style(inner, TextStyle::Underline))
    }

    fn underline_str(inner: &str) -> Self {
        Self::span(Span::new_with_style(inner, TextStyle::Underline))
    }

    fn small_caps(inner: InlineContent) -> Self {
        Self::span(Span::new_inner_with_style(inner, TextStyle::SmallCaps))
    }

    fn small_caps_str(inner: &str) -> Self {
        Self::span(Span::new_with_style(inner, TextStyle::SmallCaps))
    }

    fn superscript(inner: InlineContent) -> Self {
        Self::span(Span::new_inner_with_style(inner, TextStyle::Superscript))
    }

    fn superscript_str(inner: &str) -> Self {
        Self::span(Span::new_with_style(inner, TextStyle::Superscript))
    }

    fn subscript(inner: InlineContent) -> Self {
        Self::span(Span::new_inner_with_style(inner, TextStyle::Subscript))
    }

    fn subscript_str(inner: &str) -> Self {
        Self::span(Span::new_with_style(inner, TextStyle::Subscript))
    }

    // --------------------------------------------------------------------------------------------

    fn unformatted_string(&self) -> String {
        let mut s = String::new();
        unformat(&mut s, self.inner());
        s
    }
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn unformat(s: &mut String, content: &Vec<InlineContent>) -> String {
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
                Character::Emoji(e) => s.push_str(e.name()),
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

pub mod character;
pub use character::Character;

pub mod anchor;
pub use anchor::Anchor;

pub mod link;
pub use link::{HyperLink, HyperLinkTarget};

pub mod image;
pub use image::Image;

pub mod text;
pub use text::{Span, Text, TextStyle};
