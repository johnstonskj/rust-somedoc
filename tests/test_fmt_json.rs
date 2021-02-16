use somedoc::model::Document;
use somedoc::write::OutputFormat;

pub mod common;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[inline]
fn assert_json_eq(part_fn: impl Fn() -> Document, expected: &str) {
    let expected = format!("{{\"version\":\"{}\",{}", VERSION, expected);
    common::assert_serialized_eq(&part_fn(), OutputFormat::Json, &expected)
}

#[test]
fn test_skos() {
    assert_json_eq(
        common::skos::document,
        r###""metadata":[{"Title":"Scheme: Clothing shapes, patterns, and details"},{"Author":{"name":"Simon"}}],"content":[{"Heading":{"level":"Section","inner":[{"Text":"Scheme: Clothing shapes, patterns, and details"}]}},{"Comment":"TODO:\n- more nested lists\n- tables"},{"Paragraph":{"inner":[{"Span":{"inner":[{"Text":"Terms commonly used to describe fashion items. It includes terms for outline, fit, elements, detailing, and patterns."}],"styles":["Italic"]}}],"alignment":"Left"}},{"Paragraph":{"inner":[{"HyperLink":{"target":{"External":"http://amazon.com/vocabulary/fashion-design#DesignScheme"}}}],"alignment":"Left"}},{"Heading":{"label":"Labels","level":"SubSection","inner":[{"Text":"Labels"}]}},{"Quote":{"content":[{"Paragraph":{"inner":[{"Span":{"inner":[{"Text":"skos:prefLabel"}],"styles":["Bold"]}}],"alignment":"Left"}},{"Paragraph":{"inner":[{"Span":{"inner":[{"Text":"skos:altLabel"}],"styles":["Bold"]}}],"alignment":"Left"}}]}},{"Table":{"columns":[{"text":"Label text","alignment":"Left"},{"text":"Language","alignment":"Left"}],"rows":[{"cells":[{"inner":[{"Text":"Clothing shapes, patterns, and details"}]},{"inner":[{"Span":{"inner":[{"Text":"en"}],"styles":["Bold"]}}]}]}],"caption":"Other labels"}},{"Heading":{"label":"Other_Properties","level":"SubSection","inner":[{"Text":"Other Properties"}]}},"ThematicBreak",{"Paragraph":{"inner":[{"Text":"Jump to: "},{"HyperLink":{"target":{"Internal":"Concepts_Hierarchy"},"caption":"Concepts Hierarchy"}},{"Text":" | "},{"HyperLink":{"target":{"Internal":"Concepts"},"caption":"Concepts"}},{"Text":" | "},{"HyperLink":{"target":{"Internal":"Collections"},"caption":"Collections"}},{"Text":" | "},{"HyperLink":{"target":{"Internal":"Appendix_-_RDF"},"caption":"Appendix - RDF"}}],"alignment":"Left"}},"ThematicBreak",{"Heading":{"label":"Concept_Hierarchy","level":"SubSection","inner":[{"Text":"Concept Hierarchy"}]}},{"List":{"kind":"Unordered","inner":[{"Item":{"inner":[{"Span":{"inner":[{"Span":{"inner":[{"Text":"First item"}],"styles":["Bold"]}}]}}]}},{"Item":{"inner":[{"Text":"Second item"}]}},{"List":{"kind":"Unordered","inner":[{"Item":{"inner":[{"Span":{"inner":[{"Span":{"inner":[{"Text":"Third item"}],"styles":["Italic"]}}]}}]}}]}},{"Item":{"inner":[{"Text":"First item"}]}}]}},{"Heading":{"label":"Appendix_-_RDF","level":"SubSection","inner":[{"Text":"Appendix - RDF"}]}},{"CodeBlock":{"code":"@prefix foo: <...>\nfoo:bar foo:baz 12.","language":"turtle"}},{"Formatted":{"inner":"@prefix foo: <...>\nfoo:bar foo:baz 12."}}]}"###,
    );
}

