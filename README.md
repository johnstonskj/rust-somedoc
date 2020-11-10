# Crate somedoc

A very simple document model and markup generator.

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.40-green.svg)
[![crates.io](https://img.shields.io/crates/v/somedoc.svg)](https://crates.io/crates/somedoc)
[![docs.rs](https://docs.rs/somedoc/badge.svg)](https://docs.rs/somedoc)
![Build](https://github.com/johnstonskj/rust-somedoc/workflows/Rust/badge.svg)
![Audit](https://github.com/johnstonskj/rust-somedoc/workflows/Security%20audit/badge.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-somedoc.svg)](https://github.com/johnstonskj/rust-somedoc/stargazers)

==========

## Model

The `somedoc::model` module provides the model to construct documents.

### Example

```rust
fn readme_maker(crate_name: &str, repo_owner: &str, repo_name: &str, headline: &str) -> Document {
    let tbd = Paragraph::plain_str("TBD");
    let mut doc = Document::default();

    doc.add_heading(Heading::heading_1(&format!("Crate {}", crate_name)))
        .add_paragraph(Paragraph::plain_str(headline));

    let mut para = Paragraph::default();
    para.add_image(Image::new(HyperLink::external_with_label_str(
        "https://img.shields.io/badge/license-mit-118811.svg",
        "MIT License",
    )))
    .add_image(Image::new(HyperLink::external_with_label_str(
        "https://img.shields.io/badge/Min%20Rust-1.40-green.svg",
        "Build",
    )))
    .add_image(Image::new(HyperLink::external_with_label_str(
        &format!(
            "https://github.com/{}/{}/workflows/Rust/badge.svg",
            repo_owner, repo_name
        ),
        "Minimum Rust Version",
    )))
    .add_image(Image::new(HyperLink::external_with_label_str(
        &format!(
            "https://github.com/{}/{}/workflows/Security%20audit/badge.svg",
            repo_owner, repo_name
        ),
        "Audit",
    )));

    doc.add_paragraph(para)
        .add_thematic_break()
        .add_paragraph(tbd.clone())
        .add_heading(Heading::heading_2("Example"))
        .add_paragraph(tbd.clone())
        .add_heading(Heading::heading_2("Features"))
        .add_paragraph(tbd.clone())
        .add_thematic_break()
        .add_heading(Heading::heading_2("Changes"))
        .add_paragraph(Paragraph::bold_str("Version 0.1.0"));

    let mut list = List::default();
    list.add_item_from(Span::plain_str("Initial release.").into());

    doc.add_list(list)
        .add_heading(Heading::heading_2("Issues"))
        .add_paragraph(tbd.clone());

    doc
}

```

## Writers

The `somedoc::write` module contains a number of serializers that generate specific markup for different platforms.

### Example

The following writes a constructed document to `stdout` as a Markdown document. The default flavor supported by 
the writer is the CommonMark spec.

```rust
use somedoc::write::write_document_to_string;
use somedoc::write::markdown::MarkdownFlavor;

let doc = make_some_document();

let doc_str = write_document_to_string(&doc, MarkdownFlavor::default().into()).unwrap();
println!("{}", doc_str);
```

The following writes the same document out in the XWiki markup form.

```rust
use somedoc::write::{write_document_to_string, OutputFormat};

let doc = make_some_document();

let doc_str = write_document_to_string(&doc, OutputFormat::XWiki).unwrap();
println!("{}", doc_str);
```

==========

## Changes

**Version 0.1.6**

* Started on library documentation.
* Refactored document metadata, and Markdown writer accordingly.
* Added `readme_maker` example, this is used in the README and lib.rs documentation.
* Renamed `TextStyle` -> `SpanStyle`, added `Sized` variant.
* Removed the `read` module as it was currently empty.
* Removed the `model::visitor` module, too many changes to stabilize yet.
* Fixed bug in XWiki `write_code` function.
* Added `model::inline::emoji_names` module, but only enabled for `emoji_names` feature.
* Adding test cases.

**Version 0.1.5**

* Added some more replacement rules for XWiki anchors. 

**Version 0.1.4**

* Moved the rules for formatting an `Anchor` into each writer. 

**Version 0.1.3**

* Added `Formatted` alongside `CodeBlock`.
* Removed some additional blank lines from both Markdown and XWiki writers.

**Version 0.1.2**

* Fixed nested list bug in XWiki writer.

**Version 0.1.1**

* Added [cargo-husky](https://github.com/rhysd/cargo-husky) for git hooks.
* Fixed bug in tables for XWiki writer.

**Version 0.1.0**

* Initial commit. Basic model working, initial Markdown and XWiki writers.

## TODO