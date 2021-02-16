/*!
* Write a document in one of a number of light-weight markdown formats.
* format.
*
* # Example
*
* ```rust
* # use somedoc::model::Document;
* use somedoc::write::OutputFormat;
* use somedoc::write::markdown::{writer, MarkdownFlavor};
*
* # fn make_some_document() -> Document { Document::default() }
* let doc = make_some_document();
*
* writer(&doc, MarkdownFlavor::GitHub, &mut std::io::stdout()).unwrap();
* ```
*/

use crate::error;
use crate::model::block::{
    Alignment, Caption, Column, HasAlignment, HasCaption, HeadingLevel, Label, ListKind,
};
use crate::model::document::Metadata;
use crate::model::inline::{Character, HyperLink, HyperLinkTarget, Image, Math, SpanStyle, Text};
use crate::model::visitor::{
    walk_document, BlockVisitor, DocumentVisitor, InlineVisitor, TableVisitor,
};
use crate::model::Document;
use crate::write::utils::string_of_strings;
use crate::write::{ConfigurableWriter, OutputFormat, Writer};
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This defines the supported flavors of markdown.
///
#[derive(Clone, Debug, PartialEq)]
pub enum MarkdownFlavor {
    /// See <https://daringfireball.net/projects/markdown/syntax>
    /// and <https://www.markdownguide.org/basic-syntax/>
    Strict,

    /// See <https://spec.commonmark.org/0.29/>
    CommonMark,

    /// See <https://github.github.com/gfm/>
    GitHub,

    // See <https://docs.gitlab.com/ee/user/markdown.html>
    // GitLab,

    // See <https://kramdown.gettalong.org/quickref.html>
    // Kramdown,

    // See <https://pandoc.org/MANUAL.html#pandocs-markdown>
    // Pandoc,

    // See <https://docutils.sourceforge.io/docs/user/rst/quickref.html>
    // ReStructuredText,
    /// See <https://rawgit.com/fletcher/MultiMarkdown-6-Syntax-Guide/master/index.html>
    Multi,

    /// See <https://michelf.ca/projects/php-markdown/extra/>
    PhpExtra,

    /// See <https://www.xwiki.org/xwiki/bin/view/XWiki/XWikiSyntax?syntax=2.1>
    XWiki,
    // See <https://www.mediawiki.org/wiki/Help:Formatting>
    // MediaWiki,

    // See <https://confluence.atlassian.com/display/CONF20/Confluence+Notation+Guide+Overview>
    // Confluence,
}

///
/// Implementation of the Markdown writer structure, usually this is accessed via the `writer`
/// function, but may be used directly.
///
/// # Example
///
/// ```rust
/// # use somedoc::model::Document;
/// use somedoc::write::markdown::MarkdownWriter;
/// use somedoc::write::{write_document_to_string, Writer};
/// use somedoc::model::visitor::walk_document;
///
/// # fn make_some_document() -> Document { Document::default() }
/// let doc = make_some_document();
/// let mut out = std::io::stdout();
/// let writer = MarkdownWriter::new(&mut out);
/// assert!(writer.write_document(&doc).is_ok());
/// ```
///
#[derive(Debug)]
pub struct MarkdownWriter<'a, W: Write> {
    flavor: MarkdownFlavor,
    in_metadata: RefCell<bool>,
    list_prefix_stack: RefCell<Vec<ListKind>>,
    line_prefix_stack: RefCell<Vec<String>>,
    table_sep_row: RefCell<Vec<String>>,
    w: RefCell<&'a mut W>,
    debug: bool,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

enum DebugMark {
    SOB,
    EOB,
    SOQ,
    EOQ,
    SOL,
    EOL,
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
#[inline]
pub fn writer<W: Write>(
    doc: &Document,
    flavor: MarkdownFlavor,
    w: &mut W,
) -> crate::error::Result<()> {
    let writer = MarkdownWriter::new_with(w, flavor);
    writer.write_document(doc)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for MarkdownFlavor {
    fn default() -> Self {
        Self::CommonMark
    }
}

impl Display for MarkdownFlavor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MarkdownFlavor::Strict => "markdown",
                MarkdownFlavor::CommonMark => "commonmark",
                MarkdownFlavor::GitHub => "gfm",
                MarkdownFlavor::Multi => "multi",
                MarkdownFlavor::PhpExtra => "mdextra",
                MarkdownFlavor::XWiki => "xwiki",
            }
        )
    }
}

