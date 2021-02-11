/*!
This module provides a set of traits for implementing model visitors as well as the function
`walk_document` that walks a visitor over a specific `Document`.

# Example

The following is a version of the actual function provided by the `write::xwiki` module and
shows how the writer has been implemented as a visitor.

```rust,ignore
pub fn writer<W: Write>(doc: &Document, w: &mut W) -> crate::error::Result<()> {
    info!("xwiki::writer(.., ..)");
    let writer = XWikiWriter::new(w);
    walk_document(doc, &writer)?;
    Ok(())
}
```

*/

use crate::model::block::{
    BlockContent, Caption, Captioned, Column, DefinitionList, DefinitionListItem, HeadingLevel,
    Label, List, ListItem, ListKind, ParagraphStyle, Table,
};
use crate::model::document::Metadata;
use crate::model::inline::{Character, HyperLink, Image, InlineContent, Math, SpanStyle, Text};
use crate::model::{block::HasLabel, HasInnerContent};
use crate::model::{Document, HasStyles};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The visitor trait for a `Document` instance.
///
#[allow(unused_variables)]
pub trait DocumentVisitor {
    /// Called at the start of each `Document` instance, before any inner content.
    fn start_document(&self) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called before any metadata items, and **only if** there are metadata items.
    fn start_metadata(&self) -> crate::error::Result<()> {
        Ok(())
    }

    /// Visit each `Metadata` instance.
    fn metadata(&self, metadatum: &Metadata) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called after any metadata items, and **only if** there are metadata items.
    fn end_metadata(&self) -> crate::error::Result<()> {
        Ok(())
    }

    /// Return an implementation of the `BlockVisitor` trait, if one exists.
    fn block_visitor(&self) -> Option<&dyn BlockVisitor> {
        None
    }

