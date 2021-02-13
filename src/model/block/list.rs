use crate::error;
use crate::model::block::{BlockContent, Label};
use crate::model::inline::Text;
use crate::model::inline::{HasInlineContent, InlineContent};
use crate::model::{block::HasLabel, HasInnerContent};
#[cfg(feature = "fmt_json")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The form of list, currently this only covers the ordering of items.
///
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "fmt_json", derive(Serialize, Deserialize))]
pub enum ListKind {
    /// An ordered/numbered item list.
    Ordered,
    /// An unordered/bulleted item list.
    Unordered,
}

///
/// A list is either a bulleted or unordered set of values, or an enumerated list of values.
///
/// A `List` is a tree structure with `ListItem` being the inner nodes in the tree
/// and which may contain either another list, or a `Item`.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "fmt_json", derive(Serialize, Deserialize))]
pub struct List {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    label: Option<Label>,
    kind: ListKind,
    inner: Vec<ListItem>,
}

///
/// Inner node in a `List` tree.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "fmt_json", derive(Serialize, Deserialize))]
pub enum ListItem {
    List(List),
    Item(Item),
}

///
/// A leaf in the `List` tree, it's inner content list of `InlineContent` values.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "fmt_json", derive(Serialize, Deserialize))]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
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
    /// Create a new list of the provided `kind`.
    pub fn new(kind: ListKind) -> Self {
        Self {
            label: None,
            kind,
            inner: Default::default(),
        }
    }

    /// Create a new ordered/numbered item list.
    pub fn ordered() -> Self {
        Self::new(ListKind::Ordered)
    }

    /// Create a new unordered/bulleted item list.
    pub fn unordered() -> Self {
        Self::new(ListKind::Unordered)
    }

    // --------------------------------------------------------------------------------------------

    /// Returns true if this list contains any items or sub-lists.
    pub fn has_inner(&self) -> bool {
        !self.inner.is_empty()
    }

    /// Return the vector of list items or sub-lists.
    pub fn inner(&self) -> &Vec<ListItem> {
        &self.inner
    }

    // --------------------------------------------------------------------------------------------

    fn add_inner(&mut self, item: ListItem) -> &mut Self {
        self.inner.push(item);
        self
    }

    /// Add a new item to this list.
    pub fn add_item(&mut self, item: Item) -> &mut Self {
        self.add_inner(ListItem::Item(item))
    }

    /// Add a new item to this list.
    pub fn add_item_str(&mut self, item: &str) -> &mut Self {
        self.add_item_from(Text::from(item).into())
    }

    /// Add a new item to this list from the provided content.
    pub fn add_item_from(&mut self, item: InlineContent) -> &mut Self {
        self.add_inner(ListItem::Item(item.into()))
    }

    pub fn add_sub_list(&mut self, item: List) -> &mut Self {
        self.add_inner(ListItem::List(item))
    }

    // --------------------------------------------------------------------------------------------

    /// Return `true` if this is an ordered/numbered item list, else `false`.
    pub fn is_ordered(&self) -> bool {
        matches!(self.kind, ListKind::Ordered)
    }

    /// Return `true` if this is an unordered/bulleted item list, else `false`.
    pub fn is_unordered(&self) -> bool {
        matches!(self.kind, ListKind::Unordered)
    }

    /// Return the kind of this list.
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
    /// Is this list item a sub-list.
    pub fn is_sub_list(&self) -> bool {
        matches!(&self, Self::List(_))
    }

    /// Is this list item a leaf item.
    pub fn is_item(&self) -> bool {
        matches!(&self, Self::Item(_))
    }
}
