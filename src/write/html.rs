/*!
One-line description.

More detailed description, with

# Example

*/

use crate::model::block::{
    CodeBlock, Formatted, HeadingLevel, ListKind, MathBlock, ParagraphStyle,
};
use crate::model::document::Metadata;
use crate::model::inline::{
    Anchor, Character, HyperLink, HyperLinkTarget, Image, Math, SpanStyle, Text,
};
use crate::model::visitor::{
    walk_document, BlockVisitor, DocumentVisitor, InlineVisitor, TableVisitor,
};
use crate::model::Document;
use regex::Regex;
use std::cell::{RefCell, RefMut};
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
enum State {
    Empty,
    Head,
    Body,
}

#[derive(Debug)]
struct HtmlWriter<'a, W: Write> {
    state: RefCell<State>,
    indent: RefCell<usize>,
    w: RefCell<&'a mut W>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Implementation of the writer function for HTML.
///
/// While this can be called directly it is most often used  by calling either
/// [`model::write_document`](../fn.write_document.html) or
/// [`model::write_document_to_string`](../fn.write_document_to_string.html).
///
pub fn writer<W: Write>(doc: &Document, w: &mut W) -> crate::error::Result<()> {
    let writer = HtmlWriter::new(w);
    walk_document(doc, &writer)?;
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref HEADER_ID_REGEX: Regex = Regex::new(r"[#&% \t\n]+").unwrap();
}

impl<'a, W: Write> HtmlWriter<'a, W> {
    pub fn new(w: &'a mut W) -> Self {
        Self {
            state: RefCell::from(State::Empty),
            indent: RefCell::new(0),
            w: RefCell::from(w),
        }
    }

    fn meta_tag(
        &self,
        w: &mut RefMut<&'a mut W>,
        name: &str,
        content: &str,
    ) -> crate::error::Result<()> {
        self.start_tag_with(w, "meta", &[("name", name), ("content", content)], true)?;
        self.end_line(w)
    }

    fn start_tag(
        &self,
        w: &mut RefMut<&'a mut W>,
        tag: &str,
        start_line: bool,
    ) -> crate::error::Result<()> {
        if start_line {
            self.start_line(w)?;
        }
        write!(w, "<{}>", tag)?;
        Ok(())
    }

