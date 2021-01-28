/*!
Write the document in the [XWiki](https://www.xwiki.org/xwiki/bin/view/Documentation/UserGuide/Features/XWikiSyntax/)
format.

# Example

```rust
# use somedoc::model::Document;
use somedoc::write::OutputFormat;
use somedoc::write::xwiki::writer;

# fn make_some_document() -> Document { Document::default() }
let doc = make_some_document();

writer(&doc, &mut std::io::stdout()).unwrap();
```
*/

use crate::model::block::{
    CodeBlock, Formatted, HeadingLevel, ListKind, MathBlock, ParagraphStyle,
};
use crate::model::inline::{
    Anchor, Character, HyperLink, HyperLinkTarget, Image, Math, SpanStyle, Text,
};
use crate::model::visitor::{walk_document, BlockVisitor, DocumentVisitor, InlineVisitor};
use crate::model::Document;
use std::cell::{RefCell, RefMut};
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct XWikiWriter<'a, W: Write> {
    block_quoted: RefCell<u8>,
    list_stack: RefCell<Vec<ListKind>>,
    sol_stack: RefCell<Vec<String>>,
    w: RefCell<&'a mut W>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Implementation of the writer function for XWiki.
///
/// While this can be called directly it is most often used  by calling either
/// [`model::write_document`](../fn.write_document.html) or
/// [`model::write_document_to_string`](../fn.write_document_to_string.html).
///
pub fn writer<W: Write>(doc: &Document, w: &mut W) -> crate::error::Result<()> {
    info!("xwiki::writer(.., ..)");
    let writer = XWikiWriter::new(w);
    walk_document(doc, &writer)?;
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<'a, W: Write> XWikiWriter<'a, W> {
    pub fn new(w: &'a mut W) -> Self {
        Self {
            block_quoted: RefCell::from(0),
            list_stack: RefCell::new(Vec::default()),
            sol_stack: RefCell::new(Vec::default()),
            w: RefCell::from(w),
        }
    }

    fn start_line(&self, w: &mut RefMut<&'a mut W>) -> crate::error::Result<()> {
        if !self.sol_stack.borrow().is_empty() {
            let prefix = self.sol_stack.borrow().join("");
            if !prefix.is_empty() {
                write!(w, "{} ", prefix)?;
            }
        }
        Ok(())
    }

    fn end_line(&self, w: &mut RefMut<&'a mut W>) -> crate::error::Result<()> {
        writeln!(w)?;
        Ok(())
    }

    fn split_text(&self, text: &str) -> String {
        if self.sol_stack.borrow().is_empty() || !text.contains('\n') {
            text.to_string()
        } else {
            let prefix = format!("\n{}", self.sol_stack.borrow().join(""));
            let lines: Vec<String> = text.split('\n').map(str::to_string).collect();
            lines.join(&prefix)
        }
    }
}

impl<'a, W: Write> DocumentVisitor for XWikiWriter<'a, W> {
    fn block_visitor(&self) -> Option<&dyn BlockVisitor> {
        Some(self)
    }
}

impl<'a, W: Write> BlockVisitor for XWikiWriter<'a, W> {
    fn start_block(&self) -> crate::error::Result<()> {
        self.start_line(&mut self.w.borrow_mut())?;
        Ok(())
    }

    fn comment(&self, value: &str) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        if value.contains('\n') {
            write!(w, "{{{{comment}}}}")?;
            self.end_line(&mut w)?;
            self.start_line(&mut w)?;
            write!(w, "{}", self.split_text(value))?;
            self.end_line(&mut w)?;
            self.start_line(&mut w)?;
            write!(w, "{{{{/comment}}}}")?;
            self.end_line(&mut w)?;
        } else {
            write!(w, "{{{{comment}}}}{}{{{{/comment}}}}", value)?;
            self.end_line(&mut w)?;
        }
        Ok(())
    }

    fn start_heading(&self, level: &HeadingLevel) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        for _ in 0..level.clone() as u8 {
            write!(w, "=")?;
        }
        write!(w, " ")?;
        Ok(())
    }

    fn end_heading(&self, level: &HeadingLevel) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        write!(w, " ")?;
        for _ in 0..level.clone() as u8 {
            write!(w, "=")?;
        }
        self.end_line(&mut w)
    }

    fn image(&self, value: &Image) -> crate::error::Result<()> {
        if let Some(image_visitor) = self.inline_visitor() {
            image_visitor.image(value)?;
        }
        self.end_line(&mut self.w.borrow_mut())?;
        Ok(())
    }

    fn math(&self, value: &MathBlock) -> crate::error::Result<()> {
        self.inline_visitor().unwrap().math(value.inner())?;
        self.end_line(&mut self.w.borrow_mut())?;
        Ok(())
    }

    fn start_list(&self, kind: &ListKind) -> crate::error::Result<()> {
        self.list_stack.borrow_mut().push(kind.clone());
        Ok(())
    }

    fn end_list(&self, _: &ListKind) -> crate::error::Result<()> {
        self.list_stack.borrow_mut().pop();
        Ok(())
    }

    fn start_list_item(&self) -> crate::error::Result<()> {
        let list_stack = self.list_stack.borrow();
        if !list_stack.is_empty() {
            let mut w = self.w.borrow_mut();
            for kind in self.list_stack.borrow().iter() {
                write!(
                    w,
                    "{}",
                    match kind {
                        ListKind::Ordered => "1",
                        ListKind::Unordered => "*",
                    }
                )?;
            }
            if let Some(kind) = list_stack.last() {
                if *kind == ListKind::Ordered {
                    write!(w, ".")?;
                }
                write!(w, " ")?;
            }
        }
        Ok(())
    }

    fn end_list_item(&self) -> crate::error::Result<()> {
        self.end_line(&mut self.w.borrow_mut())
    }

    //    fn start_definition_list(&self) -> crate::error::Result<()> {}
    //    fn end_definition_list(&self) -> crate::error::Result<()> {}

    fn start_definition_list_term(&self) -> crate::error::Result<()> {
        write!(self.w.borrow_mut(), "; ")?;
        Ok(())
    }

    //    fn end_definition_list_term(&self) -> crate::error::Result<()> {}

    fn start_definition_list_text(&self) -> crate::error::Result<()> {
        write!(self.w.borrow_mut(), ": ")?;
        Ok(())
    }

    //    fn end_definition_list_text(&self) -> crate::error::Result<()> {}

    fn formatted(&self, value: &Formatted) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        write!(w, "{{{{{{")?;
        self.end_line(&mut w)?;
        self.start_line(&mut w)?;
        write!(w, "{}", self.split_text(value.inner()))?;
        self.end_line(&mut w)?;
        self.start_line(&mut w)?;
        write!(w, "}}}}}}")?;
        self.end_line(&mut w)?;
        Ok(())
    }

    fn code_block(&self, value: &CodeBlock) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        write!(w, "{{{{code")?;
        if let Some(language) = value.language() {
            write!(w, " language=\"{}\"", language)?;
        }
        write!(w, "}}}}")?;
        self.end_line(&mut w)?;
        self.start_line(&mut w)?;
        write!(w, "{}", self.split_text(value.code()))?;
        self.end_line(&mut w)?;
        self.start_line(&mut w)?;
        write!(w, "{{{{/code}}}}")?;
        self.end_line(&mut w)?;
        Ok(())
    }

    //    fn start_paragraph(&self, styles: &Vec<ParagraphStyle>) -> crate::error::Result<()> {}

    fn end_paragraph(&self, _: &Vec<ParagraphStyle>) -> crate::error::Result<()> {
        self.end_line(&mut self.w.borrow_mut())
    }

    fn start_quote(&self) -> crate::error::Result<()> {
        self.sol_stack.borrow_mut().push(">".to_string());
        Ok(())
    }
    fn end_quote(&self) -> crate::error::Result<()> {
        self.sol_stack.borrow_mut().pop();
        Ok(())
    }

    fn thematic_break(&self) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        write!(&mut w, "-----")?;
        self.end_line(&mut w)?;
        Ok(())
    }

    fn end_block(&self) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.start_line(&mut w)?;
        self.end_line(&mut w)?;
        Ok(())
    }

    fn inline_visitor(&self) -> Option<&dyn InlineVisitor> {
        Some(self)
    }
}

