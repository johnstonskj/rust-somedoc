use crate::error;
use crate::model::block::{BlockContent, Label};
use crate::model::inline::{HasInlineContent, InlineContent};
use crate::model::{block::HasLabel, HasInnerContent};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The form of list, currently this only covers the ordering of items.
///
#[derive(Clone, Debug, PartialEq)]
pub enum ListKind {
    Ordered,
    Unordered,
}

///
/// A list is either a bulleted or unordered set of values, or an enumerated list of values.
///
/// A `List` is a tree structure with `ListItem` being the inner nodes in the tree
/// and which may contain either another list, or a `Item`.
///
#[derive(Clone, Debug)]
pub struct List {
    label: Option<Label>,
    kind: ListKind,
    inner: Vec<ListItem>,
}

///
/// Inner node in a `List` tree.
///
#[derive(Clone, Debug)]
pub enum ListItem {
    List(List),
    Item(Item),
}

///
/// A leaf in the `List` tree, it's inner content list of `InlineContent` values.
///
#[derive(Clone, Debug)]
pub struct Item {
    label: Option<Label>,
    inner: Vec<InlineContent>,
}

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

label_impl!(List);

block_impls!(List);

impl List {
    pub fn new(kind: ListKind) -> Self {
        Self {
            label: None,
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

    // --------------------------------------------------------------------------------------------

    pub fn has_inner(&self) -> bool {
        !self.inner.is_empty()
    }

    pub fn inner(&self) -> &Vec<ListItem> {
        &self.inner
    }

    // --------------------------------------------------------------------------------------------

    pub fn add_inner(&mut self, item: ListItem) -> &mut Self {
        self.inner.push(item);
        self
    }

    pub fn add_item(&mut self, item: Item) -> &mut Self {
        self.add_inner(ListItem::Item(item))
    }

    pub fn add_item_from(&mut self, item: InlineContent) -> &mut Self {
        self.add_inner(ListItem::Item(item.into()))
    }

    pub fn add_sub_list(&mut self, item: List) -> &mut Self {
        self.add_inner(ListItem::List(item))
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_ordered(&self) -> bool {
        match self.kind {
            ListKind::Ordered => true,
            _ => false,
        }
    }

    pub fn is_unordered(&self) -> bool {
        match self.kind {
            ListKind::Unordered => true,
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
            label: None,
            inner: Default::default(),
        }
    }
}

label_impl!(Item);

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
