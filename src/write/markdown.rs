/*!
Write the document in one of the well-defined flavors of Markup.

# Flavors

* **[Strict](https://daringfireball.net/projects/markdown/syntax)**; the original
* **[CommonMark](https://spec.commonmark.org/0.29/)**; . This is the default flavor.
* **[GitHub](https://github.github.com/gfm/)**;
* **[Multi](https://rawgit.com/fletcher/MultiMarkdown-6-Syntax-Guide/master/index.html)**;
* **[PhpExtra](https://michelf.ca/projects/php-markdown/extra/)**;

# Example

```rust
# use somedoc::model::Document;
use somedoc::write::OutputFormat;
use somedoc::write::markdown::{writer, MarkdownFlavor};

# fn make_some_document() -> Document { Document::default() }
let doc = make_some_document();

writer(&doc, MarkdownFlavor::GitHub, &mut std::io::stdout()).unwrap();
```
*/

use crate::error;
use crate::model::block::quote::Quote;
use crate::model::block::table::Alignment;
use crate::model::block::{
    BlockContent, CodeBlock, DefinitionList, DefinitionListItem, Formatted, Heading, HeadingLevel,
    List, ListItem, Paragraph, Table,
};
use crate::model::document::Metadata;
use crate::model::inline::{
    Character, HyperLink, HyperLinkTarget, Image, InlineContent, Span, SpanStyle,
};
use crate::model::{Document, HasInnerContent, HasStyles};
use crate::write::OutputFormat;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum MarkdownFlavor {
    /// See <https://daringfireball.net/projects/markdown/syntax>
    /// and <https://www.markdownguide.org/basic-syntax/>
    Strict,

    /// See <https://spec.commonmark.org/0.29/>
    CommonMark,

    /// See <https://github.github.com/gfm/>
    GitHub,

    /// See <https://rawgit.com/fletcher/MultiMarkdown-6-Syntax-Guide/master/index.html>
    Multi,

    /// See <https://michelf.ca/projects/php-markdown/extra/>
    PhpExtra,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct MarkdownWriter<'a, W: Write> {
    features: &'static Features,
    block_quoted: u8,
    w: &'a mut W,
}

#[derive(Clone, Debug, PartialEq)]
enum MetadataFlavor {
    HiddenLinks,
    #[allow(dead_code)]
    PercentComment,
    #[allow(dead_code)]
    FencedYaml,
    Yaml,
}

