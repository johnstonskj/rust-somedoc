/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error;
use crate::model::block::{BlockContent, HasBlockContent};
use crate::model::ComplexContent;
use std::fmt::{Display, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum DocumentMetadataKind {
    Author,
    Copyright,
    Date,
    Organization,
    SubTitle,
    Title,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Metadata {
    kind: DocumentMetadataKind,
    value: Option<String>,
}

#[derive(Debug)]
pub struct Document {
    metadata: Vec<Metadata>,
    content: Vec<BlockContent>,
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

impl Display for DocumentMetadataKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DocumentMetadataKind::Author => "author",
                DocumentMetadataKind::Copyright => "copyright",
                DocumentMetadataKind::Date => "date",
                DocumentMetadataKind::Organization => "organization",
                DocumentMetadataKind::SubTitle => "sub_title",
                DocumentMetadataKind::Title => "title",
            }
        )
    }
}

impl Metadata {
    pub fn new(kind: DocumentMetadataKind) -> Self {
        Self { kind, value: None }
    }

    pub fn new_with_value(kind: DocumentMetadataKind, value: &str) -> error::Result<Self> {
        if value.is_empty() {
            Err(error::ErrorKind::MustNotBeEmpty.into())
        } else {
            Ok(Self {
                kind,
                value: Some(value.to_string()),
            })
        }
    }

    pub fn kind(&self) -> &DocumentMetadataKind {
        &self.kind
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub fn value(&self) -> &Option<String> {
        &self.value
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Document {
    fn default() -> Self {
        Self {
            metadata: Default::default(),
            content: Default::default(),
        }
    }
}

impl ComplexContent<BlockContent> for Document {
    fn inner(&self) -> &Vec<BlockContent> {
        &self.content
    }

    fn inner_mut(&mut self) -> &mut Vec<BlockContent> {
        &mut self.content
    }

    fn add_content(&mut self, content: BlockContent) -> error::Result<()> {
        self.content.push(content);
        Ok(())
    }
}

impl HasBlockContent for Document {}

impl Document {
    pub fn has_metadata(&self) -> bool {
        !self.metadata.is_empty()
    }

    pub fn metadata(&self) -> &Vec<Metadata> {
        &self.metadata
    }

    pub fn metadata_mut(&mut self) -> &mut Vec<Metadata> {
        &mut self.metadata
    }

    pub fn add_metadata(&mut self, datum: Metadata) -> error::Result<()> {
        self.metadata.push(datum);
        Ok(())
    }

    pub fn set_title(&mut self, v: &str) -> error::Result<()> {
        self.add_metadata(Metadata::new_with_value(DocumentMetadataKind::Title, v)?)
    }

    pub fn set_subtitle(&mut self, v: &str) -> error::Result<()> {
        self.add_metadata(Metadata::new_with_value(DocumentMetadataKind::SubTitle, v)?)
    }

    pub fn set_date(&mut self, v: &str) -> error::Result<()> {
        self.add_metadata(Metadata::new_with_value(DocumentMetadataKind::Date, v)?)
    }

    pub fn add_author(&mut self, v: &str) -> error::Result<()> {
        self.add_metadata(Metadata::new_with_value(DocumentMetadataKind::Author, v)?)
    }

    pub fn set_organization(&mut self, v: &str) -> error::Result<()> {
        self.add_metadata(Metadata::new_with_value(
            DocumentMetadataKind::Organization,
            v,
        )?)
    }

    pub fn set_copyright(&mut self, v: &str) -> error::Result<()> {
        self.add_metadata(Metadata::new_with_value(
            DocumentMetadataKind::Copyright,
            v,
        )?)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
