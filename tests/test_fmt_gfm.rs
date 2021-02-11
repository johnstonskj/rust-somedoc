use somedoc::model::Document;
use somedoc::write::markdown::MarkdownFlavor;

pub mod common;

#[inline]
fn assert_markdown_eq(part_fn: impl Fn() -> Document, expected: &str) {
    common::assert_serialized_eq(&part_fn(), MarkdownFlavor::GitHub.into(), expected)
}

#[test]
fn test_skos() {
    assert_markdown_eq(
        common::skos::document,
        r###"---
[_metadata_:title]:- "Scheme: Clothing shapes, patterns, and details"
[_metadata_:author]:- "Simon"
---


# Scheme: Clothing shapes, patterns, and details

[//]: # "TODO:"
[//]: # "- more nested lists"
[//]: # "- tables"


*Terms commonly used to describe fashion items. It includes terms for outline, fit, elements, detailing, and patterns.*

[[http://amazon.com/vocabulary/fashion-design#DesignScheme]]

## Labels


> **skos:prefLabel**
> 
> **skos:altLabel**
> 

|Label text|Language|
|-----|-----|
|Clothing shapes, patterns, and details|**en**|


## Other Properties

-----

Jump to: [[Concepts Hierarchy>>.||anchor=HConceptsHierarchy]] | [[Concepts>>.||anchor=HConcepts]] | [[Collections>>.||anchor=HCollections]] | [[Appendix - RDF>>.||anchor=HAppendix-RDF]]

-----

## Concept Hierarchy

* **First item**
* Second item
  * *Third item*
* First item


## Appendix - RDF

```turtle
@prefix foo: <...>
foo:bar foo:baz 12.
```

    @prefix foo: <...>
     foo:bar foo:baz 12.
     
     

"###,
    );
}

#[test]
fn test_empty_document() {
    assert_markdown_eq(common::parts::empty_document, r###""###);
}

#[test]
fn test_document_with_title() {
    assert_markdown_eq(
        common::parts::document_with_title,
        r###"---
[_metadata_:title]:- "Test Document"
---

"###,
    );
}

#[test]
fn test_document_with_heading() {
    assert_markdown_eq(
        common::parts::document_with_heading,
        r###"
# Test Document
"###,
    );
}

#[test]
fn test_document_with_headings() {
    assert_markdown_eq(
        common::parts::document_with_headings,
        r###"
# Section

## Sub-section

### Sub-sub-section

#### Sub-sub-sub-section

##### Sub-sub-sub-sub-section

###### Sub-sub-sub-sub-sub-section

####### Sub-sub-sub-sub-sub-sub-section
"###,
    );
}

#[test]
fn test_ordered_list() {
    assert_markdown_eq(
        common::parts::ordered_list,
        r###"
1. one
1. two
1. three

"###,
    );
}

#[test]
fn test_unordered_list() {
    assert_markdown_eq(
        common::parts::unordered_list,
        r###"
* one
* two
* three

"###,
    );
}

#[test]
fn test_nested_ordered_list() {
    assert_markdown_eq(
        common::parts::nested_ordered_list,
        r###"
1. one
1. two
   1. inner one
   1. inner two
1. three

"###,
    );
}

#[test]
fn test_nested_unordered_list() {
    assert_markdown_eq(
        common::parts::nested_unordered_list,
        r###"
* one
* two
  * inner one
  * inner two
* three

"###,
    );
}

#[test]
fn test_nested_mixed_lists() {
    assert_markdown_eq(
        common::parts::nested_mixed_lists,
        r###"
* one
* two
  1. inner one
     * inner inner one
  1. inner two
* three

"###,
    );
}

#[test]
fn test_definition_list() {
    assert_markdown_eq(
        common::parts::definition_list,
        r###"
**Universe**:- Big, really big
"###,
    );
}
