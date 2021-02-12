use crate::error;
use crate::model::block::{BlockContent, Label};
use crate::model::inline::{HasInlineContent, InlineContent};
use crate::model::{block::HasLabel, HasInnerContent};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A list of terms and definitions, this may be used for terminology, glossaries, or acronym
/// expansion.
///
/// Notes
///
/// 1. While some markup languages support a single definition for multiple terms, or multiple
///    definitions for the same term, `DefinitionList` only supports a one-to-one relationship.
/// 1. A `DefinitionList` is also not hierarchical, definition lists only contain definitions.
/// 1. A `Definition` consists of a string term and a `DefinitionPart` that is an inline content
///    container.
///
#[derive(Clone, Debug)]
pub struct DefinitionList {
    label: Option<Label>,
    inner: Vec<Definition>,
}

///
/// A single definition within the list.
///
#[derive(Clone, Debug)]
pub struct Definition {
    label: Option<Label>,
    term: String,
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
    /// Return `true` if this list contains any definitions, else `false`.
    pub fn has_inner(&self) -> bool {
        !self.inner.is_empty()
    }

    /// Return the list of definitions in this list.
    pub fn inner(&self) -> &Vec<Definition> {
        &self.inner
    }

    // --------------------------------------------------------------------------------------------

    /// Add a new definition to this list.
    pub fn add_definition(&mut self, item: Definition) -> &mut Self {
        self.inner.push(item);
        self
    }

    /// Add a new definition to this list.
    pub fn add_definition_from(&mut self, term: &str, text: DefinitionPart) -> &mut Self {
        self.add_definition(Definition::new(term, text))
    }
}

// ------------------------------------------------------------------------------------------------

label_impl!(Definition);

impl Definition {
    /// Create a new definition, a term and its corresponding text.
    pub fn new(term: &str, text: DefinitionPart) -> Self {
        Self {
            label: None,
            term: term.to_string(),
            text,
        }
    }

    /// Return the term part of this definition.
    pub fn term(&self) -> &String {
        &self.term
    }

    /// Return the text part of this definition.
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
