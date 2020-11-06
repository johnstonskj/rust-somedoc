#[macro_use]
extern crate somedoc;

use crate::somedoc::model::blocks::HasBlockContent;
use crate::somedoc::model::inline::HasInlineContent;
use somedoc::model::blocks::{Cell, Column, Heading, Paragraph, Quote, Row, Table};
use somedoc::model::document::Document;
use somedoc::model::inline::{Anchor, HyperLink, Text};
use somedoc::write::markdown::MarkdownFlavor;
use somedoc::write::{write_document_to_string, OutputFormat};

fn main() {
    let mut doc = Document::default();

    doc.add_heading(Heading::title(
        "Scheme: Clothing shapes, patterns, and details",
    ));
    doc.add_paragraph(Paragraph::italic_text("Terms commonly used to describe fashion items. It includes terms for outline, fit, elements, detailing, and patterns."));

    doc.add_paragraph(Paragraph::link(HyperLink::external(
        "http://amazon.com/vocabulary/fashion-design#DesignScheme",
    )));

    doc.add_heading(Heading::heading_2("Labels"));

    let mut labels = Quote::default();

    labels.add_paragraph(Paragraph::bold_text("skos:prefLabel"));

    let mut table = Table::new(&[Column::from("Label text"), Column::from("Language")]);
    table.add_row(Row::new(&[
        Cell::text_str("Clothing shapes, patterns, and details"),
        Cell::text(textbf!("en")),
    ]));
    labels.add_table(table);
    doc.add_block_quote(labels);

    doc.add_heading(Heading::heading_2("Other Properties"));

    doc.add_thematic_break();

    let mut links = Paragraph::default();
    links.add_text_str("Jump to: ");
    links.add_link(HyperLink::internal_with_label(
        Anchor::new("concepts-hierarchy").unwrap(),
        "Concepts Hierarchy",
    ));
    links.add_text_str(" | ");
    links.add_link(HyperLink::internal_with_label(
        Anchor::new("concepts").unwrap(),
        "Concepts",
    ));
    links.add_text_str(" | ");
    links.add_link(HyperLink::internal_with_label(
        Anchor::new("collections").unwrap(),
        "Collections",
    ));
    links.add_text_str(" | ");
    links.add_link(HyperLink::internal_with_label(
        Anchor::new("appendix-rdf").unwrap(),
        "Appendix - RDF",
    ));
    doc.add_paragraph(links);

    doc.add_thematic_break();

    doc.add_heading(Heading::heading_2("Concept Hierarchy"));

    let md = write_document_to_string(&doc, MarkdownFlavor::default().into()).unwrap();
    println!("{}", md);

    let xw = write_document_to_string(&doc, OutputFormat::XWiki).unwrap();
    println!("{}", xw);
}
