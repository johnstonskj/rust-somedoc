/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error;
use crate::model::Document;
use crate::write::markdown::MarkdownFlavor;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum OutputFormat {
    Markdown(MarkdownFlavor),
    XWiki,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn write_document<W: Write>(
    doc: &Document,
    format: OutputFormat,
    w: &mut W,
) -> std::io::Result<()> {
    match format {
        OutputFormat::Markdown(flavor) => markdown::writer::<MarkdownFlavor, W>(doc, flavor, w),
        OutputFormat::XWiki => xwiki::writer(doc, w),
    }
}

///
/// A convenience function that will return a String containing the output of the `DocWriter`
/// for the given `Document` instance.
///
pub fn write_document_to_string(doc: &Document, format: OutputFormat) -> std::io::Result<String> {
    use std::io::Cursor;
    let mut buffer = Cursor::new(Vec::new());
    write_document(doc, format, &mut buffer)?;
    Ok(String::from_utf8(buffer.into_inner()).unwrap())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Markdown(Default::default())
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Markdown(f) => format!("markdown+{}", f),
                OutputFormat::XWiki => "xwiki".to_string(),
            }
        )
    }
}

impl FromStr for OutputFormat {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "md" | "markdown" => Ok(Self::Markdown(Default::default())),
            "xwiki" => Ok(Self::XWiki),
            _ => Err(error::ErrorKind::UnknownFormat.into()),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod markdown;

pub mod xwiki;
