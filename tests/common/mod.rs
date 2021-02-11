use pretty_assertions::assert_eq;

use somedoc::model::Document;
use somedoc::write::{write_document_to_string, OutputFormat};

pub fn assert_serialized_eq(document: &Document, format: OutputFormat, expected: &str) {
    let result = write_document_to_string(&document, format);

    assert!(result.is_ok());

    let result = result.unwrap();
    println!("{}", result);

    assert_eq!(result, expected.to_string());
}

pub mod skos;

pub mod parts;
