/*!
This module provides the root `Document` type and document metadata properties.
*/

use crate::error;
use crate::model::block::{BlockContent, HasBlockContent, Paragraph};
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
    /// The author's name.
    pub name: String,
    /// Optional email address for the author.
    pub email: Option<String>,
    /// Optional organizational affiliation for the author.
    pub organization: Option<String>,
}

///
///  A structured metadata property.
///
#[derive(Clone, Debug, PartialEq)]
pub struct Copyright {
    /// Year of copyright.
    pub year: u16,
    /// Copyright holder.
    pub organization: Option<String>,
    /// Additional comments.
    pub comment: Option<String>,
}

///
///  A structured metadata property.
///
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleProperty {
    /// The property key, or name.
    pub key: String,
    /// The property value.
    pub value: String,
}

///
/// The root document itself, this contains a list of `BlockContent` values as well as a list of
/// metadata properties.
///
/// Note that the `add_` and `set_` methods all return `&mut Self` and so calls to these may be chained.
///
#[derive(Clone, Debug)]
pub struct Document {
    metadata: Vec<Metadata>,
    abstract_block: Option<Paragraph>,
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
            abstract_block: None,
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
    /// Returns `true` if this document has any associated metadata, else `false`.
    pub fn has_metadata(&self) -> bool {
        !self.metadata.is_empty()
    }

    /// Return the list of associated metadata.
    pub fn metadata(&self) -> &Vec<Metadata> {
        &self.metadata
    }

    /// Add a metadata item to this document.
    pub fn add_metadata(&mut self, datum: Metadata) -> &mut Self {
        self.metadata.push(datum);
        self
    }

    /// Set the title (metadata value) of this document.
    pub fn set_title(&mut self, v: &str) -> &mut Self {
        self.add_metadata(Metadata::Title(v.to_string()))
    }

    /// Set the date (metadata value) of this document.
    pub fn set_date(&mut self, v: &str) -> &mut Self {
        self.add_metadata(Metadata::Date(v.to_string()))
    }

    /// Add an author (metadata value) to this document.
    pub fn add_author(&mut self, v: Author) -> &mut Self {
        self.add_metadata(Metadata::Author(v))
    }

    /// Add an author (metadata value) to this document.
    pub fn add_author_str(
        &mut self,
        name: &str,
        email: Option<&str>,
        organization: Option<&str>,
    ) -> &mut Self {
        self.add_metadata(Metadata::Author(Author {
            name: name.to_string(),
            email: email.map(str::to_string),
            organization: organization.map(str::to_string),
        }))
    }

    /// Add a copyright (metadata value) to this document.
    pub fn add_copyright(&mut self, v: Copyright) -> &mut Self {
        self.add_metadata(Metadata::Copyright(v))
    }

    /// Add a copyright (metadata value) to this document.
    pub fn add_copyright_str(
        &mut self,
        year: u16,
        organization: Option<&str>,
        comment: Option<&str>,
    ) -> &mut Self {
        self.add_metadata(Metadata::Copyright(Copyright {
            year,
            organization: organization.map(str::to_string),
            comment: comment.map(str::to_string),
        }))
    }

    /// Add a list of keywords (metadata value) to this document.
    pub fn add_keywords(&mut self, v: &[String]) -> &mut Self {
        self.add_metadata(Metadata::Keywords(v.to_vec()))
    }

    /// Add a list of keywords (metadata value) to this document.
    pub fn add_keywords_str(&mut self, v: &[&str]) -> &mut Self {
        self.add_metadata(Metadata::Keywords(
            v.iter().cloned().map(str::to_string).collect(),
        ))
    }

    /// Add an arbitrary, named, metadata value)to this document.
    pub fn add_metadata_property(&mut self, v: SimpleProperty) -> &mut Self {
        self.add_metadata(Metadata::Other(v))
    }

    /// Add an arbitrary, named, metadata value)to this document.
    pub fn add_metadata_property_str(&mut self, name: &str, value: &str) -> &mut Self {
        self.add_metadata(Metadata::Other(SimpleProperty {
            key: name.to_string(),
            value: value.to_string(),
        }))
    }

    /// Add a `Paragraph` acting as the abstract to this document.
    pub fn add_abstract(&mut self, abstract_block: Paragraph) -> &mut Self {
        self.abstract_block = Some(abstract_block);
        self
    }

    /// Return the `Paragraph`, if present, acting as the abstract to this document.
    pub fn abstract_block(&self) -> &Option<Paragraph> {
        &self.abstract_block
    }
}

// ------------------------------------------------------------------------------------------------

impl Metadata {
    /// Return the key (name) of this metadata item.
    pub fn key(&self) -> String {
        match self {
            Metadata::Author(_) => "author",
            Metadata::Copyright(_) => "copyright",
            Metadata::Date(_) => "date",
            Metadata::Keywords(_) => "keywords",
            Metadata::Revision(_) => "revision",
            Metadata::Status(_) => "status",
            Metadata::Title(_) => "title",
            Metadata::Other(p) => &p.key,
        }
        .to_string()
    }

    /// Return a single string representation of the metadata item's value.
    pub fn value_string(&self) -> String {
        match self {
            Metadata::Author(value) => format!(
                "{}{}{}",
                value.name,
                value
                    .email
                    .as_ref()
                    .map(|s| format!("({})", s))
                    .unwrap_or_default(),
                value
                    .organization
                    .as_ref()
                    .map(|s| format!(" - {}", s))
                    .unwrap_or_default()
            ),
            Metadata::Copyright(value) => format!(
                "{}{}{}",
                value.year,
                value
                    .organization
                    .as_ref()
                    .map(|s| format!(" {}.", s))
                    .unwrap_or_default(),
                value
                    .comment
                    .as_ref()
                    .map(|s| format!(" - {}.", s))
                    .unwrap_or_default()
            ),
            Metadata::Date(value) => value.to_string(),
            Metadata::Keywords(value) => format!("[{}]", value.join(", ")),
            Metadata::Revision(value) => value.to_string(),
            Metadata::Status(value) => value.to_string(),
            Metadata::Title(value) => value.to_string(),
            Metadata::Other(value) => value.value.to_string(),
        }
    }

    /// Return a YAML formatted string representation of the metadata item's value.
    pub fn yaml_string(&self) -> String {
        match self {
            Metadata::Author(value) => format!(
                "author:\n- name: {}{}{}",
                value.name,
                value
                    .email
                    .as_ref()
                    .map(|s| format!("- email: {}\n", s))
                    .unwrap_or_default(),
                value
                    .organization
                    .as_ref()
                    .map(|s| format!("- organization: {}", s))
                    .unwrap_or_default()
            ),
            Metadata::Copyright(value) => format!(
                "copyright:\n- year: {}{}{}",
                value.year,
                value
                    .organization
                    .as_ref()
                    .map(|s| format!("- organization: {}\n", s))
                    .unwrap_or_default(),
                value
                    .comment
                    .as_ref()
                    .map(|s| format!("- comment: {}", s))
                    .unwrap_or_default()
            ),
            Metadata::Date(value) => self.yaml_one(value),
            Metadata::Keywords(value) => self.yaml_one(&format!("[{}]", value.join(", "))),
            Metadata::Revision(value) => value.to_string(),
            Metadata::Status(value) => self.yaml_one(value),
            Metadata::Title(value) => self.yaml_one(value),
            Metadata::Other(value) => self.yaml_one(&value.value),
        }
    }

    fn yaml_one(&self, value: &str) -> String {
        format!("{}: {}", self.key(), value.to_string())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
