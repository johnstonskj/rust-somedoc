/*!
One-line description.

More detailed description, with

# Example

*/

use crate::model::block::{
    BlockContent, Caption, Captioned, CodeBlock, Column, DefinitionList, DefinitionListItem,
    Formatted, HeadingLevel, List, ListItem, ListKind, MathBlock, ParagraphStyle, Table,
};
use crate::model::document::Metadata;
use crate::model::inline::{
    Anchor, Character, HyperLink, Image, InlineContent, Math, SpanStyle, Text,
};
use crate::model::HasInnerContent;
use crate::model::{Document, HasStyles};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
pub trait DocumentVisitor {
    fn start_document(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn metadata(&self, metadatum: &Metadata) -> crate::error::Result<()> {
        Ok(())
    }

    fn block_visitor(&self) -> Option<&dyn BlockVisitor> {
        None
    }

    fn end_document(&self) -> crate::error::Result<()> {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
pub trait BlockVisitor {
    fn start_block(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn comment(&self, value: &str) -> crate::error::Result<()> {
        Ok(())
    }

    fn start_heading(&self, level: &HeadingLevel) -> crate::error::Result<()> {
        Ok(())
    }

    fn end_heading(&self, level: &HeadingLevel) -> crate::error::Result<()> {
        Ok(())
    }

    fn image(&self, value: &Image) -> crate::error::Result<()> {
        Ok(())
    }

    fn math(&self, value: &MathBlock) -> crate::error::Result<()> {
        Ok(())
    }

    fn start_list(&self, kind: &ListKind) -> crate::error::Result<()> {
        Ok(())
    }

    fn end_list(&self, kind: &ListKind) -> crate::error::Result<()> {
        Ok(())
    }

    fn start_list_item(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn end_list_item(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn start_definition_list(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn end_definition_list(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn start_definition_list_term(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn end_definition_list_term(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn start_definition_list_text(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn end_definition_list_text(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn formatted(&self, value: &Formatted) -> crate::error::Result<()> {
        Ok(())
    }

    fn code_block(&self, value: &CodeBlock) -> crate::error::Result<()> {
        Ok(())
    }

    fn start_paragraph(&self, styles: &Vec<ParagraphStyle>) -> crate::error::Result<()> {
        Ok(())
    }

    fn end_paragraph(&self, styles: &Vec<ParagraphStyle>) -> crate::error::Result<()> {
        Ok(())
    }

    fn start_quote(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn end_quote(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn thematic_break(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn end_block(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn table_visitor(&self) -> Option<&dyn TableVisitor> {
        None
    }

    fn inline_visitor(&self) -> Option<&dyn InlineVisitor> {
        None
    }
}

// ------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
pub trait TableVisitor {
    fn start_table(&self, caption: &Option<Caption>) -> crate::error::Result<()> {
        Ok(())
    }

    fn start_table_header_row(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn table_header_cell(&self, cell: &Column, column: usize) -> crate::error::Result<()> {
        Ok(())
    }

    fn end_table_header_row(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn start_table_row(&self, row: usize) -> crate::error::Result<()> {
        Ok(())
    }

    fn start_table_cell(&self, column: usize) -> crate::error::Result<()> {
        Ok(())
    }

    fn end_table_cell(&self, column: usize) -> crate::error::Result<()> {
        Ok(())
    }

    fn end_table_row(&self, row: usize) -> crate::error::Result<()> {
        Ok(())
    }

    fn end_table(&self, caption: &Option<Caption>) -> crate::error::Result<()> {
        Ok(())
    }

    fn inline_visitor(&self) -> Option<&dyn InlineVisitor> {
        None
    }
}

// ------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
pub trait InlineVisitor {
    fn anchor(&self, value: &Anchor) -> crate::error::Result<()> {
        Ok(())
    }

    fn link(&self, value: &HyperLink) -> crate::error::Result<()> {
        Ok(())
    }

    fn image(&self, value: &Image) -> crate::error::Result<()> {
        Ok(())
    }

    fn text(&self, value: &Text) -> crate::error::Result<()> {
        Ok(())
    }

    fn math(&self, value: &Math) -> crate::error::Result<()> {
        Ok(())
    }

    fn character(&self, value: &Character) -> crate::error::Result<()> {
        Ok(())
    }

    fn start_span(&self, styles: &Vec<SpanStyle>) -> crate::error::Result<()> {
        Ok(())
    }

    fn line_break(&self) -> crate::error::Result<()> {
        Ok(())
    }

    fn end_span(&self, styles: &Vec<SpanStyle>) -> crate::error::Result<()> {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn walk_document(doc: &Document, visitor: &impl DocumentVisitor) -> crate::error::Result<()> {
    visitor.start_document()?;

    for datum in doc.metadata() {
        visitor.metadata(datum)?;
    }

    if let Some(block_visitor) = visitor.block_visitor() {
        walk_all_blocks(doc.inner(), block_visitor)?;
    }

    visitor.end_document()
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

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
            visitor.start_heading(v.level())?;
            if let Some(inline_visitor) = visitor.inline_visitor() {
                walk_inline(v.inner(), inline_visitor)?;
            }
            visitor.end_heading(v.level())?;
        }
        BlockContent::Image(v) => visitor.image(v)?,
        BlockContent::MathBlock(v) => visitor.math(v)?,
        BlockContent::List(v) => walk_list(v, visitor)?,
        BlockContent::DefinitionList(v) => walk_definition_list(v, visitor)?,
        BlockContent::Formatted(v) => visitor.formatted(v)?,
        BlockContent::CodeBlock(v) => visitor.code_block(v)?,
        BlockContent::Paragraph(v) => {
            visitor.start_paragraph(v.styles())?;
            if let Some(inline_visitor) = visitor.inline_visitor() {
                walk_inline(v.inner(), inline_visitor)?;
            }
            visitor.end_paragraph(v.styles())?;
        }
        BlockContent::Quote(v) => {
            visitor.start_quote()?;
            walk_all_blocks(v.inner(), visitor)?;
            visitor.end_quote()?;
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
    visitor.start_list(list.kind())?;
    for inner in list.inner() {
        match inner {
            ListItem::List(v) => {
                walk_list(&v, visitor)?;
            }
            ListItem::Item(v) => {
                visitor.start_list_item()?;
                if let Some(inline_visitor) = visitor.inline_visitor() {
                    walk_inline(v.inner(), inline_visitor)?;
                }
                visitor.end_list_item()?;
            }
        }
    }
    visitor.end_list(list.kind())
}

fn walk_definition_list(
    list: &DefinitionList,
    visitor: &dyn BlockVisitor,
) -> crate::error::Result<()> {
    visitor.start_definition_list()?;
    for inner in list.inner() {
        match inner {
            DefinitionListItem::List(v) => {
                walk_definition_list(&v, visitor)?;
            }
            DefinitionListItem::Item(v) => {
                if let Some(inline_visitor) = visitor.inline_visitor() {
                    visitor.start_definition_list_term()?;
                    walk_inline(v.term().inner(), inline_visitor)?;
                    visitor.end_definition_list_term()?;

                    visitor.start_definition_list_text()?;
                    walk_inline(v.text().inner(), inline_visitor)?;
                    visitor.end_definition_list_text()?;
                }
            }
        }
    }
    visitor.end_definition_list()
}

fn walk_table(table: &Table, visitor: &dyn TableVisitor) -> crate::error::Result<()> {
    visitor.start_table(table.caption())?;

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
            visitor.start_table_cell(j)?;
            if let Some(inline_visitor) = visitor.inline_visitor() {
                walk_inline(cell.inner(), inline_visitor)?;
            }
            visitor.end_table_cell(j)?;
        }
        visitor.end_table_row(i)?;
    }

    visitor.end_table(table.caption())
}

fn walk_inline(
    inline: &Vec<InlineContent>,
    visitor: &dyn InlineVisitor,
) -> crate::error::Result<()> {
    for inline in inline {
        match inline {
            InlineContent::Anchor(v) => visitor.anchor(v)?,
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
