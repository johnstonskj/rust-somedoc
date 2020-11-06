/*!
One-line description.

More detailed description, with

# Example

*/

use crate::model::blocks::quote::Quote;
use crate::model::blocks::{
    BlockContent, CodeBlock, DefinitionList, Heading, List, Paragraph, Table,
};
use crate::model::document::{Document, Metadata};
use crate::model::inline::{Anchor, HyperLink, Image, Text};
use crate::model::ComplexContent;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait VisitorFactory {
    fn metadata(&self) -> Option<Box<&dyn MetadataVisitor>>;
    fn block(&self) -> Option<Box<&dyn BlockContentVisitor>>;
    fn inline(&self) -> Option<Box<&dyn InlineContentVisitor>>;
}

pub trait InlineContentVisitor {
    #[allow(unused_variables)]
    fn anchor(&self, value: &Anchor) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn link(&self, value: &HyperLink) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn image(&self, value: &Image) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn text(&self, value: &Text) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn linre_break(&self) -> bool {
        true
    }
}

pub trait BlockContentVisitor {
    #[allow(unused_variables)]
    fn code_block(&self, code: &CodeBlock) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn comment(&self, code: &String) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn definition_list(&self, value: &DefinitionList) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn heading(&self, value: &Heading) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn image(&self, value: &Image) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn list(&self, value: &List) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn paragraph(&self, value: &Paragraph) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn block_quote(&self, value: &Quote) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn table(&self, value: &Table) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn thematic_break(&self) -> bool {
        true
    }
}

pub trait MetadataVisitor {
    #[allow(unused_variables)]
    fn metadata(&self, value: &Metadata) -> bool {
        true
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn visit(doc: &Document, factory: &impl VisitorFactory) -> bool {
    if let Some(visitor) = factory.metadata() {
        for datum in doc.metadata() {
            if !visitor.metadata(datum) {
                return false;
            }
        }
    }

    if let Some(visitor) = factory.block() {
        visit_blocks(doc.inner(), visitor)
    } else {
        true
    }
}

fn visit_blocks(content: &Vec<BlockContent>, visitor: Box<&dyn BlockContentVisitor>) -> bool {
    for content in content {
        if !match content {
            BlockContent::Comment(value) => visitor.comment(value),
            BlockContent::Heading(value) => visitor.heading(value),
            BlockContent::Image(value) => visitor.image(value),
            BlockContent::List(value) => visitor.list(value),
            BlockContent::DefinitionList(value) => visitor.definition_list(value),
            BlockContent::CodeBlock(value) => visitor.code_block(value),
            BlockContent::Paragraph(value) => visitor.paragraph(value),
            BlockContent::Quote(value) => visitor.block_quote(value),
            BlockContent::Table(value) => visitor.table(value),
            BlockContent::ThematicBreak => visitor.thematic_break(),
        } {
            return false;
        }
    }

    true
}

// fn visit_inline(content: &Vec<InlineContent>, visitor: Box<&dyn InlineContentVisitor>) -> bool {
//     for content in content {
//         if !match content {
//             InlineContent::Anchor(value) => visitor.anchor(value),
//             InlineContent::Link(value) => visitor.link(value),
//             InlineContent::Text(value) => visitor.text(value),
//         } {
//             return false;
//         }
//     }
//
//     true
// }

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
