/*!
Write a document as HTML. This includes a small number of additional CSS and JavaScript assets
to support math formatting and code syntax highlighting.

# Example

```rust
# use somedoc::model::Document;
use somedoc::write::{OutputFormat, write_document_to_string};

# fn make_some_document() -> Document { Document::default() }
let doc = make_some_document();

let doc_str = write_document_to_string(&doc, OutputFormat::Html).unwrap();
println!("{}", doc_str);
```

*/

use crate::model::block::{Alignment, Caption, Column, HasCaption, HeadingLevel, Label, ListKind};
use crate::model::document::Metadata;
use crate::model::inline::{Character, HyperLink, HyperLinkTarget, Image, Math, SpanStyle, Text};
use crate::model::visitor::{
    walk_document, BlockVisitor, DocumentVisitor, InlineVisitor, TableVisitor,
};
use crate::model::Document;
use crate::write::Writer;
use regex::Regex;
use std::cell::{RefCell, RefMut};
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Implementation of the HTML writer structure, usually this is accessed via the `writer`
/// function, but may be used directly.
///
/// # Example
///
/// ```rust
/// # use somedoc::model::Document;
/// use somedoc::write::html::HtmlWriter;
/// use somedoc::write::{write_document_to_string, Writer};
/// use somedoc::model::visitor::walk_document;
///
/// # fn make_some_document() -> Document { Document::default() }
/// let doc = make_some_document();
/// let mut out = std::io::stdout();
/// let writer = HtmlWriter::new(&mut out);
/// assert!(writer.write_document(&doc).is_ok());
/// ```
///
#[derive(Debug)]
pub struct HtmlWriter<'a, W: Write> {
    state: RefCell<State>,
    indent: RefCell<usize>,
    list_level: RefCell<usize>,
    w: RefCell<&'a mut W>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
enum State {
    Empty,
    Head,
    Body,
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
#[inline]
pub fn writer<W: Write>(doc: &Document, w: &mut W) -> crate::error::Result<()> {
    let writer = HtmlWriter::new(w);
    writer.write_document(doc)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref HEADER_ID_REGEX: Regex = Regex::new(r"[#&% \t\n]+").unwrap();
}

impl<'a, W: Write> Writer<'a, W> for HtmlWriter<'a, W> {
    fn new(w: &'a mut W) -> Self {
        Self {
            state: RefCell::from(State::Empty),
            indent: RefCell::new(0),
            list_level: RefCell::from(0),
            w: RefCell::from(w),
        }
    }

    fn write_document(&self, doc: &Document) -> crate::error::Result<()> {
        walk_document(doc, self)?;
        Ok(())
    }
}

impl<'a, W: Write> HtmlWriter<'a, W> {
    fn meta_tag(
        &self,
        w: &mut RefMut<'_, &'a mut W>,
        name: &str,
        content: &str,
    ) -> crate::error::Result<()> {
        self.start_tag_with(w, "meta", &[("name", name), ("content", content)], true)?;
        self.end_line(w)
    }

    fn start_tag(
        &self,
        w: &mut RefMut<'_, &'a mut W>,
        tag: &str,
        start_line: bool,
    ) -> crate::error::Result<()> {
        if start_line {
            self.start_line(w)?;
        }
        write!(w, "<{}>", tag)?;
        Ok(())
    }

    fn start_tag_labeled(
        &self,
        w: &mut RefMut<'_, &'a mut W>,
        tag: &str,
        label: &Option<Label>,
        start_line: bool,
    ) -> crate::error::Result<()> {
        if let Some(label) = label {
            self.start_tag_with(w, tag, &[("id", &label.to_string())], start_line)
        } else {
            self.start_tag(w, tag, start_line)
        }
    }

