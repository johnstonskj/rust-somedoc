/*!
Write a document as LaTeX. This includes a small number of additional CTAN packages
to support math formatting, images, and code syntax highlighting.

# Example

```rust
# use somedoc::model::Document;
use somedoc::write::{OutputFormat, write_document_to_string};

# fn make_some_document() -> Document { Document::default() }
let doc = make_some_document();

let doc_str = write_document_to_string(&doc, OutputFormat::Latex).unwrap();
println!("{}", doc_str);
```
*/

use crate::model::block::{
    Alignment, Caption, Column, HasAlignment, HasCaption, HeadingLevel, Label, ListKind,
};
use crate::model::document::Metadata;
use crate::model::inline::text::Size;
use crate::model::inline::{Character, HyperLink, HyperLinkTarget, Image, Math, SpanStyle, Text};
use crate::model::visitor::{
    walk_document, BlockVisitor, DocumentVisitor, InlineVisitor, TableVisitor,
};
use crate::model::Document;
use crate::write::utils::string_of_strings;
use crate::write::{ConfigurableWriter, Writer};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This enumeration represents the components included in a LaTeX file preamble; note that this
/// is not exhaustive, but useful enough for generation of most document content.
///
#[derive(Clone, Debug)]
pub enum PreambleItem {
    /// The value of the LaTeX `documentclass` initial declaration.
    Class {
        /// The class name.
        name: String,
        /// Any options to be passed to the class.
        options: Vec<String>,
    },
    /// A package to be included via `usepackage` in this file.
    Package {
        /// The package name.
        name: String,
        /// Any options to be passed to the package.
        options: Vec<String>,
    },
    /// A command to add, or renew, for this file.
    Command {
        /// Is this a new, or existing, command.
        renew: bool,
        /// The command name.
        name: String,
        /// How many arguments does this command expect.
        arg_count: usize,
        /// The definition of this command.
        define: String,
    },
    /// An environment to add, or renew, for this file.
    Environment {
        /// Is this a new, or existing, environment.
        renew: bool,
        /// The environment name.
        name: String,
        /// How many arguments does this environment expect.
        arg_count: usize,
        /// The default values for arguments to this environment.
        default_values: Vec<String>,
        /// The definition before to the environment's body.
        before: String,
        /// The definition after to the environment's body.
        after: String,
    },
}