#[derive(Debug)]
struct Features {
    has_tables: bool,
    has_definition_lists: bool,
    has_fenced_code_blocks: bool,
    has_code_syntax: bool,
    has_strikethrough: bool,
    metadata: MetadataFlavor,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Implementation of the writer function for a set of Markdown flavors.
///
/// While this can be called directly it is most often used  by calling either
/// [`model::write_document`](../fn.write_document.html) or
/// [`model::write_document_to_string`](../fn.write_document_to_string.html).
///
pub fn writer<W: Write>(doc: &Document, flavor: MarkdownFlavor, w: &mut W) -> std::io::Result<()> {
    info!("markdown::writer(.., {}, ..)", flavor);
    let mut writer = MarkdownWriter::new(flavor, w);
    write_document(&mut writer, doc)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const CODE_INDENT: &str = "    ";
const CODE_FENCE: &str = "```";

const DEFN_TEXT_PREFIX: &str = ":";

const HEADER: &str = "#";

const LIST_ORDERED_PREFIX: &str = "1.";
const LIST_ORDERED_INDENT: &str = "   ";

const LIST_UNORDERED_PREFIX: &str = "*";
const LIST_UNORDERED_INDENT: &str = "  ";

const QUOTE_PREFIX: &str = "> ";

const SPAN_BOLD: &str = "**";
const SPAN_ITALIC: &str = "*";
const SPAN_STRIKETHROUGH: &str = "~~";

const THEMATIC_BREAK: &str = "-----";

const TABLE_PIPE: &str = "|";
const TABLE_BAR: &str = "---";
const TABLE_ALIGN: &str = ":";

const YAML_FENCE: &str = "-----";

const STRICT_FEATURES: Features = Features {
    has_tables: false,
    has_definition_lists: false,
    has_fenced_code_blocks: false,
    has_code_syntax: false,
    has_strikethrough: false,
    metadata: MetadataFlavor::HiddenLinks,
};

const COMMONMARK_FEATURES: Features = Features {
    has_tables: false,
    has_definition_lists: false,
    has_fenced_code_blocks: true,
    has_code_syntax: true,
    has_strikethrough: false,
    metadata: MetadataFlavor::HiddenLinks,
};

const GFM_FEATURES: Features = Features {
    has_tables: true,
    has_definition_lists: false,
    has_fenced_code_blocks: true,
    has_code_syntax: true,
    has_strikethrough: true,
    metadata: MetadataFlavor::HiddenLinks,
};

const MMD_FEATURES: Features = Features {
    has_tables: true,
    has_definition_lists: true,
    has_fenced_code_blocks: true,
    has_code_syntax: true,
    has_strikethrough: false,
    metadata: MetadataFlavor::Yaml,
};

const PHP_EXTRA_FEATURES: Features = Features {
    has_tables: true,
    has_definition_lists: true,
    has_fenced_code_blocks: true,
    has_code_syntax: false,
    has_strikethrough: false,
    metadata: MetadataFlavor::HiddenLinks,
};

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
                MarkdownFlavor::Strict => "strict",
                MarkdownFlavor::CommonMark => "commonmark",
                MarkdownFlavor::GitHub => "gfm",
                MarkdownFlavor::Multi => "multi",
                MarkdownFlavor::PhpExtra => "php_extra",
            }
        )
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
            _ => Err(error::ErrorKind::UnknownFormat.into()),
        }
    }
}