impl<'a, W: Write> InlineVisitor for XWikiWriter<'a, W> {
    fn anchor(&self, value: &Anchor) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        write!(w, "(% id=\"{}\" %)", value.inner())?;
        Ok(())
    }

    fn link(&self, value: &HyperLink) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        write!(w, "[[")?;
        if let Some(alt_text) = value.alt_text() {
            write!(w, "{}", alt_text.inner())?;
            write!(w, ">>")?;
        }
        match value.target() {
            HyperLinkTarget::External(value) => write!(w, "{}]]", value)?,
            HyperLinkTarget::Internal(value) => write!(
                w,
                ".||anchor=H{}]]",
                value
                    .inner()
                    .trim()
                    .replace(" ", "")
                    .replace("(", "28")
                    .replace(")", "29")
            )?,
        }
        Ok(())
    }

    fn image(&self, value: &Image) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        write!(w, "image:")?;
        self.link(value.link())
    }

    fn text(&self, value: &Text) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        write!(w, "{}", value.inner())?;
        Ok(())
    }

    fn math(&self, value: &Math) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        write!(w, "{{{{formula}}}}{}{{{{/formula}}}}", value.inner())?;
        Ok(())
    }

    fn character(&self, value: &Character) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        write!(
            w,
            "{}",
            match value {
                Character::Space => " ".to_string(),
                Character::NonBreakSpace => "&nbsp;".to_string(),
                Character::Hyphen => "-".to_string(),
                Character::EmDash => "---".to_string(),
                Character::EnDash => "--".to_string(),
                Character::Emoji(name) => format!(":{}:", name.inner()),
                Character::Other(c) => c.to_string(),
            }
        )?;
        Ok(())
    }

    fn start_span(&self, styles: &Vec<SpanStyle>) -> crate::error::Result<()> {
        for style in styles {
            let delim: &str = match style {
                SpanStyle::Plain => "",
                SpanStyle::Italic | SpanStyle::Slanted => "//",
                SpanStyle::Bold => "**",
                SpanStyle::Mono | SpanStyle::Code => "##",
                SpanStyle::Strikethrough => "--",
                SpanStyle::Underline => "__",
                SpanStyle::SmallCaps => "",
                SpanStyle::Superscript => "^^",
                SpanStyle::Subscript => ",,",
                _ => "",
            };
            if !delim.is_empty() {
                let mut w = self.w.borrow_mut();
                write!(w, "{}", delim)?;
            }
        }
        Ok(())
    }

    fn line_break(&self) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        write!(w, "\\")?;
        self.end_line(&mut w)?;
        self.start_line(&mut w)?;
        Ok(())
    }

    fn end_span(&self, styles: &Vec<SpanStyle>) -> crate::error::Result<()> {
        for style in styles.iter().rev() {
            let delim: &str = match style {
                SpanStyle::Plain => "",
                SpanStyle::Italic | SpanStyle::Slanted => "//",
                SpanStyle::Bold => "**",
                SpanStyle::Mono | SpanStyle::Code => "##",
                SpanStyle::Strikethrough => "--",
                SpanStyle::Underline => "__",
                SpanStyle::SmallCaps => "",
                SpanStyle::Superscript => "^^",
                SpanStyle::Subscript => ",,",
                _ => "",
            };
            if !delim.is_empty() {
                let mut w = self.w.borrow_mut();
                write!(w, "{}", delim)?;
            }
        }
        Ok(())
    }
}