    fn start_tag_with(
        &self,
        w: &mut RefMut<&'a mut W>,
        tag: &str,
        attributes: &[(&str, &str)],
        start_line: bool,
    ) -> crate::error::Result<()> {
        if start_line {
            self.start_line(w)?;
        }
        self.write(
            w,
            &format!(
                "<{} {}>",
                tag,
                attributes
                    .iter()
                    .map(|(k, v)| format!("{}=\"{}\"", k, v))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
        )
    }

    fn end_tag(
        &self,
        w: &mut RefMut<&'a mut W>,
        tag: &str,
        end_line: bool,
    ) -> crate::error::Result<()> {
        self.write(w, &format!("</{}>", tag))?;
        if end_line {
            self.end_line(w)?;
        }
        Ok(())
    }

    fn closed_tag(
        &self,
        w: &mut RefMut<&'a mut W>,
        tag: &str,
        start_line: bool,
        end_line: bool,
    ) -> crate::error::Result<()> {
        if start_line {
            self.start_line(w)?;
        }
        self.write(w, &format!("<{}/>", tag))?;
        if end_line {
            self.end_line(w)?;
        }
        Ok(())
    }

    fn closed_tag_with(
        &self,
        w: &mut RefMut<&'a mut W>,
        tag: &str,
        attributes: &[(&str, &str)],
        start_line: bool,
        end_line: bool,
    ) -> crate::error::Result<()> {
        if start_line {
            self.start_line(w)?;
        }
        self.write(
            w,
            &format!(
                "<{} {}>",
                tag,
                attributes
                    .iter()
                    .map(|(k, v)| format!("{}=\"{}\"", k, v))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
        )?;
        if end_line {
            self.end_line(w)?;
        }
        Ok(())
    }

    fn end_line(&self, w: &mut RefMut<&'a mut W>) -> crate::error::Result<()> {
        writeln!(w)?;
        Ok(())
    }

    fn start_line(&self, w: &mut RefMut<&'a mut W>) -> crate::error::Result<()> {
        self.write(w, &format!("{: ^1$}", "", *self.indent.borrow() * 2))
    }

    fn indent(&self, w: &mut RefMut<&'a mut W>) -> crate::error::Result<()> {
        *self.indent.borrow_mut() += 1;
        self.end_line(w)
    }

    fn outdent(&self, _: &mut RefMut<&'a mut W>) -> crate::error::Result<()> {
        *self.indent.borrow_mut() -= 1;
        Ok(())
    }

    fn write(&self, w: &mut RefMut<&'a mut W>, value: &str) -> crate::error::Result<()> {
        write!(w, "{}", value)?;
        Ok(())
    }
}

impl<'a, W: Write> DocumentVisitor for HtmlWriter<'a, W> {
    fn start_document(&self) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.start_tag(&mut w, "html", false)?;
        self.indent(&mut w)?;

        self.start_tag(&mut w, "head", true)?;
        self.indent(&mut w)?;

        self.start_tag_with(&mut w, "link", &[("rel", "stylesheet"), ("href", "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/styles/default.min.css")], true)?;
        self.end_tag(&mut w, "link", true)?;

        self.start_tag_with(
            &mut w,
            "script",
            &[("src", "https://polyfill.io/v3/polyfill.min.js?features=es6")],
            true,
        )?;
        self.end_tag(&mut w, "script", true)?;
        self.start_tag_with(
            &mut w,
            "script",
            &[
                ("id", "MathJax-script"),
                (
                    "src",
                    "https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js",
                ),
            ],
            true,
        )?;
        self.end_tag(&mut w, "script", true)?;
        self.start_tag_with(
            &mut w,
            "script",
            &[(
                "src",
                "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/highlight.min.js",
            )],
            true,
        )?;
        self.end_tag(&mut w, "script", true)?;

        *self.state.borrow_mut() = State::Head;
        Ok(())
    }

    fn metadata(&self, metadatum: &Metadata) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        match metadatum {
            Metadata::Author(v) => {
                self.meta_tag(
                    &mut w,
                    "author",
                    &format!(
                        "{}{}{}",
                        v.name,
                        match &v.email {
                            None => String::new(),
                            Some(v) => format!(" <{}>", v),
                        },
                        match &v.organization {
                            None => String::new(),
                            Some(v) => format!(" - {}", v),
                        }
                    ),
                )?;
            }
            Metadata::Class(v) => {
                self.start_tag_with(
                    &mut w,
                    "link",
                    &[("rel", "stylesheet"), ("href", &v.name_or_path)],
                    true,
                )?;
                self.end_tag(&mut w, "link", true)?;
            }
            Metadata::Copyright(v) => {
                self.meta_tag(
                    &mut w,
                    "copyright",
                    &format!(
                        "{}{}{}",
                        v.year,
                        match &v.organization {
                            None => String::new(),
                            Some(v) => format!(" {}", v),
                        },
                        match &v.comment {
                            None => String::new(),
                            Some(v) => format!(" ({})", v),
                        }
                    ),
                )?;
            }
            Metadata::Date(v) => {
                self.meta_tag(&mut w, "date", v)?;
            }
            Metadata::Keywords(v) => {
                self.meta_tag(&mut w, "keywords", &v.join(", "))?;
            }
            Metadata::Revision(v) => {
                self.meta_tag(&mut w, "revision", v)?;
            }
            Metadata::Status(v) => {
                self.meta_tag(&mut w, "status", v)?;
            }
            Metadata::Title(v) => {
                self.start_tag(&mut w, "title", true)?;
                self.write(&mut w, v)?;
                self.end_tag(&mut w, "title", true)?;
            }
            Metadata::Other(v) => {
                self.meta_tag(&mut w, &v.name, &v.value)?;
            }
        }
        Ok(())
    }

    fn block_visitor(&self) -> Option<&dyn BlockVisitor> {
        let mut w = self.w.borrow_mut();
        if *self.state.borrow() == State::Head {
            let _ = self.outdent(&mut w);
            let _ = self.start_line(&mut w);
            let _ = self.end_tag(&mut w, "head", true);
        }
        let _ = self.start_tag(&mut w, "body", true);
        let _ = self.indent(&mut w);
        *self.state.borrow_mut() = State::Body;
        Some(self)
    }

    fn end_document(&self) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        match *self.state.borrow() {
            State::Empty => {}
            State::Head => {
                self.outdent(&mut w)?;
                self.start_line(&mut w)?;
                self.end_tag(&mut w, "head", true)?;
                self.outdent(&mut w)?;
                self.start_line(&mut w)?;
                self.end_tag(&mut w, "html", false)?;
            }
            State::Body => {
                self.outdent(&mut w)?;
                self.start_line(&mut w)?;
                self.end_tag(&mut w, "body", true)?;
                self.outdent(&mut w)?;
                self.start_line(&mut w)?;
                self.end_tag(&mut w, "html", false)?;
            }
        }
        *self.state.borrow_mut() = State::Empty;
        Ok(())
    }
}