impl Into<OutputFormat> for MarkdownFlavor {
    fn into(self) -> OutputFormat {
        OutputFormat::Markdown(self)
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a, W: Write> MarkdownWriter<'a, W> {
    pub fn new(flavor: MarkdownFlavor, w: &'a mut W) -> Self {
        Self {
            features: match flavor {
                MarkdownFlavor::Strict => &STRICT_FEATURES,
                MarkdownFlavor::CommonMark => &COMMONMARK_FEATURES,
                MarkdownFlavor::GitHub => &GFM_FEATURES,
                MarkdownFlavor::Multi => &MMD_FEATURES,
                MarkdownFlavor::PhpExtra => &PHP_EXTRA_FEATURES,
            },
            block_quoted: 0,
            w,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn write_document<W: Write>(w: &mut MarkdownWriter<W>, content: &Document) -> std::io::Result<()> {
    debug!("markdown::write_document({:?})", content);

    if content.has_metadata() {
        write_metadata(w, content.metadata())?;
        writeln!(w.w)?;
    }

    write_blocks(w, content.inner())
}

fn write_metadata<W: Write>(
    w: &mut MarkdownWriter<W>,
    metadata: &Vec<Metadata>,
) -> std::io::Result<()> {
    if w.features.metadata == MetadataFlavor::FencedYaml {
        writeln!(w.w, "{}", YAML_FENCE)?;
    }
    for datum in metadata {
        write_metadatum(w, datum)?;
    }
    if w.features.metadata == MetadataFlavor::FencedYaml {
        writeln!(w.w, "{}", YAML_FENCE)?;
    }
    Ok(())
}

fn write_metadatum<W: Write>(w: &mut MarkdownWriter<W>, datum: &Metadata) -> std::io::Result<()> {
    fn write_as_link<W: Write>(w: &mut W, k: &str, v: &str) -> std::io::Result<()> {
        writeln!(w, "[_metadata_:{}]:- \"{}\"", k, v)
    }

    fn write_as_prop<W: Write>(w: &mut W, k: &str, v: &str, pct: bool) -> std::io::Result<()> {
        writeln!(w, "{}{}: {}", if pct { "% " } else { "" }, k, v)
    }

    match datum {
        Metadata::Author(value) => match &w.features.metadata {
            MetadataFlavor::HiddenLinks => {
                write_as_link(
                    w.w,
                    "author",
                    &format!(
                        "{}{}{}",
                        value.name,
                        value
                            .email
                            .as_ref()
                            .map(|s| format!("({})", s))
                            .unwrap_or_default(),
                        value
                            .organization
                            .as_ref()
                            .map(|s| format!(" - {}", s))
                            .unwrap_or_default()
                    ),
                )?;
            }
            MetadataFlavor::PercentComment => {
                writeln!(
                    w.w,
                    "% author: {}{}{}",
                    value.name,
                    value
                        .email
                        .as_ref()
                        .map(|s| format!("({})", s))
                        .unwrap_or_default(),
                    value
                        .organization
                        .as_ref()
                        .map(|s| format!(" - {}", s))
                        .unwrap_or_default()
                )?;
            }
            MetadataFlavor::FencedYaml | MetadataFlavor::Yaml => {
                writeln!(w.w, "author:")?;
                writeln!(w.w, "- name: {}", value.name)?;
                if let Some(email) = &value.email {
                    writeln!(w.w, "  email: {}", &email)?;
                }
                if let Some(organization) = &value.organization {
                    writeln!(w.w, "  organization: {}", &organization)?;
                }
            }
        },
        Metadata::Class(value) => {
            if w.features.metadata == MetadataFlavor::HiddenLinks {
                write_as_link(w.w, "css", &value.name_or_path)?;
            } else {
                write_as_prop(
                    w.w,
                    "css",
                    &value.name_or_path,
                    w.features.metadata == MetadataFlavor::PercentComment,
                )?;
            }
        }
        Metadata::Copyright(value) => {
            let copyright = format!(
                "{}{}{}",
                value.year,
                value
                    .organization
                    .as_ref()
                    .map(|s| format!(" {}.", s))
                    .unwrap_or_default(),
                value
                    .comment
                    .as_ref()
                    .map(|s| format!(" - {}.", s))
                    .unwrap_or_default()
            );
            if w.features.metadata == MetadataFlavor::HiddenLinks {
                write_as_link(w.w, "copyright", &copyright)?;
            } else {
                write_as_prop(
                    w.w,
                    "copyright",
                    &copyright,
                    w.features.metadata == MetadataFlavor::PercentComment,
                )?;
            }
        }
        Metadata::Date(value) => {
            if w.features.metadata == MetadataFlavor::HiddenLinks {
                write_as_link(w.w, "date", value)?;
            } else {
                write_as_prop(
                    w.w,
                    "date",
                    value,
                    w.features.metadata == MetadataFlavor::PercentComment,
                )?;
            }
        }
        Metadata::Keywords(value) => {
            let keywords = format!("[{}]", value.join(", "));
            if w.features.metadata == MetadataFlavor::HiddenLinks {
                write_as_link(w.w, "keywords", &keywords)?;
            } else {
                write_as_prop(
                    w.w,
                    "keywords",
                    &keywords,
                    w.features.metadata == MetadataFlavor::PercentComment,
                )?;
            }
        }
        Metadata::Revision(value) => {
            if w.features.metadata == MetadataFlavor::HiddenLinks {
                write_as_link(w.w, "revision", value)?;
            } else {
                write_as_prop(
                    w.w,
                    "revision",
                    value,
                    w.features.metadata == MetadataFlavor::PercentComment,
                )?;
            }
        }
        Metadata::Status(value) => {
            if w.features.metadata == MetadataFlavor::HiddenLinks {
                write_as_link(w.w, "status", value)?;
            } else {
                write_as_prop(
                    w.w,
                    "status",
                    value,
                    w.features.metadata == MetadataFlavor::PercentComment,
                )?;
            }
        }
        Metadata::SubTitle(value) => {
            if w.features.metadata == MetadataFlavor::HiddenLinks {
                write_as_link(w.w, "subtitle", value)?;
            } else {
                write_as_prop(
                    w.w,
                    "subtitle",
                    value,
                    w.features.metadata == MetadataFlavor::PercentComment,
                )?;
            }
        }
        Metadata::Title(value) => {
            if w.features.metadata == MetadataFlavor::HiddenLinks {
                write_as_link(w.w, "title", value)?;
            } else {
                write_as_prop(
                    w.w,
                    "title",
                    value,
                    w.features.metadata == MetadataFlavor::PercentComment,
                )?;
            }
        }
        Metadata::Other(value) => {
            if w.features.metadata == MetadataFlavor::HiddenLinks {
                write_as_link(w.w, &value.name, &value.value)?;
            } else {
                write_as_prop(
                    w.w,
                    &value.name,
                    &value.value,
                    w.features.metadata == MetadataFlavor::PercentComment,
                )?;
            }
        }
    }

    Ok(())
}

fn write_blocks<W: Write>(
    w: &mut MarkdownWriter<W>,
    content: &Vec<BlockContent>,
) -> std::io::Result<()> {
    debug!("markdown::write_blocks({:?})", content);
    let count = content.len();
    for (idx, item) in content.iter().enumerate() {
        write_quote_prefix(w)?;
        match item {
            BlockContent::Comment(_) => Ok(()),
            BlockContent::Heading(content) => write_heading(w, content),
            BlockContent::Image(content) => write_image(w, content, false),
            BlockContent::List(content) => write_list(w, content, 0),
            BlockContent::DefinitionList(content) => {
                if w.features.has_definition_lists {
                    write_definition_list(w, content)
                } else {
                    Ok(())
                }
            }
            BlockContent::CodeBlock(content) => write_code_block(w, content),
            BlockContent::Formatted(content) => write_formatted(w, content),
            BlockContent::Paragraph(content) => write_paragraph(w, content),
            BlockContent::Quote(content) => write_quote(w, content),
            BlockContent::Table(content) => write_table(w, content),
            BlockContent::ThematicBreak => writeln!(w.w, "{}\n", THEMATIC_BREAK),
        }?;
        if idx < count - 1 {
            write_quote_prefix(w)?;
        }
    }
    Ok(())
}

fn write_heading<W: Write>(w: &mut MarkdownWriter<W>, content: &Heading) -> std::io::Result<()> {
    debug!("markdown::write_heading({:?})", content);
    let depth = content.level_as_u8();
    if depth >= HeadingLevel::Section as u8 {
        for _ in 0..depth {
            write!(w.w, "{}", HEADER)?;
        }
        write!(w.w, " ")?;
        write_inlines(w, content.inner())?;
        writeln!(w.w)?;
        writeln!(w.w)?;
    }
    Ok(())
}

fn write_image<W: Write>(
    w: &mut MarkdownWriter<W>,
    content: &Image,
    inline: bool,
) -> std::io::Result<()> {
    debug!("markdown::write_image({:?})", content);
    write!(w.w, "!")?;
    write_link(w, content.link())?;
    if !inline {
        writeln!(w.w)?;
        writeln!(w.w)?;
    }
    Ok(())
}

fn write_list<W: Write>(
    w: &mut MarkdownWriter<W>,
    content: &List,
    indent: usize,
) -> std::io::Result<()> {
    debug!("markdown::write_list({:?}, {})", content, indent);
    for item in content.inner() {
        match item {
            ListItem::List(sub_list) => {
                write_list(w, sub_list, indent + 1)?;
            }
            ListItem::Item(item) => {
                write_quote_prefix(w)?;
                if content.is_ordered() {
                    for _ in 0..indent {
                        write!(w.w, "{}", LIST_ORDERED_INDENT)?;
                    }
                    write!(w.w, "{} ", LIST_ORDERED_PREFIX)?;
                } else {
                    for _ in 0..indent {
                        write!(w.w, "{}", LIST_UNORDERED_INDENT)?;
                    }
                    write!(w.w, "{} ", LIST_UNORDERED_PREFIX)?;
                }
                write_inlines(w, item.inner())?;
                writeln!(w.w)?;
            }
        }
    }
    if indent == 0 {
        writeln!(w.w)?;
    }
    Ok(())
}

fn write_definition_list<W: Write>(
    w: &mut MarkdownWriter<W>,
    content: &DefinitionList,
) -> std::io::Result<()> {
    debug!("markdown::write_definition_list({:?})", content);
    for item in content.inner() {
        match item {
            DefinitionListItem::List(sub_list) => {
                write_definition_list(w, sub_list)?;
            }
            DefinitionListItem::Item(item) => {
                write_quote_prefix(w)?;

                write_inlines(w, item.term().inner())?;
                writeln!(w.w)?;

                write!(w.w, "{} ", DEFN_TEXT_PREFIX)?;
                write_inlines(w, item.text().inner())?;
                writeln!(w.w)?;
            }
        }
    }
    Ok(())
}

fn write_quote_prefix<W: Write>(w: &mut MarkdownWriter<W>) -> std::io::Result<()> {
    for _ in 0..w.block_quoted {
        write!(w.w, "{}", QUOTE_PREFIX)?;
    }
    Ok(())
}

fn write_paragraph<W: Write>(
    w: &mut MarkdownWriter<W>,
    content: &Paragraph,
) -> std::io::Result<()> {
    debug!("markdown::write_paragraph({:?})", content);
    write_inlines(w, content.inner())?;
    writeln!(w.w)?;
    writeln!(w.w)
}

fn write_quote<W: Write>(w: &mut MarkdownWriter<W>, content: &Quote) -> std::io::Result<()> {
    debug!("markdown::write_quote({:?})", content);
    w.block_quoted = w.block_quoted + 1;
    write_blocks(w, content.inner())?;
    w.block_quoted = w.block_quoted - 1;
    Ok(())
}

fn write_table<W: Write>(w: &mut MarkdownWriter<W>, content: &Table) -> std::io::Result<()> {
    debug!("markdown::write_table({:?})", content);
    if !content.columns().is_empty() {
        let mut sep = String::new();
        for column in content.columns() {
            write!(w.w, "{} {} ", TABLE_PIPE, column.label())?;
            sep.push_str(&match column.alignment() {
                Alignment::Default => format!("{} {}- ", TABLE_PIPE, TABLE_BAR),
                Alignment::Left => format!("{} {}{} ", TABLE_PIPE, TABLE_ALIGN, TABLE_BAR),
                Alignment::Right => format!("{} {}{} ", TABLE_PIPE, TABLE_BAR, TABLE_ALIGN),
                Alignment::Centered => format!(
                    "{} {}{}{} ",
                    TABLE_PIPE, TABLE_ALIGN, TABLE_BAR, TABLE_ALIGN
                ),
            });
        }
        writeln!(w.w, "{}", TABLE_PIPE)?;

        write_quote_prefix(w)?;
        writeln!(w.w, "{} {}", sep, TABLE_PIPE)?;

        for row in content.rows() {
            write_quote_prefix(w)?;
            for cell in row.cells() {
                if cell.has_inner() {
                    write!(w.w, "{} ", TABLE_PIPE)?;
                    write_inlines(w, cell.inner())?;
                    write!(w.w, " ")?;
                } else {
                    write!(w.w, "{}", TABLE_PIPE)?;
                }
            }
            writeln!(w.w, "{}", TABLE_PIPE)?;
        }
    }
    writeln!(w.w)
}

fn write_formatted<W: Write>(
    w: &mut MarkdownWriter<W>,
    content: &Formatted,
) -> std::io::Result<()> {
    debug!("markdown::write_formatted({:?})", content);
    for line in content.inner().split('\n') {
        writeln!(w.w, "{}{}", CODE_INDENT, line)?;
    }
    writeln!(w.w)
}

fn write_code_block<W: Write>(
    w: &mut MarkdownWriter<W>,
    content: &CodeBlock,
) -> std::io::Result<()> {
    debug!("markdown::write_code_block({:?})", content);
    if w.features.has_fenced_code_blocks {
        write!(w.w, "{}", CODE_FENCE)?;
        if w.features.has_code_syntax {
            if let Some(language) = content.language() {
                write!(w.w, "{}", language)?;
            }
        }
        writeln!(w.w)?;
        writeln!(w.w, "{}", content.code())?;
        writeln!(w.w, "{}", CODE_FENCE)?;
    } else {
        for line in content.code().split('\n') {
            writeln!(w.w, "{}{}", CODE_INDENT, line)?;
        }
    }
    writeln!(w.w)
}

fn write_inlines<W: Write>(
    w: &mut MarkdownWriter<W>,
    content: &Vec<InlineContent>,
) -> std::io::Result<()> {
    debug!("markdown::write_inlines({:?})", content);
    for item in content {
        match item {
            // TODO: all local refs need alt text.
            InlineContent::HyperLink(value) => write_link(w, value)?,
            InlineContent::Anchor(_) => {}
            InlineContent::Image(image) => write_image(w, image, true)?,
            InlineContent::Text(value) => write!(w.w, "{}", value.inner())?,
            InlineContent::Character(value) => write_character(w, value)?,
            InlineContent::LineBreak => {
                writeln!(w.w, "  ")?;
            }
            InlineContent::Span(value) => write_span(w, value)?,
        }
    }
    Ok(())
}

fn write_span<W: Write>(w: &mut MarkdownWriter<W>, content: &Span) -> std::io::Result<()> {
    let mut style_stack = Vec::new();
    for style in content.styles() {
        let delim: &str = match style {
            SpanStyle::Plain => {
                style_stack.clear();
                ""
            }
            SpanStyle::Italic | SpanStyle::Slanted => SPAN_ITALIC,
            SpanStyle::Bold => SPAN_BOLD,
            SpanStyle::Mono | SpanStyle::Code => "`",
            SpanStyle::Strikethrough => {
                if w.features.has_strikethrough {
                    SPAN_STRIKETHROUGH
                } else {
                    ""
                }
            }
            _ => "",
        };
        write!(w.w, "{}", delim)?;
        style_stack.push(delim);
    }
    write_inlines(w, content.inner())?;
    for delim in style_stack.iter().rev() {
        write!(w.w, "{}", delim)?;
    }
    Ok(())
}

fn write_character<W: Write>(
    w: &mut MarkdownWriter<W>,
    content: &Character,
) -> std::io::Result<()> {
    write!(
        w.w,
        "{}",
        match content {
            Character::Space => " ".to_string(),
            Character::NonBreakSpace => "&nbsp;".to_string(),
            Character::Hyphen => "-".to_string(),
            Character::EmDash => "---".to_string(),
            Character::EnDash => "--".to_string(),
            Character::Emoji(name) => format!(":{}:", name.inner()),
            Character::Other(c) => c.to_string(),
        }
    )
}

fn write_link<W: Write>(w: &mut MarkdownWriter<W>, content: &HyperLink) -> std::io::Result<()> {
    if let Some(alt_text) = &content.alt_text() {
        write!(w.w, "[")?;
        write_inlines(w, alt_text.inner())?;
        write!(w.w, "](")?;
    } else {
        write!(w.w, "<")?;
    }
    match content.target() {
        HyperLinkTarget::External(value) => write!(w.w, "{}", value)?,
        HyperLinkTarget::Internal(value) => write!(
            w.w,
            "#{}",
            value
                .inner()
                .to_lowercase()
                .trim()
                .replace(" ", "-")
                .replace(&['(', ')', ',', '\"', '.', ';', ':', '\''][..], "")
        )?,
    }
    if let Some(_) = content.alt_text() {
        write!(w.w, ")")?;
    } else {
        write!(w.w, ">")?;
    }
    Ok(())
}
