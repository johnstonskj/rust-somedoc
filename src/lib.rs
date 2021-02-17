/*!
* A very simple document model and set of markup wriers.
*
* This crate provides a simple document [`model`](model/index.html) that captures structure of simple documents. This is
* not an all-purpose model, it does not have the richness of full document systems but is intended
* for tools that simply need to generate useful documentation or reports.
*
* The model can then be serialized into specific formats using the formatters in the [`write`](write/index.html)
* module. Currently various flavors of Markdown are implemented as well as the XWiki format.
*
* ## Model
*
* The `somedoc::model` module provides the model to construct documents.
*
* ### Example
*
* ```rust
* use somedoc::model::block::{HasBlockContent, Heading, List, Paragraph};
* use somedoc::model::document::Document;
* use somedoc::model::inline::{HasInlineContent, HyperLink, Image, Span};
*
* fn readme_maker(crate_name: &str, repo_owner: &str, repo_name: &str, headline: &str) -> Document {
*     let tbd = Paragraph::plain_str("TBD");
*     let mut doc = Document::default();
*
*     doc.add_heading(Heading::section(&format!("Crate {}", crate_name)))
*         .add_paragraph(Paragraph::plain_str(headline));
*
*     let mut para = Paragraph::default();
*     para.add_image(Image::with_alt_text(
*         "https://img.shields.io/badge/license-mit-118811.svg",
*         "MIT License",
*     ))
*     .add_image(Image::with_alt_text(
*         "https://img.shields.io/badge/Min%20Rust-1.40-green.svg",
*         "Build",
*     ))
*     .add_image(Image::with_alt_text(
*         &format!(
*             "https://github.com/{}/{}/workflows/Rust/badge.svg",
*             repo_owner, repo_name
*         ),
*         "Minimum Rust Version",
*     ))
*     .add_image(Image::with_alt_text(
*         &format!(
*             "https://github.com/{}/{}/workflows/Security%20audit/badge.svg",
*             repo_owner, repo_name
*         ),
*         "Audit",
*     ));
*
*     doc.add_paragraph(para)
*         .add_thematic_break()
*         .add_paragraph(tbd.clone())
*         .add_heading(Heading::sub_section("Example"))
*         .add_paragraph(tbd.clone())
*         .add_heading(Heading::sub_section("Features"))
*         .add_paragraph(tbd.clone())
*         .add_thematic_break()
*         .add_heading(Heading::sub_section("Changes"))
*         .add_paragraph(Paragraph::bold_str("Version 0.1.0"));
*
*     let mut list = List::default();
*     list.add_item_from(Span::plain_str("Initial release.").into());
*
*     doc.add_list(list)
*         .add_heading(Heading::sub_section("Issues"))
*         .add_paragraph(tbd.clone());
*
*     doc
* }
* ```
*
* ## Writers
*
* The `somedoc::write` module contains a number of serializers that generate specific markup for
* different platforms [`html`](html/index.html), [`json`](json/index.html),
* [`latex`](latex/index.html), and [`markdown`](markdown/index.html).
*
* The JSON module is rather different from the rest, it is intended for tool usage and is a
* direct representation of the `Document` model so that other tools may consume it. To read this
* format the [`read`](read/index.html) module provides for parsing from a string value of from
* a `std::io::Read` implementation.
*
* ### Examples
*
* The following writes a constructed document to `stdout` as a Markdown document. The default
* flavor supported by the writer is the [CommonMark](https://spec.commonmark.org/0.29/) spec.
*
* ```rust
* # use somedoc::model::Document;
* use somedoc::write::write_document_to_string;
* use somedoc::write::markdown::MarkdownFlavor;
*
* # fn make_some_document() -> Document { Document::default() }
* let doc = make_some_document();
*
* let doc_str = write_document_to_string(&doc, MarkdownFlavor::default().into()).unwrap();
* println!("{}", doc_str);
* ```
*
* The following writes the same document out in the
* [XWiki](https://www.xwiki.org/xwiki/bin/view/Documentation/UserGuide/Features/XWikiSyntax/)
* markup format.
*
* ```rust
* # use somedoc::model::Document;
* use somedoc::write::{write_document_to_string, OutputFormat};
* use somedoc::write::markdown::MarkdownFlavor;
*
* # fn make_some_document() -> Document { Document::default() }
* let doc = make_some_document();
*
* let doc_str = write_document_to_string(&doc, MarkdownFlavor::XWiki.into()).unwrap();
* println!("{}", doc_str);
* ```
*
* ```rust
* # use somedoc::model::Document;
* use somedoc::write::{write_document_to_string, OutputFormat};
*
* # fn make_some_document() -> Document { Document::default() }
* let doc = make_some_document();
*
* let doc_str = write_document_to_string(&doc, OutputFormat::Html).unwrap();
* println!("{}", doc_str);
* ```
*
* ## Features
*
* * Formats:
*   * **fmt_html** - HTML writer.
*   * **fmt_latex** - LaTeX (experimental) writer.
*   * **fmt_markdown** - Markdown/wiki writer.
* * **emoji_names**; adds a new module `emoji_names` to `model::inline` which only contains string
*   constants for commonly supported emoji names. These can then be used to construct `Emoji` values
*   for inline characters. This feature is not included by default.
* * **math_builder**; - experimental support for building math expressions. This feature is not
*   included by default.
*/

// ------------------------------------------------------------------------------------------------
// Preamble
// ------------------------------------------------------------------------------------------------

#![warn(
    // ---------- Stylistic
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Public
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    // ---------- Unused
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
)]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate lazy_static;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
#[macro_use]
pub mod macros;

pub mod error;

pub mod model;

#[cfg(feature = "fmt_json")]
pub mod read;

pub mod write;
