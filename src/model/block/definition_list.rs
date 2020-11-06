/*!
One-line description.

More detailed description, with

# Example

*/
use crate::error;
use crate::model::block::BlockContent;
use crate::model::inline::{HasInlineContent, InlineContent};
use crate::model::ComplexContent;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct DefinitionList {
    inner: Vec<DefinitionListItem>,
}

#[derive(Clone, Debug)]
pub enum DefinitionListItem {
    List(DefinitionList),
    Item(Definition),
}

#[derive(Clone, Debug)]
pub struct Definition {
    term: DefinitionPart,
    text: DefinitionPart,
}

#[derive(Clone, Debug)]
pub struct DefinitionPart {
    inner: Vec<InlineContent>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