impl<'a, W: Write> BlockVisitor for HtmlWriter<'a, W> {
    fn start_block(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn comment(&self, value: &str) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        write!(w, "<!-- {} -->", value)?;
        self.end_line(&mut w)
    }

    fn start_heading(&self, level: &HeadingLevel) -> crate::error::Result<()> {
        self.start_tag(
            &mut self.w.borrow_mut(),
            &format!("h{}", level.clone() as u8),
            true,
        )?;
        Ok(())
    }

    fn end_heading(&self, level: &HeadingLevel) -> crate::error::Result<()> {
        self.end_tag(
            &mut self.w.borrow_mut(),
            &format!("h{}", level.clone() as u8),
            true,
        )
    }

    fn image(&self, value: &Image) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.start_tag(&mut w, "div", false)?;
        BlockVisitor::inline_visitor(self).unwrap().image(value)?;
        self.start_line(&mut w)?;
        self.end_tag(&mut w, "div", true)
    }

    fn math(&self, value: &MathBlock) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.start_line(&mut w)?;
        self.write(&mut w, &format!("\\[ {} \\]", value.inner().inner()))?;
        self.end_line(&mut w)
    }

    fn start_list(&self, kind: &ListKind) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.start_tag(
            &mut w,
            match kind {
                ListKind::Ordered => "ol",
                ListKind::Unordered => "ul",
            },
            true,
        )?;
        self.indent(&mut w)
    }

    fn end_list(&self, kind: &ListKind) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.outdent(&mut w)?;
        self.start_line(&mut w)?;
        self.end_tag(
            &mut w,
            match kind {
                ListKind::Ordered => "ol",
                ListKind::Unordered => "ul",
            },
            true,
        )
    }

    fn start_list_item(&self) -> crate::error::Result<()> {
        self.start_tag(&mut self.w.borrow_mut(), "li", true)
    }

    fn end_list_item(&self) -> crate::error::Result<()> {
        self.end_tag(&mut self.w.borrow_mut(), "li", true)
    }

    fn start_definition_list(&self) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.start_tag(&mut w, "dl", true)?;
        self.indent(&mut w)
    }

    fn end_definition_list(&self) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.outdent(&mut w)?;
        self.start_line(&mut w)?;
        self.end_tag(&mut w, "dl", true)
    }

    fn start_definition_list_term(&self) -> crate::error::Result<()> {
        self.start_tag(&mut self.w.borrow_mut(), "dt", true)
    }

    fn end_definition_list_term(&self) -> crate::error::Result<()> {
        self.end_tag(&mut self.w.borrow_mut(), "dt", true)
    }

    fn start_definition_list_text(&self) -> crate::error::Result<()> {
        self.start_tag(&mut self.w.borrow_mut(), "dd", true)
    }

    fn end_definition_list_text(&self) -> crate::error::Result<()> {
        self.end_tag(&mut self.w.borrow_mut(), "dd", true)
    }

    fn formatted(&self, value: &Formatted) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.start_tag(&mut w, "pre", true)?;
        self.write(&mut w, &format!("{}\n", value.inner()))?;
        self.start_line(&mut w)?;
        self.end_tag(&mut w, "pre", true)
    }

    fn code_block(&self, value: &CodeBlock) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.start_tag(&mut w, "pre", true)?;
        self.indent(&mut w)?;
        self.start_line(&mut w)?;
        if let Some(language) = value.language() {
            self.start_tag_with(&mut w, "code", &vec![("class", language.as_str())], false)?;
        } else {
            self.start_tag(&mut w, "code", false)?;
        }

        self.write(&mut w, &format!("{}\n", value.code()))?;

        self.start_line(&mut w)?;
        self.end_tag(&mut w, "code", true)?;
        self.outdent(&mut w)?;
        self.start_line(&mut w)?;
        self.end_tag(&mut w, "pre", true)
    }

    fn start_paragraph(&self, _styles: &Vec<ParagraphStyle>) -> crate::error::Result<()> {
        // TODO: complete the addition of styles.
        self.start_tag(&mut self.w.borrow_mut(), "p", true)?;
        Ok(())
    }

    fn end_paragraph(&self, _styles: &Vec<ParagraphStyle>) -> crate::error::Result<()> {
        // TODO: complete the addition of styles.
        self.end_tag(&mut self.w.borrow_mut(), "p", true)
    }

    fn start_quote(&self) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.start_tag(&mut w, "blockquote", true)?;
        self.indent(&mut w)
    }

    fn end_quote(&self) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.outdent(&mut w)?;
        self.start_line(&mut w)?;
        self.end_tag(&mut w, "blockquote", true)
    }

    fn thematic_break(&self) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.end_line(&mut w)?;
        self.closed_tag(&mut w, "hr", true, true)
    }

    fn end_block(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn table_visitor(&self) -> Option<&dyn TableVisitor> {
        Some(self)
    }

    fn inline_visitor(&self) -> Option<&dyn InlineVisitor> {
        Some(self)
    }
}

