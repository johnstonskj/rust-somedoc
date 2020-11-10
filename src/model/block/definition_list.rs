use crate::error;
use crate::model::block::BlockContent;
use crate::model::inline::{HasInlineContent, InlineContent};
use crate::model::HasInnerContent;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A list of terms and definitions. Note that while some markup languages support a single definition
/// for multiple terms, or multiple definitions for the same term, `DefinitionList` only supports a
/// one-to-one relationship.
///
/// A `DefinitionList` is a tree structure with `DefinitionListItem` being the inner nodes in the tree
/// and which may contain either another list, or a `Definition`.
///
/// Finally, a `Definition` consists of a string term and a `DefinitionPart` that is an inline content
/// container.
///
#[derive(Clone, Debug)]
pub struct DefinitionList {
    inner: Vec<DefinitionListItem>,
}

///
/// Inner node in a `DefinitionList` tree.
///
#[derive(Clone, Debug)]
pub enum DefinitionListItem {
    List(DefinitionList),
    Item(Definition),
}

///
/// A leaf in the `DefinitionList` tree, it contains the term and definition text.
///
#[derive(Clone, Debug)]
pub struct Definition {
    term: DefinitionPart,
    text: DefinitionPart,
}

///
/// The `Definition` text, it's inner content list of `InlineContent` values.
///
#[derive(Clone, Debug)]
pub struct DefinitionPart {
    inner: Vec<InlineContent>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for DefinitionList {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

block_impls!(DefinitionList);

impl DefinitionList {
    pub fn inner(&self) -> &Vec<DefinitionListItem> {
        &self.inner
    }

    pub fn add_item(&mut self, item: DefinitionListItem) {
        self.inner.push(item);
    }
}

// ------------------------------------------------------------------------------------------------

impl From<DefinitionList> for DefinitionListItem {
    fn from(value: DefinitionList) -> Self {
        Self::List(value)
    }
}

impl DefinitionListItem {
    pub fn is_sub_list(&self) -> bool {
        match &self {
            Self::List(_) => true,
            _ => false,
        }
    }

    pub fn is_item(&self) -> bool {
        match &self {
            Self::Item(_) => true,
            _ => false,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Definition {
    pub fn new(term: DefinitionPart, text: DefinitionPart) -> Self {
        Self { term, text }
    }

    pub fn term(&self) -> &DefinitionPart {
        &self.term
    }

    pub fn text(&self) -> &DefinitionPart {
        &self.text
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for DefinitionPart {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

has_inline_impls!(DefinitionPart);
