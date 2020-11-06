/*!
One-line description.

More detailed description, with

# Example

*/

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! doc {
    ($statements:block) => {{
        let mut doc = Document::default();

        $statements

        doc
    }};
}

#[macro_export]
macro_rules! textbf {
    ($s:expr) => {
        Span::bold($s);
    };
}

#[macro_export]
macro_rules! textit {
    ($s:expr) => {
        Span::italic($s);
    };
}

#[macro_export]
macro_rules! textsl {
    ($s:expr) => {
        Span::slanted($s);
    };
}

#[macro_export]
macro_rules! texttt {
    ($s:expr) => {
        Span::mono($s);
    };
}

#[macro_export]
macro_rules! text {
    ($s:expr) => {
        Span::plain($s);
    };
}

#[macro_export]
macro_rules! textsc {
    ($s:expr) => {
        Span::small_caps($s);
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
macro_rules! has_inline_impls {
    ($name:ident) => {
        impl ComplexContent<InlineContent> for $name {
            fn inner(&self) -> &Vec<InlineContent> {
                &self.inner
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
