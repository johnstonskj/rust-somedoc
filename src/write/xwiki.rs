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

use crate::model::block::quote::Quote;
use crate::model::block::{
    BlockContent, CodeBlock, DefinitionList, DefinitionListItem, Formatted, Heading, HeadingLevel,
    List, ListItem, Paragraph, Table,
};
use crate::model::inline::{
    Character, HyperLink, HyperLinkTarget, Image, InlineContent, Span, SpanStyle,
};
use crate::model::{Document, HasInnerContent, HasStyles};
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct XWikiWriter<'a, W: Write> {
    block_quoted: u8,
    w: &'a mut W,
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
pub fn writer<W: Write>(doc: &Document, w: &mut W) -> std::io::Result<()> {
    info!("xwiki::writer(.., ..)");
    let mut writer = XWikiWriter::new(w);
    write_document(&mut writer, doc)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<'a, W: Write> XWikiWriter<'a, W> {
    pub fn new(w: &'a mut W) -> Self {
        Self { block_quoted: 0, w }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn write_document<W: Write>(w: &mut XWikiWriter<W>, content: &Document) -> std::io::Result<()> {
    debug!("xwiki::write_document({:?})", content);

    write_blocks(w, content.inner())
}

fn write_blocks<W: Write>(
    w: &mut XWikiWriter<W>,
    content: &Vec<BlockContent>,
) -> std::io::Result<()> {
    debug!("xwiki::write_blocks({:?})", content);
    let count = content.len();
    for (idx, item) in content.iter().enumerate() {
        write_quote_prefix(w)?;
        match item {
            BlockContent::Comment(_) => Ok(()),
            BlockContent::Heading(content) => write_heading(w, content),
            BlockContent::Image(content) => write_image(w, content, false),
            BlockContent::List(content) => write_list(w, content, 0),
            BlockContent::DefinitionList(content) => write_definition_list(w, content),
            BlockContent::Formatted(content) => write_formatted(w, content),
            BlockContent::CodeBlock(content) => write_code_block(w, content),
            BlockContent::Paragraph(content) => write_paragraph(w, content),
            BlockContent::Quote(content) => write_quote(w, content),
            BlockContent::Table(content) => write_table(w, content),
            BlockContent::ThematicBreak => writeln!(w.w, "-----\n"),
        }?;
        if idx < count - 1 {
            write_quote_prefix(w)?;
        }
    }
    Ok(())
}

fn write_heading<W: Write>(w: &mut XWikiWriter<W>, content: &Heading) -> std::io::Result<()> {
    debug!("xwiki::write_heading({:?})", content);
    let depth = content.level_as_u8();
    if depth >= HeadingLevel::Section as u8 {
        for _ in 0..depth {
            write!(w.w, "=")?;
        }
        write!(w.w, " ")?;
        write_inlines(w, content.inner())?;
        write!(w.w, " ")?;
        for _ in 0..depth {
            write!(w.w, "=")?;
        }
        writeln!(w.w)?;
        writeln!(w.w)?;
    }
    Ok(())
}

fn write_image<W: Write>(
    w: &mut XWikiWriter<W>,
    content: &Image,
    inline: bool,
) -> std::io::Result<()> {
    debug!("xwiki::write_image({:?})", content);
    write!(w.w, "image:")?;
    write_link(w, content.link())?;
    if !inline {
        writeln!(w.w)?;
        writeln!(w.w)?;
    }
    Ok(())
}

fn write_list<W: Write>(
    w: &mut XWikiWriter<W>,
    content: &List,
    indent: usize,
) -> std::io::Result<()> {
    debug!("xwiki::write_list({:?}, {})", content, indent);
    for item in content.inner() {
        match item {
            ListItem::List(sub_list) => {
                write_list(w, sub_list, indent + 1)?;
            }
            ListItem::Item(item) => {
                write_quote_prefix(w)?;
                if content.is_ordered() {
                    for _ in 0..indent + 1 {
                        write!(w.w, "1")?;
                    }
                    write!(w.w, ".")?;
                } else {
                    for _ in 0..indent + 1 {
                        write!(w.w, "*")?;
                    }
                }
                write!(w.w, " ")?;
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
    w: &mut XWikiWriter<W>,
    content: &DefinitionList,
) -> std::io::Result<()> {
    debug!("xwiki::write_definition_list({:?})", content);
    for item in content.inner() {
        match item {
            DefinitionListItem::List(sub_list) => {
                write_definition_list(w, sub_list)?;
            }
            DefinitionListItem::Item(item) => {
                write_quote_prefix(w)?;

                write!(w.w, "; ")?;
                write_inlines(w, item.term().inner())?;
                writeln!(w.w)?;

                write!(w.w, "; ")?;
                write_inlines(w, item.text().inner())?;
                writeln!(w.w)?;
            }
        }
    }
    Ok(())
}

fn write_quote_prefix<W: Write>(w: &mut XWikiWriter<W>) -> std::io::Result<()> {
    for _ in 0..w.block_quoted {
        write!(w.w, ">")?;
    }
    if w.block_quoted > 0 {
        write!(w.w, " ")?;
    }
    Ok(())
}

fn write_paragraph<W: Write>(w: &mut XWikiWriter<W>, content: &Paragraph) -> std::io::Result<()> {
    debug!("xwiki::write_paragraph({:?})", content);
    write_inlines(w, content.inner())?;
    writeln!(w.w)?;
    writeln!(w.w)
}

fn write_quote<W: Write>(w: &mut XWikiWriter<W>, content: &Quote) -> std::io::Result<()> {
    debug!("xwiki::write_quote({:?})", content);
    w.block_quoted = w.block_quoted + 1;
    write_blocks(w, content.inner())?;
    w.block_quoted = w.block_quoted - 1;
    Ok(())
}

fn write_table<W: Write>(w: &mut XWikiWriter<W>, content: &Table) -> std::io::Result<()> {
    debug!("xwiki::write_table({:?})", content);
    if !content.columns().is_empty() {
        writeln!(w.w, "(% border=\"1\" %)")?;
        write_quote_prefix(w)?;
        for column in content.columns() {
            write!(w.w, "|={}", column.label())?;
        }
        writeln!(w.w)?;

        for row in content.rows() {
            write_quote_prefix(w)?;
            for cell in row.cells() {
                write!(w.w, "|")?;
                if cell.has_inner() {
                    write_inlines(w, cell.inner())?;
                }
            }
            writeln!(w.w)?;
        }
    }
    writeln!(w.w)
}

fn write_formatted<W: Write>(w: &mut XWikiWriter<W>, content: &Formatted) -> std::io::Result<()> {
    debug!("xwiki::write_formatted({:?})", content);
    writeln!(w.w, "{{{{{{")?;
    writeln!(w.w, "{}", content.inner())?;
    writeln!(w.w, "}}}}}}")?;
    writeln!(w.w)
}

fn write_code_block<W: Write>(w: &mut XWikiWriter<W>, content: &CodeBlock) -> std::io::Result<()> {
    debug!("xwiki::write_code_block({:?})", content);
    write!(w.w, "{{{{code")?;
    if let Some(language) = content.language() {
        write!(w.w, " language=\"{}\"", language)?;
    }
    writeln!(w.w, "}}}}")?;
    writeln!(w.w)?;
    writeln!(w.w, "{}", content.code())?;
    writeln!(w.w, "{{{{/code}}}}")?;
    writeln!(w.w)
}

fn write_inlines<W: Write>(
    w: &mut XWikiWriter<W>,
    content: &Vec<InlineContent>,
) -> std::io::Result<()> {
    debug!("xwiki::write_inlines({:?})", content);
    for item in content {
        match item {
            InlineContent::HyperLink(value) => write_link(w, value)?,
            InlineContent::Anchor(value) => write!(w.w, "(% id=\"{}\" %)", value.inner())?,
            InlineContent::Image(image) => write_image(w, image, true)?,
            InlineContent::Text(value) => write!(w.w, "{}", value.inner())?,
            InlineContent::Character(value) => write_character(w, value)?,
            InlineContent::LineBreak => {
                writeln!(w.w, "\\")?;
            }
            InlineContent::Span(value) => write_span(w, value)?,
        }
    }
    Ok(())
}

fn write_span<W: Write>(w: &mut XWikiWriter<W>, content: &Span) -> std::io::Result<()> {
    let mut style_stack = Vec::new();
    for style in content.styles() {
        let delim: &str = match style {
            SpanStyle::Plain => {
                style_stack.clear();
                ""
            }
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
            write!(w.w, "{}", delim)?;
        }
        style_stack.push(delim);
    }
    write_inlines(w, content.inner())?;
    for delim in style_stack.iter().rev() {
        if !delim.is_empty() {
            write!(w.w, "{}", delim)?;
        }
    }
    Ok(())
}

fn write_character<W: Write>(w: &mut XWikiWriter<W>, content: &Character) -> std::io::Result<()> {
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

fn write_link<W: Write>(w: &mut XWikiWriter<W>, content: &HyperLink) -> std::io::Result<()> {
    write!(w.w, "[[")?;
    if let Some(alt_text) = content.alt_text() {
        write_inlines(w, alt_text.inner())?;
        write!(w.w, ">>")?;
    }
    match content.target() {
        HyperLinkTarget::External(value) => write!(w.w, "{}]]", value)?,
        HyperLinkTarget::Internal(value) => write!(
            w.w,
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
