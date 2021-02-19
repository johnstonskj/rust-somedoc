use somedoc::model::Document;
use somedoc::write::markdown::MarkdownFlavor;

pub mod common;

#[inline]
fn assert_markdown_eq(part_fn: impl Fn() -> Document, expected: &str) {
    common::assert_serialized_eq(&part_fn(), MarkdownFlavor::XWiki.into(), expected)
}

#[test]
fn test_skos() {
    assert_markdown_eq(
        common::skos::document,
        r###"{{comment}}
title: Scheme: Clothing shapes, patterns, and details
author:
- name: Simon
{{/comment}}


= Scheme: Clothing shapes, patterns, and details =

{{comment}}
TODO:
- more nested lists
- tables
{{/comment}}


//Terms commonly used to describe fashion items. It includes terms for outline, fit, elements, detailing, and patterns.//

[[http://amazon.com/vocabulary/fashion-design#DesignScheme]]

(% id="Labels" %)
== Labels ==


> **skos:prefLabel**
> 
> **skos:altLabel**
> 

|=Label text|=Language
|Clothing shapes, patterns, and details|**en**


(% id="Other_Properties" %)
== Other Properties ==

-----

Jump to: [[Concepts Hierarchy>>||anchor=Concepts_Hierarchy]] | [[Concepts>>||anchor=Concepts]] | [[Collections>>||anchor=Collections]] | [[Appendix - RDF>>||anchor=Appendix_-_RDF]]

-----

(% id="Concept_Hierarchy" %)
== Concept Hierarchy ==

* **First item**
* Second item
** //Third item//
* First item


(% id="Appendix_-_RDF" %)
== Appendix - RDF ==

{{code language="turtle"}}
@prefix foo: <...>
foo:bar foo:baz 12.
{{/code}}

{{{
@prefix foo: <...>
foo:bar foo:baz 12.
}}}
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
        r###"{{comment}}
title: Test Document
{{/comment}}

"###,
    );
}

#[test]
fn test_document_with_heading() {
    assert_markdown_eq(
        common::parts::document_with_heading,
        r###"
= Test Document =
"###,
    );
}

#[test]
fn test_document_with_labeled_heading() {
    assert_markdown_eq(
        common::parts::document_with_labeled_heading,
        r###"
(% id="Test_Document" %)
= Test Document =
"###,
    );
}

#[test]
fn test_document_with_headings() {
    assert_markdown_eq(
        common::parts::document_with_headings,
        r###"
= Section =

== Sub-section ==

=== Sub-sub-section ===

==== Sub-sub-sub-section ====

===== Sub-sub-sub-sub-section =====

====== Sub-sub-sub-sub-sub-section ======

======= Sub-sub-sub-sub-sub-sub-section =======
"###,
    );
}

#[test]
fn test_document_with_front_matter() {
    assert_markdown_eq(
        common::parts::document_with_front_matter,
        r###"
{{toc/}}










= Section One =

= Section Two =
"###,
    );
}

#[test]
fn test_paragraph_alignment() {
    assert_markdown_eq(
        common::parts::paragraph_alignment,
        r###"
left-aligned

right-aligned

center-aligned

both-aligned
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
fn test_labeled_ordered_list() {
    assert_markdown_eq(
        common::parts::labeled_ordered_list,
        r###"
(% id="lst1" %)
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
11. inner one
11. inner two
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
** inner one
** inner two
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
*1. inner one
*1* inner inner one
*1. inner two
* three

"###,
    );
}

#[test]
fn test_definition_list() {
    assert_markdown_eq(
        common::parts::definition_list,
        r###"
; Universe
: Big, really big
"###,
    );
}

#[test]
fn test_image_block() {
    assert_markdown_eq(
        common::parts::image_block,
        r###"

image:https://example.org/example.png
"###,
    );
}

#[test]
fn test_image_block_with_label_and_caption() {
    assert_markdown_eq(
        common::parts::image_block_with_label_and_caption,
        r###"

(% id="img:example" %)
image:https://example.org/example.png
"###,
    );
}

#[test]
fn test_math_block() {
    assert_markdown_eq(
        common::parts::math_block,
        r###"

{{formula}}x=2+2^2{{/formula}}
"###,
    );
}

#[test]
fn test_math_block_with_label_and_caption() {
    assert_markdown_eq(
        common::parts::math_block_with_label_and_caption,
        r###"

(% id="math:example" %)
{{formula}}x=2+2^2{{/formula}}
"###,
    );
}

#[test]
fn test_block_quote() {
    assert_markdown_eq(
        common::parts::block_quote,
        r###"

> a block quote
> 
"###,
    );
}

#[test]
fn test_nested_block_quotes() {
    assert_markdown_eq(
        common::parts::nested_block_quotes,
        r###"

> a block quote
> 
> 
>> another block quote
>> 
> 
"###,
    );
}

#[test]
fn test_text_styles() {
    assert_markdown_eq(
        common::parts::text_styles,
        r###"
Here is some plain **bold** //italic// ##mono## ##code## plain --strikethrough-- __underline__ small caps ^^superscript^^ ,,subscript,, text.
"###,
    );
}

#[test]
fn test_nested_text_styles() {
    assert_markdown_eq(
        common::parts::nested_text_styles,
        r###"
Here is some **//bold italic//** text.

Here is some bold italic plain text.

Here is some //bold plain italic// text.
"###,
    );
}

#[test]
fn test_hyper_links() {
    assert_markdown_eq(
        common::parts::hyper_links,
        r###"
[[https://example.org/]]

[[example>>https://example.org/]]

[[||anchor=section-2]]

[[example>>||anchor=section-2]]
"###,
    );
}

#[test]
fn test_complex_paragraph() {
    assert_markdown_eq(
        common::parts::complex_paragraph,
        r###"
This paragraph has [[a link>>https://example.org/]], some math:&nbsp;{{formula}}x=2+2^2{{/formula}}, a line break,\\an image:&nbsp;image:https://example.org/favicon.png&nbsp;---&nbsp; all together!
"###,
    );
}