    fn start_tag_with(
        &self,
        w: &mut RefMut<'_, &'a mut W>,
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
        w: &mut RefMut<'_, &'a mut W>,
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
        w: &mut RefMut<'_, &'a mut W>,
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
        w: &mut RefMut<'_, &'a mut W>,
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
                "<{} {}/>",
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

    fn end_line(&self, w: &mut RefMut<'_, &'a mut W>) -> crate::error::Result<()> {
        writeln!(w)?;
        Ok(())
    }

    fn start_line(&self, w: &mut RefMut<'_, &'a mut W>) -> crate::error::Result<()> {
        self.write(w, &format!("{: ^1$}", "", *self.indent.borrow() * 2))
    }

    fn indent(&self, w: &mut RefMut<'_, &'a mut W>) -> crate::error::Result<()> {
        *self.indent.borrow_mut() += 1;
        self.end_line(w)
    }

    fn indent_no_newline(&self) -> crate::error::Result<()> {
        *self.indent.borrow_mut() += 1;
        Ok(())
    }

    fn outdent(&self, _: &mut RefMut<'_, &'a mut W>) -> crate::error::Result<()> {
        *self.indent.borrow_mut() -= 1;
        Ok(())
    }

    fn write(&self, w: &mut RefMut<'_, &'a mut W>, value: &str) -> crate::error::Result<()> {
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
                self.meta_tag(&mut w, &v.key, &v.value)?;
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

    fn start_heading(
        &self,
        level: &HeadingLevel,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        self.start_tag_labeled(
            &mut self.w.borrow_mut(),
            &format!("h{}", level.clone() as u8),
            label,
            true,
        )
    }

    fn end_heading(
        &self,
        level: &HeadingLevel,
        _label: &Option<Label>,
    ) -> crate::error::Result<()> {
        self.end_tag(
            &mut self.w.borrow_mut(),
            &format!("h{}", level.clone() as u8),
            true,
        )
    }

    fn image(
        &self,
        value: &Image,
        _caption: &Option<Caption>,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        self.start_tag_labeled(&mut self.w.borrow_mut(), "div", label, true)?;
        BlockVisitor::inline_visitor(self).unwrap().image(value)?;
        self.end_tag(&mut self.w.borrow_mut(), "div", true)
    }

    fn math(
        &self,
        value: &Math,
        _caption: &Option<Caption>,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.start_tag_labeled(&mut w, "div", label, true)?;
        self.write(&mut w, &format!("\\[ {} \\]", value.inner()))?;
        self.end_tag(&mut w, "div", true)
    }

    fn start_list(&self, kind: &ListKind, label: &Option<Label>) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        if *self.list_level.borrow() > 0 {
            self.start_tag(&mut w, "li", true)?;
            self.indent(&mut w)?;
        }
        self.start_tag_labeled(
            &mut w,
            match kind {
                ListKind::Ordered => "ol",
                ListKind::Unordered => "ul",
            },
            label,
            true,
        )?;
        self.indent(&mut w)?;
        *self.list_level.borrow_mut() += 1;
        Ok(())
    }

    fn end_list(&self, kind: &ListKind, _label: &Option<Label>) -> crate::error::Result<()> {
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
        )?;
        *self.list_level.borrow_mut() -= 1;
        if *self.list_level.borrow() > 0 {
            self.outdent(&mut w)?;
            self.start_line(&mut w)?;
            self.end_tag(&mut w, "li", true)?;
        }
        Ok(())
    }

    fn start_list_item(&self, label: &Option<Label>) -> crate::error::Result<()> {
        self.start_tag_labeled(&mut self.w.borrow_mut(), "li", label, true)
    }

    fn end_list_item(&self, _label: &Option<Label>) -> crate::error::Result<()> {
        self.end_tag(&mut self.w.borrow_mut(), "li", true)
    }

    fn start_definition_list(&self, label: &Option<Label>) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.start_tag_labeled(&mut w, "dl", label, true)?;
        self.indent(&mut w)
    }

    fn end_definition_list(&self, _label: &Option<Label>) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.outdent(&mut w)?;
        self.start_line(&mut w)?;
        self.end_tag(&mut w, "dl", true)
    }

    fn start_definition(&self, term: &str, label: &Option<Label>) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.start_tag_labeled(&mut w, "dt", label, true)?;
        self.write(&mut w, term)?;
        self.end_tag(&mut w, "dt", true)
    }

    fn start_definition_list_text(&self) -> crate::error::Result<()> {
        self.start_tag(&mut self.w.borrow_mut(), "dd", true)
    }

    fn end_definition_list_text(&self) -> crate::error::Result<()> {
        self.end_tag(&mut self.w.borrow_mut(), "dd", true)
    }

    fn formatted(&self, value: &str, label: &Option<Label>) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.start_tag_labeled(&mut w, "pre", label, true)?;
        self.write(&mut w, &format!("{}\n", value))?;
        self.start_line(&mut w)?;
        self.end_tag(&mut w, "pre", true)
    }

    fn code_block(
        &self,
        code: &str,
        language: &Option<String>,
        _caption: &Option<Caption>,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.start_tag(&mut w, "pre", true)?;
        self.indent(&mut w)?;
        self.start_line(&mut w)?;
        if let Some(language) = language {
            // TODO: add label
            self.start_tag_with(&mut w, "code", &[("class", language.as_str())], false)?;
        } else {
            self.start_tag_labeled(&mut w, "code", label, false)?;
        }

        self.write(&mut w, &format!("{}\n", code))?;

        self.start_line(&mut w)?;
        self.end_tag(&mut w, "code", true)?;
        self.outdent(&mut w)?;
        self.start_line(&mut w)?;
        self.end_tag(&mut w, "pre", true)
    }

    fn start_paragraph(
        &self,
        _alignment: &Alignment,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        // TODO: complete the addition of styles.
        self.start_tag_labeled(&mut self.w.borrow_mut(), "p", label, true)?;
        Ok(())
    }

    fn end_paragraph(
        &self,
        _alignment: &Alignment,
        _label: &Option<Label>,
    ) -> crate::error::Result<()> {
        // TODO: complete the addition of styles.
        self.end_tag(&mut self.w.borrow_mut(), "p", true)
    }

    fn start_quote(&self, label: &Option<Label>) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.start_tag_labeled(&mut w, "blockquote", label, true)?;
        self.indent(&mut w)
    }

    fn end_quote(&self, _label: &Option<Label>) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.outdent(&mut w)?;
        self.start_line(&mut w)?;
        self.end_tag(&mut w, "blockquote", true)
    }

    fn thematic_break(&self) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
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