    /// Called at the end of each `Document` instance, after any inner content.
    fn end_document(&self) -> crate::error::Result<()> {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

///
/// The visitor trait for all block content instances.
///
#[allow(unused_variables)]
pub trait BlockVisitor {
    /// Called at the start of each `BlockContent` instance, before any value.
    fn start_block(&self) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the start of the document's abstract.
    fn start_abstract(&self) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the end of the document's abstract.
    fn end_abstract(&self) -> crate::error::Result<()> {
        Ok(())
    }

    /// Visit each `BlockContent::Comment` instance.
    fn comment(&self, value: &str) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the start of each `BlockContent::Heading` instance, before any inner content.
    fn start_heading(
        &self,
        level: &HeadingLevel,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the end of each `BlockContent::Heading` instance, after any inner content.
    fn end_heading(&self, level: &HeadingLevel, label: &Option<Label>) -> crate::error::Result<()> {
        Ok(())
    }

    /// Visit each `BlockContent::Image` instance.
    fn image(
        &self,
        value: &Image,
        caption: &Option<Caption>,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        Ok(())
    }

    /// Visit each `BlockContent::MathBlock` instance.
    fn math(
        &self,
        value: &Math,
        caption: &Option<Caption>,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the start of each `BlockContent::List` instance, before any inner content.
    fn start_list(&self, kind: &ListKind, label: &Option<Label>) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the start of each `BlockContent::List` instance, before any inner content.
    fn end_list(&self, kind: &ListKind, label: &Option<Label>) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the start of each `Item` instance, before any inner content.
    fn start_list_item(&self, label: &Option<Label>) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the end of each `Item` instance, after any inner content.
    fn end_list_item(&self, label: &Option<Label>) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the start of each `BlockContent::DefinitionList` instance, before any inner content.
    fn start_definition_list(&self, label: &Option<Label>) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the end of each `BlockContent::DefinitionList` instance, after any inner content.
    fn end_definition_list(&self, label: &Option<Label>) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the start of each `Definition` instance `term`, before any inner content.
    fn start_definition_list_term(&self, label: &Option<Label>) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the end of each `Definition` instance `term`, after any inner content.
    fn end_definition_list_term(&self, label: &Option<Label>) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the start of each `Definition` instance `text`, before any inner content.
    fn start_definition_list_text(&self) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the end of each `Definition` instance `text`, after any inner content.
    fn end_definition_list_text(&self) -> crate::error::Result<()> {
        Ok(())
    }

    /// Visit each `BlockContent::Formatted` instance.
    fn formatted(&self, value: &String, label: &Option<Label>) -> crate::error::Result<()> {
        Ok(())
    }

    /// Visit each `BlockContent::CodeBlock` instance.
    fn code_block(
        &self,
        code: &String,
        language: &Option<String>,
        caption: &Option<Caption>,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the start of each `BlockContent::Paragraph` instance, before any inner content.
    fn start_paragraph(
        &self,
        styles: &Vec<ParagraphStyle>,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the end of each `BlockContent::Paragraph` instance, after any inner content.
    fn end_paragraph(
        &self,
        styles: &Vec<ParagraphStyle>,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the start of each `BlockContent::Quote` instance, before any inner content.
    fn start_quote(&self, label: &Option<Label>) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the end of each `BlockContent::Quote` instance, after any inner content.
    fn end_quote(&self, label: &Option<Label>) -> crate::error::Result<()> {
        Ok(())
    }

    /// Visit each `BlockContent::ThematicBreak` instance.
    fn thematic_break(&self) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the end of each `BlockContent` instance, after any value.
    fn end_block(&self) -> crate::error::Result<()> {
        Ok(())
    }

    /// Return an implementation of the `TableVisitor` trait, if one exists.
    fn table_visitor(&self) -> Option<&dyn TableVisitor> {
        None
    }

    /// Return an implementation of the `InlineVisitor` trait, if one exists.
    fn inline_visitor(&self) -> Option<&dyn InlineVisitor> {
        None
    }
}

// ------------------------------------------------------------------------------------------------

///
/// The visitor trait for all `Table` instances.
///
#[allow(unused_variables)]
pub trait TableVisitor {
    /// Called at the start of each `Table` instance, before any values.
    fn start_table(
        &self,
        caption: &Option<Caption>,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the start of each `Table`'s header row, before any cells.
    fn start_table_header_row(&self) -> crate::error::Result<()> {
        Ok(())
    }

    /// Visit each `Column` instance, with it's position.
    fn table_header_cell(
        &self,
        column_cell: &Column,
        column_idx: usize,
    ) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the end of each `Table`'s header row, after any cells.
    fn end_table_header_row(&self) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the start of each `Table`'s body row, before any cells.
    fn start_table_row(&self, row: usize) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the start of each `Cell` instance, before any inner content.
    fn start_table_cell(
        &self,
        column_idx: usize,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the end of each `Cell` instance, after any inner content.
    fn end_table_cell(&self, column_idx: usize, label: &Option<Label>) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the end of each `Table`'s body row, after any cells.
    fn end_table_row(&self, row: usize) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the end of each `Table` instance, after any values.
    fn end_table(
        &self,
        caption: &Option<Caption>,
        label: &Option<Label>,
    ) -> crate::error::Result<()> {
        Ok(())
    }

    /// Return an implementation of the `InlineVisitor` trait, if one exists.
    fn inline_visitor(&self) -> Option<&dyn InlineVisitor> {
        None
    }
}

// ------------------------------------------------------------------------------------------------

///
/// The visitor trait for all inline content instances.
///
#[allow(unused_variables)]
pub trait InlineVisitor {
    /// Visit each `InlineContent::HyperLink` instance.
    fn link(&self, value: &HyperLink) -> crate::error::Result<()> {
        Ok(())
    }

    /// Visit each `InlineContent::Image` instance.
    fn image(&self, value: &Image) -> crate::error::Result<()> {
        Ok(())
    }

    /// Visit each `InlineContent::Text` instance.
    fn text(&self, value: &Text) -> crate::error::Result<()> {
        Ok(())
    }

    /// Visit each `InlineContent::Math` instance.
    fn math(&self, value: &Math) -> crate::error::Result<()> {
        Ok(())
    }

    /// Visit each `InlineContent::Character` instance.
    fn character(&self, value: &Character) -> crate::error::Result<()> {
        Ok(())
    }

    /// Visit each `InlineContent::LineBreak` instance.
    fn line_break(&self) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the start of each `InlineContent::Span` instance, before any inner content.
    fn start_span(&self, styles: &Vec<SpanStyle>) -> crate::error::Result<()> {
        Ok(())
    }

    /// Called at the end of each `InlineContent::Span` instance, after any inner content.
    fn end_span(&self, styles: &Vec<SpanStyle>) -> crate::error::Result<()> {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Walk the specified `visitor` over the provided `doc`.
///
pub fn walk_document(doc: &Document, visitor: &impl DocumentVisitor) -> crate::error::Result<()> {
    visitor.start_document()?;

    if doc.has_metadata() {
        visitor.start_metadata()?;
        for datum in doc.metadata() {
            visitor.metadata(datum)?;
        }
        visitor.end_metadata()?;
    }

    if let Some(block_visitor) = visitor.block_visitor() {
        if let Some(abstract_block) = doc.abstract_block() {
            block_visitor.start_block()?;
            block_visitor.start_abstract()?;
            if let Some(inline_visitor) = block_visitor.inline_visitor() {
                walk_inline(abstract_block.inner(), inline_visitor)?;
            }
            block_visitor.end_abstract()?;
            block_visitor.end_block()?;
        }
        walk_all_blocks(doc.inner(), block_visitor)?;
    }

    visitor.end_document()
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn walk_all_blocks(
    blocks: &Vec<BlockContent>,
    visitor: &dyn BlockVisitor,
) -> crate::error::Result<()> {
    for block in blocks {
        walk_block(block, visitor)?;
    }
    Ok(())
}

fn walk_block(block: &BlockContent, visitor: &dyn BlockVisitor) -> crate::error::Result<()> {
    visitor.start_block()?;
    match block {
        BlockContent::Comment(v) => visitor.comment(v)?,
        BlockContent::Heading(v) => {
            visitor.start_heading(v.level(), v.label())?;
            if let Some(inline_visitor) = visitor.inline_visitor() {
                walk_inline(v.inner(), inline_visitor)?;
            }
            visitor.end_heading(v.level(), v.label())?;
        }
        BlockContent::ImageBlock(v) => visitor.image(v.inner(), v.caption(), v.label())?,
        BlockContent::MathBlock(v) => visitor.math(v.inner(), v.caption(), v.label())?,
        BlockContent::List(v) => walk_list(v, visitor)?,
        BlockContent::DefinitionList(v) => walk_definition_list(v, visitor)?,
        BlockContent::Formatted(v) => visitor.formatted(v.inner(), v.label())?,
        BlockContent::CodeBlock(v) => {
            visitor.code_block(v.code(), v.language(), v.caption(), v.label())?
        }
        BlockContent::Paragraph(v) => {
            visitor.start_paragraph(v.styles(), v.label())?;
            if let Some(inline_visitor) = visitor.inline_visitor() {
                walk_inline(v.inner(), inline_visitor)?;
            }
            visitor.end_paragraph(v.styles(), v.label())?;
        }
        BlockContent::Quote(v) => {
            visitor.start_quote(v.label())?;
            walk_all_blocks(v.inner(), visitor)?;
            visitor.end_quote(v.label())?;
        }
        BlockContent::Table(v) => {
            if let Some(table_visitor) = visitor.table_visitor() {
                walk_table(v, table_visitor)?;
            }
        }
        BlockContent::ThematicBreak => visitor.thematic_break()?,
    }
    visitor.end_block()?;
    Ok(())
}

fn walk_list(list: &List, visitor: &dyn BlockVisitor) -> crate::error::Result<()> {
    visitor.start_list(list.kind(), list.label())?;
    for inner in list.inner() {
        match inner {
            ListItem::List(v) => {
                walk_list(&v, visitor)?;
            }
            ListItem::Item(v) => {
                visitor.start_list_item(v.label())?;
                if let Some(inline_visitor) = visitor.inline_visitor() {
                    walk_inline(v.inner(), inline_visitor)?;
                }
                visitor.end_list_item(v.label())?;
            }
        }
    }
    visitor.end_list(list.kind(), list.label())
}

fn walk_definition_list(
    list: &DefinitionList,
    visitor: &dyn BlockVisitor,
) -> crate::error::Result<()> {
    visitor.start_definition_list(list.label())?;
    for inner in list.inner() {
        match inner {
            DefinitionListItem::List(v) => {
                walk_definition_list(&v, visitor)?;
            }
            DefinitionListItem::Item(v) => {
                if let Some(inline_visitor) = visitor.inline_visitor() {
                    visitor.start_definition_list_term(v.label())?;
                    walk_inline(v.term().inner(), inline_visitor)?;
                    visitor.end_definition_list_term(v.label())?;

                    visitor.start_definition_list_text()?;
                    walk_inline(v.text().inner(), inline_visitor)?;
                    visitor.end_definition_list_text()?;
                }
            }
        }
    }
    visitor.end_definition_list(list.label())
}

fn walk_table(table: &Table, visitor: &dyn TableVisitor) -> crate::error::Result<()> {
    visitor.start_table(table.caption(), table.label())?;

    if table.has_columns() {
        visitor.start_table_header_row()?;
        for (i, col) in table.columns().iter().enumerate() {
            visitor.table_header_cell(col, i)?;
        }
        visitor.end_table_header_row()?;
    }

    for (i, row) in table.rows().iter().enumerate() {
        visitor.start_table_row(i)?;
        for (j, cell) in row.cells().iter().enumerate() {
            visitor.start_table_cell(j, cell.label())?;
            if let Some(inline_visitor) = visitor.inline_visitor() {
                walk_inline(cell.inner(), inline_visitor)?;
            }
            visitor.end_table_cell(j, cell.label())?;
        }
        visitor.end_table_row(i)?;
    }

    visitor.end_table(table.caption(), table.label())
}

fn walk_inline(
    inline: &Vec<InlineContent>,
    visitor: &dyn InlineVisitor,
) -> crate::error::Result<()> {
    for inline in inline {
        match inline {
            InlineContent::HyperLink(v) => visitor.link(v)?,
            InlineContent::Image(v) => visitor.image(v)?,
            InlineContent::Text(v) => visitor.text(v)?,
            InlineContent::Math(v) => visitor.math(v)?,
            InlineContent::Character(v) => visitor.character(v)?,
            InlineContent::LineBreak => visitor.line_break()?,
            InlineContent::Span(v) => {
                visitor.start_span(v.styles())?;
                walk_inline(v.inner(), visitor)?;
                visitor.end_span(v.styles())?;
            }
        }
    }
    Ok(())
}
