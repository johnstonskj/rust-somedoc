// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
#[macro_export]
macro_rules! doc {
    ($statements:block) => {{
        let mut doc = Document::default();

        $statements

        doc
    }};
}

/// Create a new `Span` with the provided text and the style `TextStyle::Bold`.
#[macro_export]
macro_rules! textbf {
    ($s:expr) => {
        Span::bold($s);
    };
}

/// Create a new `Span` with the provided text and the style `TextStyle::Italic`.
#[macro_export]
macro_rules! textit {
    ($s:expr) => {
        Span::italic($s);
    };
}

/// Create a new `Span` with the provided text and the style `TextStyle::Slanted`.
#[macro_export]
macro_rules! textsl {
    ($s:expr) => {
        Span::slanted($s);
    };
}

/// Create a new `Span` with the provided text and the style `TextStyle::Mono`.
#[macro_export]
macro_rules! texttt {
    ($s:expr) => {
        Span::mono($s);
    };
}

/// Create a new `Span` with the provided text and the style `TextStyle::Plain`.
#[macro_export]
macro_rules! text {
    ($s:expr) => {
        Span::plain($s);
    };
}

/// Create a new `Span` with the provided text and the style `TextStyle::SmallCaps`.
#[macro_export]
macro_rules! textsc {
    ($s:expr) => {
        Span::small_caps($s);
    };
}

/// Create a new `Span` with the provided text and the style `TextStyle::Strikethrough`.
#[macro_export]
macro_rules! strike {
    ($s:expr) => {
        Span::strikethrough($s);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! block_impls {
    ($name:ident) => {
        impl Into<BlockContent> for $name {
            fn into(self) -> BlockContent {
                BlockContent::$name(self)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! has_block_impls {
    ($name:ident) => {
        impl HasInnerContent<BlockContent> for $name {
            fn inner(&self) -> &Vec<BlockContent> {
                &self.content
            }

            fn into_inner(self) -> Vec<BlockContent> {
                self.content
            }

            fn inner_mut(&mut self) -> &mut Vec<BlockContent> {
                &mut self.content
            }

            fn add_content(&mut self, content: BlockContent) -> error::Result<()> {
                self.content.push(content);
                Ok(())
            }
        }

        impl HasBlockContent for $name {}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! inline_impls {
    ($name:ident) => {
        impl Into<InlineContent> for $name {
            fn into(self) -> InlineContent {
                InlineContent::$name(self)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! has_inline_impls {
    ($name:ident) => {
        impl HasInnerContent<InlineContent> for $name {
            fn inner(&self) -> &Vec<InlineContent> {
                &self.inner
            }

            fn into_inner(self) -> Vec<InlineContent> {
                self.inner
            }

            fn inner_mut(&mut self) -> &mut Vec<InlineContent> {
                &mut self.inner
            }

            fn add_content(&mut self, content: InlineContent) -> error::Result<()> {
                self.inner.push(content);
                Ok(())
            }
        }

        impl HasInlineContent for $name {}

        impl From<InlineContent> for $name {
            fn from(value: InlineContent) -> Self {
                let mut new_self = Self::default();
                new_self.add_content(value).unwrap();
                new_self
            }
        }

        impl From<Vec<InlineContent>> for $name {
            fn from(value: Vec<InlineContent>) -> Self {
                let mut new_self = Self::default();
                let mut value = value;
                for value in value.drain(..) {
                    new_self.add_content(value).unwrap();
                }
                new_self
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self::text_str(&s)
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self::text_str(s)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! has_styles_impls {
    ($struct_name:ty, $style_name:ty) => {
        impl HasStyles<$style_name> for $struct_name {
            fn styles(&self) -> &Vec<$style_name> {
                &self.styles
            }

            fn styles_mut(&mut self) -> &mut Vec<$style_name> {
                &mut self.styles
            }

            fn add_style(&mut self, style: $style_name) -> error::Result<()> {
                self.styles.push(style);
                Ok(())
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! has_captioned_impls {
    ($struct_name:ty) => {
        impl Captioned for $struct_name {
            fn caption(&self) -> &Option<Caption> {
                &self.caption
            }

            fn set_caption(&mut self, caption: Caption) {
                self.caption = Some(caption);
            }

            fn unset_caption(&mut self) {
                self.caption = None;
            }
        }
    };
}