///
/// This structure represents the preamble included in the generated LaTeX source. Note that the
/// default includes a number of package references that you'll lose if you simply replace it with
/// another. It is generally better to append to this preamble in the following manner:
///
/// ```rust
/// use somedoc::write::ConfigurableWriter;
/// use somedoc::write::latex::{LatexPreamble, PreambleItem, LatexWriter};
///
/// let mut preamble = LatexPreamble::default();
/// preamble.push(PreambleItem::package("fontenc"));
///
/// let mut out = std::io::stdout();
/// let writer = LatexWriter::new_with(&mut out, preamble);
/// ```
#[derive(Clone, Debug)]
pub struct LatexPreamble(Vec<PreambleItem>);

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Implementation of the LaTeX writer structure, usually this is accessed via the `writer`
/// function, but may be used directly.
///
/// # Example
///
/// ```rust
/// # use somedoc::model::Document;
/// use somedoc::write::latex::LatexWriter;
/// use somedoc::write::{write_document_to_string, Writer};
/// use somedoc::model::visitor::walk_document;
///
/// # fn make_some_document() -> Document { Document::default() }
/// let doc = make_some_document();
/// let mut out = std::io::stdout();
/// let writer = LatexWriter::new(&mut out);
/// assert!(walk_document(&doc, &writer).is_ok());
/// ```
///
#[derive(Debug)]
pub struct LatexWriter<'a, W: Write> {
    preamble: LatexPreamble,
    metadata: RefCell<HashMap<String, Vec<String>>>,
    table_head: RefCell<Vec<Column>>,
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
    let writer = LatexWriter::new_with(w, LatexPreamble::default());
    walk_document(doc, &writer)?;
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl PreambleItem {
    /// Create a new class item, note there can only be one class item per preamble.
    pub fn class(name: &str) -> PreambleItem {
        Self::class_with(name, &[])
    }

    /// Create a new class item with options, note there can only be one class item per preamble.
    pub fn class_with(name: &str, options: &[&str]) -> PreambleItem {
        Self::Class {
            name: name.to_string(),
            options: options.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Create a new package preamble item.
    pub fn package(name: &str) -> PreambleItem {
        Self::package_with(name, &[])
    }

    /// Create a new package preamble item with options.
    pub fn package_with(name: &str, options: &[&str]) -> PreambleItem {
        Self::Package {
            name: name.to_string(),
            options: options.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Create a new command definition.
    pub fn new_command(name: &str, define: &str) -> PreambleItem {
        Self::new_command_with(name, define, 0)
    }

    /// Create a new command definition with argument count.
    pub fn new_command_with(name: &str, define: &str, arg_count: usize) -> PreambleItem {
        Self::Command {
            renew: false,
            name: name.to_string(),
            arg_count,
            define: define.to_string(),
        }
    }

    /// Create a renew command definition.
    pub fn renew_command(name: &str, define: &str) -> PreambleItem {
        Self::renew_command_with(name, define, 0)
    }

    /// Create a renew command definition with argument count.
    pub fn renew_command_with(name: &str, define: &str, arg_count: usize) -> PreambleItem {
        Self::Command {
            renew: true,
            name: name.to_string(),
            arg_count,
            define: define.to_string(),
        }
    }

    /// Return `true` if this is a `PreambleItem::Class` variant, else `false`.
    pub fn is_class(&self) -> bool {
        matches!(self, PreambleItem::Class { .. })
    }

    /// Return `true` if this is a `PreambleItem::Package` variant, else `false`.
    pub fn is_package(&self) -> bool {
        matches!(self, PreambleItem::Package { .. })
    }

    /// Return `true` if this is a `PreambleItem::Command` variant, else `false`.
    pub fn is_command(&self) -> bool {
        matches!(self, PreambleItem::Command { .. })
    }

    /// Return `true` if this is a `PreambleItem::Package` variant, else `false`.
    pub fn is_environment(&self) -> bool {
        matches!(self, PreambleItem::Package { .. })
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for LatexPreamble {
    fn default() -> Self {
        Self(vec![
            PreambleItem::class_with("article", &["twoside", "12pt", "lettersize"]),
            PreambleItem::package("amsmath"),
            PreambleItem::package("csquotes"),
            PreambleItem::package("graphicx"),
            PreambleItem::package("hyperref"),
            PreambleItem::package("listings"),
            PreambleItem::new_command(
                "thematicbreak",
                r"\par\bigskip\noindent\hrulefill\par\bigskip",
            ),
        ])
    }
}

impl From<Vec<PreambleItem>> for LatexPreamble {
    fn from(v: Vec<PreambleItem>) -> Self {
        Self(v)
    }
}

impl LatexPreamble {
    /// Return an iterator over all the items in the preamble.
    pub fn items(&self) -> impl Iterator<Item = &PreambleItem> {
        self.0.iter()
    }

    /// Push a new item into the preamble.
    pub fn push(&mut self, item: PreambleItem) {
        self.0.push(item)
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a, W: Write> Writer<'a, W> for LatexWriter<'a, W> {
    fn new(w: &'a mut W) -> Self {
        Self::new_with(w, Default::default())
    }

    fn write_document(&self, doc: &Document) -> crate::error::Result<()> {
        walk_document(doc, self)?;
        Ok(())
    }
}

impl<'a, W: Write> ConfigurableWriter<'a, W, LatexPreamble> for LatexWriter<'a, W> {
    fn new_with(w: &'a mut W, preamble: LatexPreamble) -> Self {
        Self {
            preamble,
            metadata: RefCell::new(Default::default()),
            table_head: RefCell::new(vec![]),
            indent: RefCell::new(0),
            w: RefCell::from(w),
        }
    }
}

impl<'a, W: Write> LatexWriter<'a, W> {
    fn write(&self, s: &str) -> crate::error::Result<()> {
        write!(&mut self.w.borrow_mut(), "{}", s)?;
        Ok(())
    }

    fn end_line(&self) -> crate::error::Result<()> {
        self.write("\n")
    }

    fn end_lines(&self, count: usize) -> crate::error::Result<()> {
        for _ in 0..count {
            self.end_line()?;
        }
        Ok(())
    }

    fn begin_line(&self) -> crate::error::Result<()> {
        self.write(&string_of_strings("  ", *self.indent.borrow()))
    }

    fn write_label(&self, label: &Option<Label>) -> crate::error::Result<()> {
        if let Some(label) = label {
            self.command("label", &label)?;
        }
        Ok(())
    }

    fn just_command(&self, cmd: &str) -> crate::error::Result<()> {
        self.write(&format!("\\{}", cmd))
    }

    fn command(&self, cmd: &str, s: &str) -> crate::error::Result<()> {
        self.command_with(cmd, s, &[])
    }

    fn command_with(&self, cmd: &str, s: &str, args: &[&str]) -> crate::error::Result<()> {
        self.begin_cmd_with(cmd, args)?;
        self.write(s)?;
        self.end_cmd()
    }

    fn begin_cmd(&self, cmd: &str) -> crate::error::Result<()> {
        self.begin_cmd_with(cmd, &[])
    }

    fn begin_cmd_with(&self, cmd: &str, args: &[&str]) -> crate::error::Result<()> {
        if args.is_empty() {
            self.write(&format!("\\{}{{", cmd))
        } else {
            self.write(&format!("\\{}[{}]{{", cmd, args.join(", ")))
        }
    }

    fn end_cmd(&self) -> crate::error::Result<()> {
        self.write(r"}")
    }

    fn begin_env(&self, env: &str) -> crate::error::Result<()> {
        self.begin_env_with(env, &[])
    }

    fn begin_env_with(&self, env: &str, args: &[&str]) -> crate::error::Result<()> {
        if args.is_empty() {
            self.write(&format!("\\begin{{{}}}", env))?;
        } else {
            self.write(&format!("\\begin{{{}}}[{}]", env, args.join(", ")))?;
        }
        self.indent();
        Ok(())
    }

    fn end_env(&self, env: &str) -> crate::error::Result<()> {
        self.outdent();
        self.begin_line()?;
        self.write(&format!("\\end{{{}}}", env))
    }

    fn braced(&self, s: &str) -> crate::error::Result<()> {
        self.write(&format!("{{{}}}", s))
    }

    fn indent(&self) {
        *self.indent.borrow_mut() += 1;
    }

    fn outdent(&self) {
        *self.indent.borrow_mut() -= 1;
    }

    fn preamble(&self) -> crate::error::Result<()> {
        let md = self.metadata.borrow();

        if let Some(PreambleItem::Class { name, options }) =
            self.preamble.items().find(|i| i.is_class())
        {
            self.command_with(
                "documentclass",
                name,
                &options.iter().map(String::as_str).collect::<Vec<&str>>(),
            )?;
        } else {
            self.command("documentclass", "article")?;
        }
        self.end_lines(2)?;

        for item in self.preamble.items() {
            match item {
                PreambleItem::Package { name, options } => {
                    self.command_with(
                        "usepackage",
                        name,
                        &options.iter().map(String::as_str).collect::<Vec<&str>>(),
                    )?;
                    self.end_line()?;
                }
                PreambleItem::Command {
                    renew,
                    name,
                    define,
                    arg_count,
                } => {
                    let command = if *renew { "renewcommand" } else { "newcommand" };
                    let name = format!("\\{}", name);
                    if *arg_count > 0 {
                        self.command_with(command, &name, &[arg_count.to_string().as_str()])?;
                    } else {
                        self.command(command, &name)?;
                    }
                    self.write(&format!("{{{}}}", define))?;
                    self.end_line()?;
                }
                PreambleItem::Environment {
                    renew,
                    name,
                    arg_count,
                    default_values,
                    before,
                    after,
                } => {
                    let command = if *renew {
                        "renewenvironment"
                    } else {
                        "newenvironment"
                    };
                    let name = format!("\\{}", name);
                    if *arg_count > 0 {
                        self.command_with(command, &name, &[arg_count.to_string().as_str()])?;
                    } else {
                        self.command(command, &name)?;
                    }
                    if !default_values.is_empty() {
                        self.write(&format!("[{}]", default_values.join(", ")))?;
                    }
                    self.end_line()?;
                    self.indent();
                    self.begin_line()?;
                    self.braced(before)?;
                    self.end_line()?;
                    self.begin_line()?;
                    self.braced(after)?;
                    self.end_line()?;
                    self.outdent();
                }
                _ => {}
            }
        }
        self.end_line()?;

        let mut title_block = false;
        if let Some(v) = md.get("title") {
            self.command("title", v.get(0).unwrap())?;
            self.end_line()?;
            title_block = true;
        }

        if let Some(v) = md.get("author") {
            self.command("author", &v.join(" \\and "))?;
            self.end_line()?;
            title_block = true;
        }

        if let Some(v) = md.get("date") {
            self.command("date", v.get(0).unwrap())?;
            self.end_line()?;
            title_block = true;
        }

        if title_block {
            self.end_line()?;
        }

        self.begin_env("document")?;
        self.end_lines(2)?;

        self.begin_line()?;
        self.just_command("maketitle")?;
        self.end_lines(2)
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a, W: Write> DocumentVisitor for LatexWriter<'a, W> {
    fn start_document(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn metadata(&self, metadatum: &Metadata) -> crate::error::Result<()> {
        let mut md = self.metadata.borrow_mut();
        match metadatum {
            Metadata::Author(v) => {
                let author = format!(
                    "{}{}{}",
                    v.name,
                    v.email
                        .as_ref()
                        .map(|s| format!("\\{}", s))
                        .unwrap_or_default(),
                    v.organization
                        .as_ref()
                        .map(|s| format!("\\{}", s))
                        .unwrap_or_default()
                );
                if let Some(values) = md.get("author") {
                    let mut values = values.clone();
                    values.push(author);
                    let _ = md.insert("author".to_string(), values);
                } else {
                    let _ = md.insert("author".to_string(), vec![author]);
                }
            }
            Metadata::Copyright(_) => {
                let _ = md.insert("copyright".to_string(), vec![metadatum.value_string()]);
            }
            Metadata::Date(v) => {
                let _ = md.insert("date".to_string(), vec![v.to_string()]);
            }
            Metadata::Keywords(v) => {
                if let Some(keywords) = md.get("keywords") {
                    let mut keywords = keywords.clone();
                    keywords.extend(v.clone());
                    let _ = md.insert("keywords".to_string(), keywords);
                } else {
                    let _ = md.insert("keywords".to_string(), v.clone());
                }
            }
            Metadata::Revision(v) => {
                let _ = md.insert("revision".to_string(), vec![v.to_string()]);
            }
            Metadata::Status(v) => {
                let _ = md.insert("status".to_string(), vec![v.to_string()]);
            }
            Metadata::Title(v) => {
                let _ = md.insert("title".to_string(), vec![v.to_string()]);
            }
            Metadata::Other(v) => {
                if let Some(values) = md.get(&v.key) {
                    let mut values = values.clone();
                    values.push(v.value.clone());
                    let _ = md.insert(v.key.clone(), values);
                } else {
                    let _ = md.insert(v.key.clone(), vec![v.value.clone()]);
                }
            }
        }
        Ok(())
    }

    fn block_visitor(&self) -> Option<&dyn BlockVisitor> {
        let _ = self.preamble();
        Some(self)
    }

    fn end_document(&self) -> crate::error::Result<()> {
        self.end_env("document")
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a, W: Write> BlockVisitor for LatexWriter<'a, W> {
    fn start_block(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn comment(&self, value: &str) -> crate::error::Result<()> {
        for line in value.split('\n') {
            self.begin_line()?;
            self.write(&format!("%% {}", line))?;
            self.end_line()?;
        }
        self.end_line()
    }

    fn start_heading(
        &self,
        level: &HeadingLevel,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        self.begin_line()?;
        self.write_label(&label)?;
        self.begin_cmd(&format!(
            "{}section",
            string_of_strings("sub", (level.clone() as usize) - 1)
        ))
    }

    fn end_heading(&self, _: &HeadingLevel, _: &Option<Label>) -> crate::error::Result<()> {
        self.end_cmd()?;
        self.end_lines(2)
    }

    fn image(
        &self,
        value: &Image,
        caption: &Option<Caption>,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        self.begin_line()?;
        self.begin_env_with("figure", &["h!bt"])?;
        self.end_line()?;
        self.begin_line()?;
        self.just_command("centering")?;
        self.end_line()?;

        let inline_visitor = BlockVisitor::inline_visitor(self).unwrap();
        inline_visitor.image(value)?;

        self.end_line()?;
        if let Some(caption) = caption {
            self.begin_line()?;
            self.command("caption", caption)?;
            self.end_line()?;
        }
        if let Some(label) = label {
            self.begin_line()?;
            self.command("label", label)?;
            self.end_line()?;
        }
        self.end_env("figure")?;
        self.end_lines(2)
    }

    fn math(
        &self,
        value: &Math,
        _caption: &Option<Caption>,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        self.begin_line()?;
        self.write_label(&label)?;
        self.begin_env("equation")?;
        self.end_line()?;
        self.write(value)?;
        if !value.ends_with('\n') {
            self.write("\n")?;
        }
        self.end_env("equation")?;
        self.end_lines(2)
    }

    fn start_list(&self, kind: &ListKind, label: &Option<Label>) -> crate::error::Result<()> {
        self.begin_line()?;
        self.write_label(&label)?;
        self.begin_env(match kind {
            ListKind::Ordered => "enumerate",
            ListKind::Unordered => "itemize",
        })?;
        self.end_line()
    }

    fn end_list(&self, kind: &ListKind, _: &Option<Label>) -> crate::error::Result<()> {
        self.end_env(match kind {
            ListKind::Ordered => "enumerate",
            ListKind::Unordered => "itemize",
        })?;
        self.end_lines(if *self.indent.borrow() == 1 { 2 } else { 1 })
    }

    fn start_list_item(&self, label: &Option<Label>) -> crate::error::Result<()> {
        self.begin_line()?;
        self.just_command("item")?;
        self.write_label(&label)?;
        self.write(" ")
    }

    fn end_list_item(&self, _: &Option<Label>) -> crate::error::Result<()> {
        self.end_line()
    }

    fn start_definition_list(&self, label: &Option<Label>) -> crate::error::Result<()> {
        self.begin_line()?;
        self.write_label(&label)?;
        self.begin_env("description")?;
        self.end_line()
    }

    fn end_definition_list(&self, _: &Option<Label>) -> crate::error::Result<()> {
        self.end_env("description")?;
        self.end_lines(if *self.indent.borrow() == 1 { 2 } else { 1 })
    }

    fn start_definition(&self, term: &str, label: &Option<Label>) -> crate::error::Result<()> {
        self.begin_line()?;
        self.just_command("item")?;
        self.write_label(&label)?;
        self.write(&format!(" [{}] ", term))
    }

    fn start_definition_list_text(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn end_definition_list_text(&self) -> crate::error::Result<()> {
        self.end_line()
    }

    fn formatted(&self, value: &str, label: &Option<Label>) -> crate::error::Result<()> {
        self.begin_line()?;
        self.write_label(&label)?;
        self.begin_env("verbatim")?;
        self.end_line()?;
        self.write(value)?;
        if !value.ends_with('\n') {
            self.write("\n")?;
        }
        self.end_env("verbatim")?;
        self.end_lines(2)
    }

    fn code_block(
        &self,
        code: &str,
        language: &Option<String>,
        caption: &Option<Caption>,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        self.begin_line()?;
        let mut options: Vec<String> = Vec::new();
        if let Some(language) = language {
            options.push(format!("language={}", language));
        }
        if let Some(caption) = caption {
            options.push(format!("caption={}", caption.to_string()));
        }
        if let Some(label) = label {
            options.push(format!("label={}", label.to_string()));
        }
        self.begin_env_with(
            "lstlisting",
            &options.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
        )?;
        self.end_line()?;
        self.write(code)?;
        if !code.ends_with('\n') {
            self.write("\n")?;
        }
        self.end_env("lstlisting")?;
        self.end_lines(2)
    }

    fn start_paragraph(
        &self,
        _alignment: &Alignment,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        self.begin_line()?;
        self.write_label(&label)
    }

    fn end_paragraph(&self, _alignment: &Alignment, _: &Option<Label>) -> crate::error::Result<()> {
        self.end_lines(2)
    }

    fn start_quote(&self, label: &Option<Label>) -> crate::error::Result<()> {
        self.begin_line()?;
        self.write_label(&label)?;
        self.begin_env("displayquote")?;
        self.end_line()
    }

    fn end_quote(&self, _: &Option<Label>) -> crate::error::Result<()> {
        self.end_env("displayquote")?;
        self.end_lines(2)
    }

    fn thematic_break(&self) -> crate::error::Result<()> {
        self.begin_line()?;
        self.just_command("thematicbreak")?;
        self.end_lines(2)
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

// ------------------------------------------------------------------------------------------------

impl<'a, W: Write> TableVisitor for LatexWriter<'a, W> {
    fn start_table(&self, _: &Option<Caption>, _: &Option<Label>) -> crate::error::Result<()> {
        self.begin_line()?;
        self.begin_env_with("table", &["h!bt"])?;
        self.end_line()?;
        self.begin_line()?;
        self.just_command("centering")?;
        self.end_line()
    }

    fn start_table_header_row(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn table_header_cell(&self, column_cell: &Column, _: usize) -> crate::error::Result<()> {
        self.table_head.borrow_mut().push(column_cell.clone());
        Ok(())
    }

    fn end_table_header_row(&self) -> crate::error::Result<()> {
        let table_head = self.table_head.borrow();
        let col_spec: Vec<&str> = table_head
            .iter()
            .map(|c| match c.alignment() {
                Alignment::Left | Alignment::Justified => "l",
                Alignment::Right => "r",
                Alignment::Centered => "c",
            })
            .collect();
        self.begin_line()?;
        self.begin_env("tabular")?;
        self.braced(&format!("| {} |", col_spec.join(" | ")))?;
        self.end_line()?;
        self.begin_line()?;
        self.just_command("hline")?;
        self.end_line()?;
        self.begin_line()?;
        self.write(&format!(
            "{} \\\\",
            table_head
                .iter()
                .map(|c| c.text().as_str())
                .collect::<Vec<&str>>()
                .join(" & ")
        ))?;
        self.end_line()?;
        self.begin_line()?;
        self.just_command("hline")?;
        self.just_command("hline")?;
        self.end_line()
    }

    fn start_table_row(&self, _: usize) -> crate::error::Result<()> {
        self.begin_line()
    }

    fn start_table_cell(
        &self,
        column_idx: usize,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        self.write_label(&label)?;
        if column_idx > 0 {
            self.write(" & ")?;
        }
        Ok(())
    }

    fn end_table_row(&self, _: usize) -> crate::error::Result<()> {
        self.write(" \\\\")?;
        self.end_line()
    }

    fn end_table(
        &self,
        caption: &Option<Caption>,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        self.begin_line()?;
        self.just_command("hline")?;
        self.end_line()?;
        self.end_env("tabular")?;
        self.end_line()?;
        if let Some(caption) = caption {
            self.begin_line()?;
            self.command("caption", caption)?;
            self.end_line()?;
        }
        if let Some(label) = label {
            self.begin_line()?;
            self.command("label", label)?;
            self.end_line()?;
        }
        self.end_env("table")?;
        self.end_lines(2)
    }

    fn inline_visitor(&self) -> Option<&dyn InlineVisitor> {
        Some(self)
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a, W: Write> InlineVisitor for LatexWriter<'a, W> {
    fn link(&self, value: &HyperLink) -> crate::error::Result<()> {
        match value.target() {
            HyperLinkTarget::External(v) => {
                if let Some(alt_text) = value.caption() {
                    self.command("href", v)?;
                    self.braced(alt_text)?;
                } else {
                    self.command("url", v)?;
                }
            }
            HyperLinkTarget::Internal(v) => {
                if let Some(alt_text) = value.caption() {
                    self.command_with("hyperref", alt_text, &[&v])?;
                } else {
                    self.command("ref", v)?;
                }
                self.command("ref", v)?;
            }
        }
        Ok(())
    }

    fn image(&self, value: &Image) -> crate::error::Result<()> {
        self.command(
            "includegraphics",
            &match value.inner().target() {
                HyperLinkTarget::External(v) => v.to_string(),
                HyperLinkTarget::Internal(v) => v.to_string(),
            },
        )
    }

    fn text(&self, value: &Text) -> crate::error::Result<()> {
        self.write(value)
    }

    fn math(&self, value: &Math) -> crate::error::Result<()> {
        self.write(&format!("\\({}\\)", value.to_string()))
    }

    fn character(&self, value: &Character) -> crate::error::Result<()> {
        self.write(&match value {
            Character::Space => " ".to_string(),
            Character::NonBreakSpace => "~".to_string(),
            Character::Hyphen => "-".to_string(),
            Character::EmDash => "--".to_string(),
            Character::EnDash => "---".to_string(),
            Character::Emoji(e) => format!("\\texttt{{{}}}", e),
            Character::Other(c) => format!("{}", c),
        })
    }

    fn line_break(&self) -> crate::error::Result<()> {
        self.just_command("newline")?;
        self.write(" ")
    }

    fn start_span(&self, styles: &[SpanStyle]) -> crate::error::Result<()> {
        let mut style_stack: Vec<&str> = Vec::new();
        for style in styles {
            match style {
                SpanStyle::Plain => {
                    style_stack.clear();
                }
                SpanStyle::Italic => {
                    style_stack.push("textit");
                }
                SpanStyle::Bold => {
                    style_stack.push("textbf");
                }
                SpanStyle::Mono => {
                    style_stack.push("texttt");
                }
                SpanStyle::Code => {
                    style_stack.push("texttt");
                }
                SpanStyle::Strikethrough => {
                    style_stack.push("sout");
                }
                SpanStyle::Underline => {
                    style_stack.push("underline");
                }
                SpanStyle::SmallCaps => {
                    style_stack.push("textsc");
                }
                SpanStyle::Superscript => {
                    style_stack.push("textsuperscript");
                }
                SpanStyle::Subscript => {
                    style_stack.push("textsubscript");
                }
                SpanStyle::Sized(s) => match s {
                    Size::Largest => {
                        style_stack.push("LARGE");
                    }
                    Size::Larger => {
                        style_stack.push("Large");
                    }
                    Size::Large => {
                        style_stack.push("large");
                    }
                    Size::Normal => {
                        style_stack.push("normalsize");
                    }
                    Size::Small => {
                        style_stack.push("small");
                    }
                    Size::Smaller => {
                        style_stack.push("footnotesize");
                    }
                    Size::Smallest => {
                        style_stack.push("scriptsize");
                    }
                },
            }
        }
        for style_cmd in style_stack {
            self.begin_cmd(style_cmd)?;
        }
        Ok(())
    }

    fn end_span(&self, styles: &[SpanStyle]) -> crate::error::Result<()> {
        self.write(&string_of_strings(
            "}",
            styles
                .iter()
                .filter(|style| **style != SpanStyle::Plain)
                .count(),
        ))
    }
}