impl Into<OutputFormat> for MarkdownFlavor {
    fn into(self) -> OutputFormat {
        OutputFormat::Markdown(self)
    }
}

impl FromStr for MarkdownFlavor {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "strict" => Ok(Self::Strict),
            "cm" | "common" | "commonmark" => Ok(Self::CommonMark),
            "gfm" | "github" => Ok(Self::GitHub),
            "mmd" | "multi" => Ok(Self::Multi),
            "php_extra" | "mdextra" => Ok(Self::PhpExtra),
            "xwiki" => Ok(Self::XWiki),
            _ => Err(error::ErrorKind::UnknownFormat.into()),
        }
    }
}

impl<'a, W: Write> Writer<'a, W> for MarkdownWriter<'a, W> {
    fn new(w: &'a mut W) -> Self {
        Self::new_with(w, Default::default())
    }

    fn write_document(&self, doc: &Document) -> crate::error::Result<()> {
        walk_document(doc, self)?;
        Ok(())
    }
}

impl<'a, W: Write> ConfigurableWriter<'a, W, MarkdownFlavor> for MarkdownWriter<'a, W> {
    fn new_with(w: &'a mut W, config: MarkdownFlavor) -> Self {
        Self {
            flavor: config,
            in_metadata: RefCell::from(false),
            list_prefix_stack: RefCell::from(Vec::default()),
            line_prefix_stack: RefCell::from(Vec::default()),
            table_sep_row: RefCell::from(Vec::default()),
            w: RefCell::from(w),
            debug: false,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a, W: Write> MarkdownWriter<'a, W> {
    #[inline]
    fn debug(&self, mark: DebugMark) -> crate::error::Result<()> {
        if self.debug {
            self.write(&mark.to_string())?;
        }
        Ok(())
    }

    fn write_label_before(&self, label: &Option<Label>) -> crate::error::Result<()> {
        if let Some(label) = label {
            match self.flavor {
                MarkdownFlavor::Multi => {
                    self.write(&format!("[{}] ", label.to_string()))?;
                }
                MarkdownFlavor::XWiki => {
                    self.write(&format!("(% id=\"{}\" %) ", label.to_string()))?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn write_label_after(&self, label: &Option<Label>) -> crate::error::Result<()> {
        if let Some(label) = label {
            match self.flavor {
                MarkdownFlavor::PhpExtra => {
                    self.write(&format!(" {{#{}}}", label.to_string()))?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn write(&self, text: &str) -> crate::error::Result<()> {
        if self.line_prefix_stack.borrow().is_empty() || !text.contains('\n') {
            // if no prefix stack just let `write!` handle newline processing.
            write!(&mut self.w.borrow_mut(), "{}", text)?;
        } else {
            for line in text.split('\n') {
                self.write(line)?;
                self.end_line()?;
                self.start_line()?;
            }
            if text.ends_with('\n') {
                self.end_line()?;
            }
        }
        Ok(())
    }

    fn start_line(&self) -> crate::error::Result<()> {
        self.debug(DebugMark::SOL)?;
        if !self.line_prefix_stack.borrow().is_empty() {
            let prefix = self.line_prefix_stack.borrow().join("");
            self.write(&format!("{} ", prefix))?;
        }
        Ok(())
    }

    fn end_line(&self) -> crate::error::Result<()> {
        self.debug(DebugMark::EOL)?;
        writeln!(&mut self.w.borrow_mut())?;
        Ok(())
    }

    fn make_style_stack(&self, styles: &[SpanStyle]) -> Vec<&str> {
        let mut style_stack = Vec::new();
        for style in styles {
            match style {
                SpanStyle::Plain => {
                    style_stack.clear();
                }
                SpanStyle::Italic => {
                    if self.flavor == MarkdownFlavor::XWiki {
                        style_stack.push("//")
                    } else {
                        style_stack.push("*")
                    }
                }
                SpanStyle::Bold => style_stack.push("**"),
                SpanStyle::Mono | SpanStyle::Code => {
                    if self.flavor == MarkdownFlavor::XWiki {
                        style_stack.push("##")
                    } else {
                        style_stack.push("`")
                    }
                }
                SpanStyle::Strikethrough => {
                    if self.flavor == MarkdownFlavor::GitHub {
                        style_stack.push("~~")
                    } else if self.flavor == MarkdownFlavor::XWiki {
                        style_stack.push("--")
                    }
                }
                SpanStyle::Underline => {
                    if self.flavor == MarkdownFlavor::XWiki {
                        style_stack.push("__")
                    }
                }
                SpanStyle::Superscript => {
                    if self.flavor == MarkdownFlavor::XWiki {
                        style_stack.push("^^")
                    }
                }
                SpanStyle::Subscript => {
                    if self.flavor == MarkdownFlavor::XWiki {
                        style_stack.push(",,")
                    }
                }
                _ => {}
            };
        }
        style_stack
    }
}

impl<'a, W: Write> DocumentVisitor for MarkdownWriter<'a, W> {
    fn metadata(&self, meta_datum: &Metadata) -> crate::error::Result<()> {
        if !self.in_metadata.replace(true) {
            match &self.flavor {
                MarkdownFlavor::GitHub | MarkdownFlavor::Multi => {
                    let _ = self.write("---");
                    let _ = self.end_line();
                }
                MarkdownFlavor::XWiki => {
                    let _ = self.write("{{comment}}");
                    let _ = self.end_line();
                }
                _ => {}
            }
        }
        match &self.flavor {
            MarkdownFlavor::Strict
            | MarkdownFlavor::CommonMark
            | MarkdownFlavor::GitHub
            | MarkdownFlavor::Multi
            | MarkdownFlavor::PhpExtra => {
                let _ = self.write(&format!(
                    "[_metadata_:{}]:- \"{}\"",
                    meta_datum.key(),
                    meta_datum.value_string()
                ));
            }
            MarkdownFlavor::XWiki => {
                let _ = self.write(&meta_datum.yaml_string());
            }
        }
        self.end_line()?;
        Ok(())
    }

    fn block_visitor(&self) -> Option<&dyn BlockVisitor> {
        if self.in_metadata.replace(false) {
            match self.flavor {
                MarkdownFlavor::GitHub | MarkdownFlavor::Multi => {
                    let _ = self.write("---");
                    let _ = self.end_line();
                }
                MarkdownFlavor::XWiki => {
                    let _ = self.write("{{/comment}}");
                    let _ = self.end_line();
                }
                _ => {}
            }
            let _ = self.end_line();
        }
        Some(self)
    }
}

impl<'a, W: Write> BlockVisitor for MarkdownWriter<'a, W> {
    fn start_block(&self) -> crate::error::Result<()> {
        self.debug(DebugMark::SOB)?;
        self.end_line()?;
        self.start_line()
    }

    fn comment(&self, value: &str) -> crate::error::Result<()> {
        match self.flavor {
            MarkdownFlavor::Strict
            | MarkdownFlavor::CommonMark
            | MarkdownFlavor::GitHub
            | MarkdownFlavor::Multi
            | MarkdownFlavor::PhpExtra => {
                for line in value.split('\n') {
                    self.write(&format!("[//]: # \"{}\"", line))?;
                    self.end_line()?;
                    self.start_line()?;
                }
            }
            MarkdownFlavor::XWiki => {
                self.write(&format!("{{{{comment}}}}\n{}\n{{{{/comment}}}}", value,))?;
                self.end_line()?;
            }
        }
        Ok(())
    }

    fn start_heading(
        &self,
        level: &HeadingLevel,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        self.write_label_before(label)?;
        self.write(&format!(
            "{} ",
            string_of_strings(
                if self.flavor == MarkdownFlavor::XWiki {
                    "="
                } else {
                    "#"
                },
                level.clone() as usize
            )
        ))
    }

    fn end_heading(&self, level: &HeadingLevel, label: &Option<Label>) -> crate::error::Result<()> {
        if self.flavor == MarkdownFlavor::XWiki {
            let mut s = String::new();
            for _ in 0..level.clone() as usize {
                s.push('=');
            }
            self.write(&format!(" {}", s))?;
        }
        self.write_label_after(label)?;
        Ok(())
    }

    fn image(
        &self,
        value: &Image,
        _caption: &Option<Caption>,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        if let Some(inline_visitor) = BlockVisitor::inline_visitor(self) {
            self.end_line()?;
            self.start_line()?;
            self.write_label_before(label)?;
            inline_visitor.image(value)?;
            self.write_label_after(label)?;
        }
        Ok(())
    }

    fn math(
        &self,
        value: &Math,
        _caption: &Option<Caption>,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        if let Some(inline_visitor) = BlockVisitor::inline_visitor(self) {
            self.end_line()?;
            self.start_line()?;
            self.write_label_before(label)?;
            inline_visitor.math(value)?;
            self.write_label_after(label)?;
        }
        Ok(())
    }

    fn start_list(&self, kind: &ListKind, label: &Option<Label>) -> crate::error::Result<()> {
        self.write_label_before(label)?;
        self.list_prefix_stack.borrow_mut().push(kind.clone());
        Ok(())
    }

    fn end_list(&self, _: &ListKind, label: &Option<Label>) -> crate::error::Result<()> {
        let _ = self.list_prefix_stack.borrow_mut().pop();
        self.write_label_after(label)?;
        Ok(())
    }

    fn start_list_item(&self, label: &Option<Label>) -> crate::error::Result<()> {
        self.write_label_before(label)?;
        let list_stack = self.list_prefix_stack.borrow();
        if !list_stack.is_empty() {
            let length = if self.flavor == MarkdownFlavor::XWiki {
                list_stack.len()
            } else {
                list_stack.len() - 1
            };
            for kind in list_stack.iter().take(length) {
                if self.flavor == MarkdownFlavor::XWiki {
                    self.write(match kind {
                        ListKind::Ordered => "1",
                        ListKind::Unordered => "*",
                    })?;
                } else {
                    self.write(match kind {
                        ListKind::Ordered => "   ",
                        ListKind::Unordered => "  ",
                    })?;
                }
            }
            if let Some(kind) = list_stack.last() {
                if self.flavor == MarkdownFlavor::XWiki {
                    if *kind == ListKind::Ordered {
                        self.write(".")?;
                    }
                    self.write(" ")?;
                } else if *kind == ListKind::Ordered {
                    self.write("1. ")?;
                } else {
                    self.write("* ")?;
                }
            }
        }
        Ok(())
    }

    fn end_list_item(&self, label: &Option<Label>) -> crate::error::Result<()> {
        self.write_label_after(label)?;
        self.end_line()
    }

    fn start_definition(&self, term: &str, label: &Option<Label>) -> crate::error::Result<()> {
        match self.flavor {
            MarkdownFlavor::Multi | MarkdownFlavor::PhpExtra => {
                // Do nothing
            }
            MarkdownFlavor::XWiki => {
                self.write_label_before(label)?;
                self.write(&format!("; {}", term))?;
                self.write_label_after(label)?;
                self.end_line()?;
                self.start_line()?;
            }
            _ => {
                self.write_label_before(label)?;
                self.write(&format!("**{}**:- ", term))?;
                self.write_label_after(label)?;
            }
        }
        Ok(())
    }

    fn start_definition_list_text(&self) -> crate::error::Result<()> {
        match self.flavor {
            MarkdownFlavor::Multi | MarkdownFlavor::PhpExtra | MarkdownFlavor::XWiki => {
                write!(self.w.borrow_mut(), ": ")?;
            }
            _ => {}
        }
        Ok(())
    }

    fn formatted(&self, value: &str, label: &Option<Label>) -> crate::error::Result<()> {
        self.write_label_before(label)?;
        match self.flavor {
            MarkdownFlavor::Strict
            | MarkdownFlavor::CommonMark
            | MarkdownFlavor::GitHub
            | MarkdownFlavor::Multi
            | MarkdownFlavor::PhpExtra => {
                self.line_prefix_stack.borrow_mut().push("    ".to_string());
                self.write(&format!("    {}\n", value))?;
                let _ = self.line_prefix_stack.borrow_mut().pop();
            }
            MarkdownFlavor::XWiki => {
                self.write(&format!("{{{{{{\n{}\n}}}}}}", value))?;
            }
        }
        self.write_label_after(label)
    }

    fn code_block(
        &self,
        code: &str,
        language: &Option<String>,
        _caption: &Option<Caption>,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        self.write_label_before(label)?;
        match self.flavor {
            MarkdownFlavor::Strict => {
                self.line_prefix_stack.borrow_mut().push("    ".to_string());
                self.write(&format!("    {}", code))?;
                let _ = self.line_prefix_stack.borrow_mut().pop();
            }
            MarkdownFlavor::CommonMark | MarkdownFlavor::GitHub | MarkdownFlavor::Multi => {
                if let Some(language) = language {
                    self.write(&format!("```{}\n{}\n```", language, code))?;
                } else {
                    self.write(&format!("```\n{}\n```", code))?;
                }
            }
            MarkdownFlavor::PhpExtra => {
                self.write(&format!("```\n{}\n```\n", code))?;
            }
            MarkdownFlavor::XWiki => {
                if let Some(language) = language {
                    self.write(&format!(
                        "{{{{code language=\"{}\"}}}}\n{}\n{{{{/code}}}}",
                        language, code
                    ))?;
                } else {
                    self.write(&format!("{{{{code}}}}\n{}\n{{{{/code}}}}", code))?;
                }
            }
        }
        self.write_label_after(label)
    }

    fn start_quote(&self, label: &Option<Label>) -> crate::error::Result<()> {
        self.debug(DebugMark::SOQ)?;
        self.write_label_before(label)?;
        let mut line_prefix_stack = self.line_prefix_stack.borrow_mut();
        if self.flavor == MarkdownFlavor::XWiki {
            if line_prefix_stack.is_empty() {
                line_prefix_stack.push(">".to_string());
            } else {
                line_prefix_stack.push(" >".to_string());
            }
        } else {
            line_prefix_stack.push(">".to_string());
        }
        Ok(())
    }

    fn end_quote(&self, label: &Option<Label>) -> crate::error::Result<()> {
        self.debug(DebugMark::EOQ)?;
        let _ = self.line_prefix_stack.borrow_mut().pop();
        self.write_label_after(label)
    }

    fn thematic_break(&self) -> crate::error::Result<()> {
        self.write("-----")
    }

    fn end_block(&self) -> crate::error::Result<()> {
        self.debug(DebugMark::EOB)?;
        self.end_line()?;
        self.start_line()
    }

    fn table_visitor(&self) -> Option<&dyn TableVisitor> {
        match self.flavor {
            MarkdownFlavor::GitHub
            | MarkdownFlavor::Multi
            | MarkdownFlavor::PhpExtra
            | MarkdownFlavor::XWiki => Some(self),
            _ => None,
        }
    }

    fn inline_visitor(&self) -> Option<&dyn InlineVisitor> {
        Some(self)
    }
}

impl<'a, W: Write> TableVisitor for MarkdownWriter<'a, W> {
    fn start_table(
        &self,
        _caption: &Option<Caption>,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        self.write_label_before(label)?;
        Ok(())
    }

    fn start_table_header_row(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn table_header_cell(
        &self,
        column_cell: &Column,
        _column_idx: usize,
    ) -> crate::error::Result<()> {
        self.write(&format!(
            "|{}{}",
            if self.flavor == MarkdownFlavor::XWiki {
                "="
            } else {
                self.table_sep_row.borrow_mut().push(
                    match column_cell.alignment() {
                        Alignment::Justified => "-----",
                        Alignment::Left => ":----",
                        Alignment::Right => "----:",
                        Alignment::Centered => "--:--",
                    }
                    .to_string(),
                );
                ""
            },
            column_cell.text()
        ))
    }

    fn end_table_header_row(&self) -> crate::error::Result<()> {
        self.write("|")?;
        self.end_line()?;
        self.start_line()?;
        if self.flavor != MarkdownFlavor::XWiki {
            self.write(&format!("|{}|", self.table_sep_row.borrow().join("|")))?;
            self.table_sep_row.borrow_mut().clear();
            self.end_line()?;
            self.start_line()?;
        }
        Ok(())
    }

    fn start_table_row(&self, _: usize) -> crate::error::Result<()> {
        Ok(())
    }

    fn start_table_cell(&self, _: usize, label: &Option<Label>) -> crate::error::Result<()> {
        self.write_label_before(label)?;
        self.write("|")
    }

    fn end_table_cell(&self, _: usize, label: &Option<Label>) -> crate::error::Result<()> {
        self.write_label_after(label)
    }

    fn end_table_row(&self, _: usize) -> crate::error::Result<()> {
        self.write("|")?;
        self.end_line()?;
        self.start_line()
    }

    fn inline_visitor(&self) -> Option<&dyn InlineVisitor> {
        Some(self)
    }
}

impl<'a, W: Write> InlineVisitor for MarkdownWriter<'a, W> {
    // fn anchor(&self, value: &Anchor) -> crate::error::Result<()> {
    //     if self.flavor == MarkdownFlavor::XWiki {
    //         self.write(&format!("(% id=\"{}\" %)", value.inner()))?;
    //     }
    //     Ok(())
    // }

    fn link(&self, value: &HyperLink) -> crate::error::Result<()> {
        let mut w = self.w.borrow_mut();
        write!(w, "[[")?;
        if let Some(alt_text) = value.caption() {
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
        let prefix = if self.flavor == MarkdownFlavor::XWiki {
            "image:"
        } else {
            "!"
        };
        self.write(prefix)?;
        self.link(value.inner())?;
        Ok(())
    }

    fn text(&self, value: &Text) -> crate::error::Result<()> {
        self.write(value.inner())
    }

    fn math(&self, value: &Math) -> crate::error::Result<()> {
        if self.flavor == MarkdownFlavor::XWiki {
            self.write(&format!("{{{{formula}}}}{}{{{{/formula}}}}", value.inner()))?;
        }
        Ok(())
    }

    fn character(&self, value: &Character) -> crate::error::Result<()> {
        write!(
            &mut self.w.borrow_mut(),
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

    fn line_break(&self) -> crate::error::Result<()> {
        if self.flavor == MarkdownFlavor::XWiki {
            self.write(" \\")?;
        } else {
            self.write("  \n")?;
        }
        Ok(())
    }

    fn start_span(&self, styles: &[SpanStyle]) -> crate::error::Result<()> {
        let style_stack = self.make_style_stack(styles);
        if !style_stack.is_empty() {
            self.write(&style_stack.join(""))?;
        }
        Ok(())
    }

    fn end_span(&self, styles: &[SpanStyle]) -> crate::error::Result<()> {
        let style_stack = self.make_style_stack(styles);
        if !style_stack.is_empty() {
            self.write(
                &style_stack
                    .into_iter()
                    .rev()
                    .collect::<Vec<&str>>()
                    .join(""),
            )?;
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for DebugMark {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DebugMark::SOB => "⤷",
                DebugMark::EOB => "⤶",
                DebugMark::SOQ => "«",
                DebugMark::EOQ => "»",
                DebugMark::SOL => "⇒",
                DebugMark::EOL => "⇐",
            }
        )
    }
}
