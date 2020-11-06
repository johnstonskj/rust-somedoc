/*!
One-line description.

More detailed description, with

# Example

*/
use crate::error;
use crate::model::blocks::BlockContent;
use crate::model::inline::{HasInlineContent, InlineContent};
use crate::model::ComplexContent;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum ListKind {
    Ordered,
    Unordered,
}

#[derive(Clone, Debug)]
pub struct List {
    kind: ListKind,
    inner: Vec<ListItem>,
}

#[derive(Clone, Debug)]
pub enum ListItem {
    List(List),
    Item(Item),
}

#[derive(Clone, Debug)]
pub struct Item {
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

impl Default for ListKind {
    fn default() -> Self {
        Self::Unordered
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for List {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

block_impls!(List);

impl List {
    pub fn new(kind: ListKind) -> Self {
        Self {
            kind,
            inner: Default::default(),
        }
    }

    pub fn ordered() -> Self {
        Self::new(ListKind::Ordered)
    }

    pub fn unordered() -> Self {
        Self::new(ListKind::Unordered)
    }

    pub fn inner(&self) -> &Vec<ListItem> {
        &self.inner
    }

    pub fn add_item(&mut self, item: ListItem) {
        self.inner.push(item);
    }

    pub fn is_ordered(&self) -> bool {
        match self.kind {
            ListKind::Ordered => true,
            _ => false,
        }
    }

    pub fn kind(&self) -> &ListKind {
        &self.kind
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Item {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

has_inline_impls!(Item);

// ------------------------------------------------------------------------------------------------

impl From<List> for ListItem {
    fn from(value: List) -> Self {
        Self::List(value)
    }
}

impl From<Item> for ListItem {
    fn from(value: Item) -> Self {
        Self::Item(value)
    }
}

impl ListItem {
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
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
