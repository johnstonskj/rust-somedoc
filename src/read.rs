/*!
This module introduces the ability to read the JSON representation used for external tool
integration.

# Example

TBD

*/

use crate::model::Document;
use std::io::Read;

/// Parse a `Document` instance from the JSON provided in the `json` string.
pub fn from_str(json: &str) -> crate::error::Result<Document> {
    let doc: Document = serde_json::from_str(json)?;
    Ok(doc)
}

/// Read from the provided `reader` and parse the JSON into a `Document` instance.
pub fn from_reader(reader: impl Read) -> crate::error::Result<Document> {
    let doc: Document = serde_json::from_reader(reader)?;
    Ok(doc)
}
