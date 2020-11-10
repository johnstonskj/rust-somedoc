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
*     para.add_image(Image::new(HyperLink::external_with_label_str(
*         "https://img.shields.io/badge/license-mit-118811.svg",
*         "MIT License",
*     )))
*     .add_image(Image::new(HyperLink::external_with_label_str(
*         "https://img.shields.io/badge/Min%20Rust-1.40-green.svg",
*         "Build",
*     )))
*     .add_image(Image::new(HyperLink::external_with_label_str(
*         &format!(
*             "https://github.com/{}/{}/workflows/Rust/badge.svg",
*             repo_owner, repo_name
*         ),
*         "Minimum Rust Version",
*     )))
*     .add_image(Image::new(HyperLink::external_with_label_str(
*         &format!(
*             "https://github.com/{}/{}/workflows/Security%20audit/badge.svg",
*             repo_owner, repo_name
*         ),
*         "Audit",
*     )));
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
* The `somedoc::write` module contains a number of serializers that generate specific markup for different platforms.
*
* ### Example
*
* The following writes a constructed document to `stdout` as a Markdown document. The default flavor supported by
* the writer is the [CommonMark](https://spec.commonmark.org/0.29/) spec.
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
*
* # fn make_some_document() -> Document { Document::default() }
* let doc = make_some_document();
*
* let doc_str = write_document_to_string(&doc, OutputFormat::XWiki).unwrap();
* println!("{}", doc_str);
* ```
*
*/

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
#[macro_use]
pub mod macros;

pub mod error;

pub mod model;

//pub mod read;

pub mod write;
