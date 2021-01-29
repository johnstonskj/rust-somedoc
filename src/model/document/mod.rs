/*!
This module provides the root `Document` type and document metadata properties.
*/

use crate::error;
use crate::model::block::{BlockContent, HasBlockContent};
use crate::model::HasInnerContent;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Common metadata properties.
///
#[derive(Clone, Debug, PartialEq)]
pub enum Metadata {
    /// An author to attribute.
    Author(Author),
    /// The document's class, or type.
    Class(Class),
    /// A structured copyright statement.
    Copyright(Copyright),
    /// The date of this document.
    Date(String),
    /// Keywords to apply to this document.
    Keywords(Vec<String>),
    /// The revision identifier of this document.
    Revision(String),
    /// The publication status of this document.
    Status(String),
    /// This document's title.
    Title(String),
    /// An unknown property.
    Other(SimpleProperty),
}

///
///  A structured metadata property.
///
#[derive(Clone, Debug, PartialEq)]
pub struct Author {
    pub name: String,
    pub email: Option<String>,
    pub organization: Option<String>,
}

///
///  A structured metadata property.
///
#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    pub name_or_path: String,
}

///
///  A structured metadata property.
///
#[derive(Clone, Debug, PartialEq)]
pub struct Copyright {
    pub year: u16,
    pub organization: Option<String>,
    pub comment: Option<String>,
}

///
///  A structured metadata property.
///
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleProperty {
    pub name: String,
    pub value: String,
}

///
/// The root document itself, this contains a list of `BlockContent` values as well as a list of
/// metadata properties.
///
/// Note that the `add_` and `set_` methods all return `&mut Self` and so calls to these may be chained.
///
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

impl Default for Document {
    fn default() -> Self {
        Self {
            metadata: Default::default(),
            content: Default::default(),
        }
    }
}

impl Into<Document> for BlockContent {
    fn into(self) -> Document {
        let mut doc = Document::default();
        let _ = doc.add_content(self);
        doc
    }
}

has_block_impls!(Document);

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

    pub fn add_metadata(&mut self, datum: Metadata) -> error::Result<&mut Self> {
        self.metadata.push(datum);
        Ok(self)
    }

    pub fn set_title(&mut self, v: &str) -> error::Result<&mut Self> {
        self.add_metadata(Metadata::Title(v.to_string()))?;
        Ok(self)
    }

    pub fn set_date(&mut self, v: &str) -> error::Result<&mut Self> {
        self.add_metadata(Metadata::Date(v.to_string()))?;
        Ok(self)
    }

    pub fn add_author(&mut self, v: Author) -> error::Result<&mut Self> {
        self.add_metadata(Metadata::Author(v))?;
        Ok(self)
    }

    pub fn add_author_str(
        &mut self,
        name: &str,
        email: Option<&str>,
        organization: Option<&str>,
    ) -> error::Result<&mut Self> {
        self.add_metadata(Metadata::Author(Author {
            name: name.to_string(),
            email: email.map(str::to_string),
            organization: organization.map(str::to_string),
        }))?;
        Ok(self)
    }

    pub fn add_copyright(&mut self, v: Copyright) -> error::Result<&mut Self> {
        self.add_metadata(Metadata::Copyright(v))?;
        Ok(self)
    }

    pub fn add_copyright_str(
        &mut self,
        year: u16,
        organization: Option<&str>,
        comment: Option<&str>,
    ) -> error::Result<&mut Self> {
        self.add_metadata(Metadata::Copyright(Copyright {
            year,
            organization: organization.map(str::to_string),
            comment: comment.map(str::to_string),
        }))?;
        Ok(self)
    }

    pub fn add_keywords(&mut self, v: &[String]) -> error::Result<&mut Self> {
        self.add_metadata(Metadata::Keywords(v.to_vec()))
    }

    pub fn add_keywords_str(&mut self, v: &[&str]) -> error::Result<&mut Self> {
        self.add_metadata(Metadata::Keywords(
            v.iter().cloned().map(str::to_string).collect(),
        ))?;
        Ok(self)
    }

    pub fn add_metadata_property(&mut self, v: SimpleProperty) -> error::Result<&mut Self> {
        self.add_metadata(Metadata::Other(v))?;
        Ok(self)
    }

    pub fn add_metadata_property_str(
        &mut self,
        name: &str,
        value: &str,
    ) -> error::Result<&mut Self> {
        self.add_metadata(Metadata::Other(SimpleProperty {
            name: name.to_string(),
            value: value.to_string(),
        }))?;
        Ok(self)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
