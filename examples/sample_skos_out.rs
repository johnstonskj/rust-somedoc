use somedoc::model::block::{
    Cell, CodeBlock, Column, Formatted, HasBlockContent, Heading, List, Paragraph, Quote, Row,
    Table,
};
use somedoc::model::document::Document;
use somedoc::model::inline::{Anchor, HasInlineContent, HyperLink, Span, Text};
use somedoc::write::markdown::MarkdownFlavor;
use somedoc::write::{write_document_to_string, OutputFormat};

fn main() {
    let mut doc = Document::default();

    doc.add_heading(Heading::title(
        "Scheme: Clothing shapes, patterns, and details",
    ));
    doc.add_paragraph(Paragraph::italic_str("Terms commonly used to describe fashion items. It includes terms for outline, fit, elements, detailing, and patterns."));

    doc.add_paragraph(Paragraph::link(HyperLink::external(
        "http://amazon.com/vocabulary/fashion-design#DesignScheme",
    )));

    doc.add_heading(Heading::heading_2("Labels"));

    let mut labels = Quote::default();
    labels.add_paragraph(Paragraph::bold_str("skos:prefLabel"));
    doc.add_block_quote(labels);

    let mut table = Table::new(&[Column::from("Label text"), Column::from("Language")]);
    table.add_row(Row::new(&[
        Cell::text_str("Clothing shapes, patterns, and details"),
        Cell::bold_str("en"),
    ]));
    doc.add_table(table);

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

    let mut top_list = List::default();
    top_list.add_item_from(Span::bold_str("First item").into());
    top_list.add_item_from(Text::from("Second item").into());
    let mut inner_list = List::default();
    inner_list.add_item_from(Span::italic_str("Third item").into());
    top_list.add_sub_list(inner_list);
    top_list.add_item_from(Text::from("First item").into());
    doc.add_list(top_list);

    doc.add_heading(Heading::heading_2("Appendix - RDF"));

    doc.add_code_block(CodeBlock::new_with_language(
        "@prefix foo: <...>\nfoo:bar foo:baz 12.",
        "turtle",
    ));

    doc.add_formatted(Formatted::new("@prefix foo: <...>\nfoo:bar foo:baz 12."));

    let md = write_document_to_string(&doc, MarkdownFlavor::default().into()).unwrap();
    println!("{}", md);

    let xw = write_document_to_string(&doc, OutputFormat::XWiki).unwrap();
    println!("{}", xw);
}
