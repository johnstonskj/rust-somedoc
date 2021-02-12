// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
#[macro_export]
macro_rules! alignment_impl {
    ($name:ident) => {
        impl HasAlignment for $name {
            fn alignment(&self) -> &Alignment {
                &self.alignment
            }

            fn set_alignment(&mut self, alignment: Alignment) -> &mut Self {
                self.alignment = alignment;
                self
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! label_impl {
    ($name:ident) => {
        impl HasLabel for $name {
            fn has_label(&self) -> bool {
                self.label.is_some()
            }

            fn label(&self) -> &Option<Label> {
                &self.label
            }

            fn set_label(&mut self, label: Label) -> &mut Self {
                self.label = Some(label);
                self
            }

            fn unset_label(&mut self) -> &mut Self {
                self.label = None;
                self
            }
        }
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
        impl HasCaption for $struct_name {
            fn caption(&self) -> &Option<Caption> {
                &self.caption
            }

            fn set_caption(&mut self, caption: Caption) -> &mut Self {
                self.caption = Some(caption);
                self
            }

            fn unset_caption(&mut self) -> &mut Self {
                self.caption = None;
                self
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! inner_impl {
    ($outer_name:ty, $inner_name:ty) => {
        impl $outer_name {
            /// Return a reference to the inner value.
            pub fn inner(&self) -> &$inner_name {
                &self.0
            }

            /// Return the inner value, consuming the outer one.
            pub fn into_inner(self) -> $inner_name {
                self.0
            }
        }
    };
    ($outer_name:ty, $inner_name:ty, $field_name:ident) => {
        impl $outer_name {
            /// Return a reference to the inner value.
            pub fn inner(&self) -> &$inner_name {
                &self.$field_name
            }

            /// Return the inner value, consuming the outer one.
            pub fn into_inner(self) -> $inner_name {
                self.$field_name
            }
        }
    };
}
