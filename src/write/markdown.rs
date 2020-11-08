/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error;
use crate::model::block::quote::Quote;
use crate::model::block::table::Alignment;
use crate::model::block::{
    BlockContent, CodeBlock, DefinitionList, DefinitionListItem, Formatted, Heading, HeadingKind,
    List, ListItem, Paragraph, Table,
};
use crate::model::inline::{
    Character, HyperLink, HyperLinkTarget, Image, InlineContent, Span, TextStyle,
};
use crate::model::{ComplexContent, Document, Styled};
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

#[derive(Debug)]
enum MetadataFlavor {
    #[allow(dead_code)]
    PercentComment,
    #[allow(dead_code)]
    FencedYaml(String),
    Yaml,
}

#[derive(Debug)]
struct Features {
    has_tables: bool,
    has_definition_lists: bool,
    has_fenced_code_blocks: bool,
    has_code_syntax: bool,
    has_strikethrough: bool,
    metadata: Option<MetadataFlavor>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn writer<E, W: Write>(
    doc: &Document,
    flavor: MarkdownFlavor,
    w: &mut W,
) -> std::io::Result<()> {
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

const THEMATIC_BREAK: &str = "---";

const TABLE_PIPE: &str = "|";
const TABLE_BAR: &str = "---";
const TABLE_ALIGN: &str = ":";

#[allow(dead_code)]
const YAML_FENCE: &str = "-----";

const STRICT_FEATURES: Features = Features {
    has_tables: false,
    has_definition_lists: false,
    has_fenced_code_blocks: false,
    has_code_syntax: false,
    has_strikethrough: false,
    metadata: None,
};

const COMMONMARK_FEATURES: Features = Features {
    has_tables: false,
    has_definition_lists: false,
    has_fenced_code_blocks: true,
    has_code_syntax: true,
    has_strikethrough: false,
    metadata: None,
};

const GFM_FEATURES: Features = Features {
    has_tables: true,
    has_definition_lists: false,
    has_fenced_code_blocks: true,
    has_code_syntax: true,
    has_strikethrough: true,
    metadata: None,
};

const MMD_FEATURES: Features = Features {
    has_tables: true,
    has_definition_lists: true,
    has_fenced_code_blocks: true,
    has_code_syntax: true,
    has_strikethrough: false,
    metadata: Some(MetadataFlavor::Yaml),
};

const PHP_EXTRA_FEATURES: Features = Features {
    has_tables: true,
    has_definition_lists: true,
    has_fenced_code_blocks: true,
    has_code_syntax: false,
    has_strikethrough: false,
    metadata: None,
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

// type WriteFn<T, W: Write> = dyn FnMut(&mut MarkdownWriter<W>, &T) -> std::io::Result<()>;

fn write_document<W: Write>(w: &mut MarkdownWriter<W>, content: &Document) -> std::io::Result<()> {
    debug!("markdown::write_document({:?})", content);

    if content.has_metadata() && w.features.metadata.is_some() {
        let (prefix, fence) = match &w.features.metadata.as_ref().unwrap() {
            MetadataFlavor::PercentComment => (Some("%"), None),
            MetadataFlavor::FencedYaml(fence) => (None, Some(fence)),
            MetadataFlavor::Yaml => (None, None),
        };
        if let Some(fence) = fence {
            writeln!(w.w, "{}", fence)?;
        }
        for datum in content.metadata() {
            if let Some(prefix) = prefix {
                writeln!(
                    w.w,
                    "{} {}: {}",
                    prefix,
                    datum.kind(),
                    match datum.value() {
                        None => "",
                        Some(s) => s,
                    }
                )?;
            }
        }
        if let Some(fence) = fence {
            writeln!(w.w, "{}", fence)?;
        }
        writeln!(w.w)?;
    }

    write_blocks(w, content.inner())
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
    let depth = match content.kind() {
        HeadingKind::Title => 1,
        HeadingKind::Subtitle => 2,
        HeadingKind::Chapter => 3,
        HeadingKind::Heading(d) => *d,
    };
    for _ in 0..depth {
        write!(w.w, "{}", HEADER)?;
    }
    write!(w.w, " ")?;
    write_inlines(w, content.inner())?;
    writeln!(w.w)?;
    writeln!(w.w)
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
            TextStyle::Plain => "",
            TextStyle::Italic | TextStyle::Slanted | TextStyle::Quote => SPAN_ITALIC,
            TextStyle::Light => "",
            TextStyle::Bold => SPAN_BOLD,
            TextStyle::Mono | TextStyle::Code => "`",
            TextStyle::Strikethrough => {
                if w.features.has_strikethrough {
                    SPAN_STRIKETHROUGH
                } else {
                    ""
                }
            }
            TextStyle::Underline => "",
            TextStyle::SmallCaps => "",
            TextStyle::Superscript => "",
            TextStyle::Subscript => "",
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
            Character::Emoji(name) => format!(":{}:", name.name()),
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

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