impl<'a, W: Write> TableVisitor for HtmlWriter<'a, W> {
    fn start_table(
        &self,
        caption: &Option<Caption>,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.start_tag_labeled(&mut w, "table", label, true)?;
        self.indent(&mut w)?;
        if let Some(caption) = caption {
            self.start_tag(&mut w, "caption", true)?;
            self.write(&mut w, caption)?;
            self.end_tag(&mut w, "caption", true)?;
        }
        Ok(())
    }

    fn start_table_header_row(&self) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.start_tag(&mut w, "thead", true)?;
        self.indent(&mut w)?;
        self.start_line(&mut w)?;
        self.start_tag(&mut w, "tr", false)
    }

    fn table_header_cell(&self, column_cell: &Column, _: usize) -> crate::error::Result<()> {
        self.write(
            &mut self.w.borrow_mut(),
            &format!("<th>{}</th>", column_cell.text()),
        )
    }

    fn end_table_header_row(&self) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.end_tag(&mut w, "tr", true)?;
        self.outdent(&mut w)?;
        self.start_line(&mut w)?;
        self.end_tag(&mut w, "thead", true)?;
        self.start_line(&mut w)?;
        self.end_tag(&mut w, "tbody", true)
    }

    fn start_table_row(&self, _: usize) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.indent_no_newline()?;
        self.start_line(&mut w)?;
        self.start_tag(&mut w, "tr", false)
    }

    fn start_table_cell(&self, _: usize, label: &Option<Label>) -> crate::error::Result<()> {
        self.start_tag_labeled(&mut self.w.borrow_mut(), "td", label, false)
    }

    fn end_table_cell(&self, _: usize, _label: &Option<Label>) -> crate::error::Result<()> {
        self.end_tag(&mut self.w.borrow_mut(), "td", false)
    }

    fn end_table_row(&self, _: usize) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.end_tag(&mut w, "tr", true)?;
        self.outdent(&mut w)
    }

    fn end_table(&self, _: &Option<Caption>, _label: &Option<Label>) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        self.start_line(&mut w)?;
        self.end_tag(&mut w, "tbody", true)?;
        self.outdent(&mut w)?;
        self.start_line(&mut w)?;
        self.end_tag(&mut w, "table", true)
    }

    fn inline_visitor(&self) -> Option<&dyn InlineVisitor> {
        Some(self)
    }
}

impl<'a, W: Write> InlineVisitor for HtmlWriter<'a, W> {
    fn link(&self, value: &HyperLink) -> crate::error::Result<()> {
        if let Some(caption) = value.caption() {
            self.start_tag_with(
                &mut self.w.borrow_mut(),
                "a",
                &[(
                    "href",
                    &match value.target() {
                        HyperLinkTarget::External(v) => v.to_string(),
                        HyperLinkTarget::Internal(v) => format!("#{}", self.anchor_id(v)),
                    },
                )],
                false,
            )?;
            self.write(&mut self.w.borrow_mut(), caption.inner())?;
            self.end_tag(&mut self.w.borrow_mut(), "a", false)
        } else {
            self.closed_tag_with(
                &mut self.w.borrow_mut(),
                "a",
                &[(
                    "href",
                    &match value.target() {
                        HyperLinkTarget::External(v) => v.to_string(),
                        HyperLinkTarget::Internal(v) => format!("#{}", self.anchor_id(v)),
                    },
                )],
                false,
                false,
            )
        }
    }

    fn image(&self, value: &Image) -> crate::error::Result<()> {
        self.closed_tag_with(
            &mut self.w.borrow_mut(),
            "img",
            &[("src", &value.inner())],
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
            &match value {
                Character::Space => "&#32;".to_string(),
                Character::NonBreakSpace => "&nbsp;".to_string(),
                Character::Hyphen => "&dash;".to_string(),
                Character::EmDash => "&mdash;".to_string(),
                Character::EnDash => "&ndash;".to_string(),
                Character::Emoji(v) => v.inner().to_string(),
                Character::Other(v) => v.to_string(),
            },
        )
    }

    fn line_break(&self) -> crate::error::Result<()> {
        self.closed_tag(&mut self.w.borrow_mut(), "br", false, true)?;
        Ok(())
    }

    fn start_span(&self, styles: &[SpanStyle]) -> crate::error::Result<()> {
        for tag in self.span_tags(styles) {
            self.start_tag(&mut self.w.borrow_mut(), &tag, false)?;
        }
        Ok(())
    }

    fn end_span(&self, styles: &[SpanStyle]) -> crate::error::Result<()> {
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

    fn span_tags(&self, styles: &[SpanStyle]) -> Vec<String> {
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