impl<'a, W: Write> TableVisitor for HtmlWriter<'a, W> {}

impl<'a, W: Write> InlineVisitor for HtmlWriter<'a, W> {
    fn anchor(&self, value: &Anchor) -> crate::error::Result<()> {
        self.closed_tag_with(
            &mut self.w.borrow_mut(),
            "span",
            &[("id", &self.anchor_id(value))],
            false,
            false,
        )
    }

    fn link(&self, value: &HyperLink) -> crate::error::Result<()> {
        self.closed_tag_with(
            &mut self.w.borrow_mut(),
            "a",
            &[(
                "href",
                &match value.target() {
                    HyperLinkTarget::External(v) => v.to_string(),
                    HyperLinkTarget::Internal(v) => format!("#{}", self.anchor_id(v.inner())),
                },
            )],
            false,
            false,
        )
    }

    fn image(&self, value: &Image) -> crate::error::Result<()> {
        let src = match value.link().target() {
            HyperLinkTarget::External(v) => v.to_string(),
            HyperLinkTarget::Internal(v) => format!("#{}", v.inner()),
        };
        let attributes: Vec<(&str, &str)> = if let Some(alt_text) = value.link().alt_text() {
            vec![("src", &src), ("alt", alt_text.inner())]
        } else {
            vec![("src", &src)]
        };
        self.closed_tag_with(
            &mut self.w.borrow_mut(),
            "img",
            attributes.as_slice(),
            false,
            false,
        )
    }

    fn text(&self, value: &Text) -> crate::error::Result<()> {
        self.write(&mut self.w.borrow_mut(), value)
    }

    fn math(&self, value: &Math) -> crate::error::Result<()> {
        self.write(
            &mut self.w.borrow_mut(),
            &format!("\\( {} \\)", value.inner()),
        )
    }

    fn character(&self, value: &Character) -> crate::error::Result<()> {
        self.write(
            &mut self.w.borrow_mut(),
            &format!(
                "{}",
                match value {
                    Character::Space => "&#32;".to_string(),
                    Character::NonBreakSpace => "&nbsp;".to_string(),
                    Character::Hyphen => "&dash;".to_string(),
                    Character::EmDash => "&mdash;".to_string(),
                    Character::EnDash => "&ndash;".to_string(),
                    Character::Emoji(v) => v.inner().to_string(),
                    Character::Other(v) => v.to_string(),
                }
            ),
        )
    }

    fn line_break(&self) -> crate::error::Result<()> {
        self.closed_tag(&mut self.w.borrow_mut(), "br", false, true)?;
        Ok(())
    }

    fn start_span(&self, styles: &Vec<SpanStyle>) -> crate::error::Result<()> {
        for tag in self.span_tags(styles) {
            self.start_tag(&mut self.w.borrow_mut(), &tag, false)?;
        }
        Ok(())
    }

    fn end_span(&self, styles: &Vec<SpanStyle>) -> crate::error::Result<()> {
        for tag in self.span_tags(styles).iter().rev() {
            self.end_tag(&mut self.w.borrow_mut(), &tag, false)?;
        }
        Ok(())
    }
}

impl<'a, W: Write> HtmlWriter<'a, W> {
    fn anchor_id(&self, header: &str) -> String {
        HEADER_ID_REGEX.replace_all(header, "_").to_string()
    }

    fn span_tags(&self, styles: &Vec<SpanStyle>) -> Vec<String> {
        let mut tags: Vec<String> = Default::default();
        for style in styles {
            match style {
                SpanStyle::Plain => {
                    tags.clear();
                }
                SpanStyle::Italic => {
                    tags.push("em".to_string());
                }
                SpanStyle::Bold => {
                    tags.push("strong".to_string());
                }
                SpanStyle::Mono => {
                    tags.push("code".to_string());
                }
                SpanStyle::Code => {
                    tags.push("code".to_string());
                }
                SpanStyle::Strikethrough => {
                    tags.push("del".to_string());
                }
                SpanStyle::Underline => {
                    tags.push("ins".to_string());
                }
                SpanStyle::SmallCaps => {
                    // font-variant: small-caps;
                }
                SpanStyle::Superscript => {
                    tags.push("sup".to_string());
                }
                SpanStyle::Subscript => {
                    tags.push("sub".to_string());
                }
                // font-size:
                // xx-small, x-small, small, medium, large, x-large, xx-large
                SpanStyle::Sized(_) => {}
            }
        }
        tags
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------