#[test]
fn test_empty_document() {
    assert_json_eq(common::parts::empty_document, r###""content":[]}"###);
}

#[test]
fn test_document_with_title() {
    assert_json_eq(
        common::parts::document_with_title,
        r###""metadata":[{"Title":"Test Document"}],"content":[]}"###,
    );
}

#[test]
fn test_document_with_heading() {
    assert_json_eq(
        common::parts::document_with_heading,
        r###""content":[{"Heading":{"level":"Section","inner":[{"Text":"Test Document"}]}}]}"###,
    );
}

#[test]
fn test_document_with_headings() {
    assert_json_eq(
        common::parts::document_with_headings,
        r###""content":[{"Heading":{"level":"Section","inner":[{"Text":"Section"}]}},{"Heading":{"level":"SubSection","inner":[{"Text":"Sub-section"}]}},{"Heading":{"level":"SubSubSection","inner":[{"Text":"Sub-sub-section"}]}},{"Heading":{"level":"SubSubSubSection","inner":[{"Text":"Sub-sub-sub-section"}]}},{"Heading":{"level":"SubSubSubSubSection","inner":[{"Text":"Sub-sub-sub-sub-section"}]}},{"Heading":{"level":"SubSubSubSubSubSection","inner":[{"Text":"Sub-sub-sub-sub-sub-section"}]}},{"Heading":{"level":"SubSubSubSubSubSubSection","inner":[{"Text":"Sub-sub-sub-sub-sub-sub-section"}]}}]}"###,
    );
}

#[test]
fn test_unordered_list() {
    assert_json_eq(
        common::parts::unordered_list,
        r###""content":[{"List":{"kind":"Unordered","inner":[{"Item":{"inner":[{"Span":{"inner":[{"Text":"one"}],"styles":["Plain"]}}]}},{"Item":{"inner":[{"Span":{"inner":[{"Text":"two"}],"styles":["Plain"]}}]}},{"Item":{"inner":[{"Span":{"inner":[{"Text":"three"}],"styles":["Plain"]}}]}}]}}]}"###,
    );
}

#[test]
fn test_ordered_list() {
    assert_json_eq(
        common::parts::ordered_list,
        r###""content":[{"List":{"kind":"Ordered","inner":[{"Item":{"inner":[{"Span":{"inner":[{"Text":"one"}],"styles":["Plain"]}}]}},{"Item":{"inner":[{"Span":{"inner":[{"Text":"two"}],"styles":["Plain"]}}]}},{"Item":{"inner":[{"Span":{"inner":[{"Text":"three"}],"styles":["Plain"]}}]}}]}}]}"###,
    );
}

#[test]
fn test_nested_unordered_list() {
    assert_json_eq(
        common::parts::nested_unordered_list,
        r###""content":[{"List":{"kind":"Unordered","inner":[{"Item":{"inner":[{"Span":{"inner":[{"Text":"one"}],"styles":["Plain"]}}]}},{"Item":{"inner":[{"Span":{"inner":[{"Text":"two"}],"styles":["Plain"]}}]}},{"List":{"kind":"Unordered","inner":[{"Item":{"inner":[{"Span":{"inner":[{"Text":"inner one"}],"styles":["Plain"]}}]}},{"Item":{"inner":[{"Span":{"inner":[{"Text":"inner two"}],"styles":["Plain"]}}]}}]}},{"Item":{"inner":[{"Span":{"inner":[{"Text":"three"}],"styles":["Plain"]}}]}}]}}]}"###,
    );
}

#[test]
fn test_nested_ordered_list() {
    assert_json_eq(
        common::parts::nested_ordered_list,
        r###""content":[{"List":{"kind":"Ordered","inner":[{"Item":{"inner":[{"Span":{"inner":[{"Text":"one"}],"styles":["Plain"]}}]}},{"Item":{"inner":[{"Span":{"inner":[{"Text":"two"}],"styles":["Plain"]}}]}},{"List":{"kind":"Ordered","inner":[{"Item":{"inner":[{"Span":{"inner":[{"Text":"inner one"}],"styles":["Plain"]}}]}},{"Item":{"inner":[{"Span":{"inner":[{"Text":"inner two"}],"styles":["Plain"]}}]}}]}},{"Item":{"inner":[{"Span":{"inner":[{"Text":"three"}],"styles":["Plain"]}}]}}]}}]}"###,
    );
}

#[test]
fn test_nested_mixed_lists() {
    assert_json_eq(
        common::parts::nested_mixed_lists,
        r###""content":[{"List":{"kind":"Unordered","inner":[{"Item":{"inner":[{"Span":{"inner":[{"Text":"one"}],"styles":["Plain"]}}]}},{"Item":{"inner":[{"Span":{"inner":[{"Text":"two"}],"styles":["Plain"]}}]}},{"List":{"kind":"Ordered","inner":[{"Item":{"inner":[{"Span":{"inner":[{"Text":"inner one"}],"styles":["Plain"]}}]}},{"List":{"kind":"Unordered","inner":[{"Item":{"inner":[{"Span":{"inner":[{"Text":"inner inner one"}],"styles":["Plain"]}}]}}]}},{"Item":{"inner":[{"Span":{"inner":[{"Text":"inner two"}],"styles":["Plain"]}}]}}]}},{"Item":{"inner":[{"Span":{"inner":[{"Text":"three"}],"styles":["Plain"]}}]}}]}}]}"###,
    );
}

#[test]
fn test_definition_list() {
    assert_json_eq(
        common::parts::definition_list,
        r###""content":[{"DefinitionList":{"inner":[{"term":"Universe","text":{"inner":[{"Text":"Big, really big"}]}}]}}]}"###,
    );
}
