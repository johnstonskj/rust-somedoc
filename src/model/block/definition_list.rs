use crate::error;
use crate::model::block::{BlockContent, Label};
use crate::model::inline::{HasInlineContent, InlineContent};
use crate::model::{block::HasLabel, HasInnerContent};

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
    label: Option<Label>,
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
    label: Option<Label>,
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
            label: None,
            inner: Default::default(),
        }
    }
}

label_impl!(DefinitionList);

block_impls!(DefinitionList);

impl DefinitionList {
    pub fn has_inner(&self) -> bool {
        !self.inner.is_empty()
    }

    pub fn inner(&self) -> &Vec<DefinitionListItem> {
        &self.inner
    }

    // --------------------------------------------------------------------------------------------

    pub fn add_inner(&mut self, item: DefinitionListItem) -> &mut Self {
        self.inner.push(item);
        self
    }

    pub fn add_definition(&mut self, item: Definition) -> &mut Self {
        self.add_inner(DefinitionListItem::Item(item))
    }

    pub fn add_definition_from(&mut self, term: DefinitionPart, text: DefinitionPart) -> &mut Self {
        self.add_inner(DefinitionListItem::Item(Definition::new(term, text)))
    }

    pub fn add_sub_list(&mut self, item: DefinitionList) -> &mut Self {
        self.add_inner(DefinitionListItem::List(item))
    }
}

// ------------------------------------------------------------------------------------------------

impl From<DefinitionList> for DefinitionListItem {
    fn from(value: DefinitionList) -> Self {
        Self::List(value)
    }
}

impl From<Definition> for DefinitionListItem {
    fn from(value: Definition) -> Self {
        Self::Item(value)
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

label_impl!(Definition);

impl Definition {
    pub fn new(term: DefinitionPart, text: DefinitionPart) -> Self {
        Self {
            label: None,
            term,
            text,
        }
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
