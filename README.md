# Crate somedoc

A very simple document model and markup generator.

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.40-green.svg)
[![crates.io](https://img.shields.io/crates/v/somedoc.svg)](https://crates.io/crates/somedoc)
[![docs.rs](https://docs.rs/somedoc/badge.svg)](https://docs.rs/somedoc)
![Build](https://github.com/johnstonskj/rust-somedoc/workflows/Rust/badge.svg)
![Audit](https://github.com/johnstonskj/rust-somedoc/workflows/Security%20audit/badge.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-somedoc.svg)](https://github.com/johnstonskj/rust-somedoc/stargazers)

-----

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

-----

## Changes

**Version 0.2.2**

* Added: mostly complete API/library documentation.
* Added: `inner_impl` macro for consistent use of `inner` and `into_inner` methods.
  * Refactor: this required renaming `link` to `inner` on `Image`.
* Fixed: Duplicate `From` and `from` implementations on inline content.
* Refactor: renamed `label` to `text` on `model::block::table::Column`.
* Refactor: renamed `Captioned` trait to `HasCaption` for consistency.
* Refactor: replaced `Text` with `Caption` as type for `HyperLink` caption; implemented 
  `HasCaption` also.
* Refactor: made definition lists only a single level, and the term is simply a `String`.
* Refactor: removed constructors that only took a single value and use `From<>` instead in `Caption`, `Code`,
  `Formatted`, `Heading`, `Image`.
* Refactor: renaming `new_with_` functions to simply `with_`.
* Refactor: removed `ParagraphStyle`, made `alignment` a single value property on `Paragraph`. 
  * Moved the Alignment enum to its own module shared with `Paragraph` and `table::Column`.
  * Added new `HasAlignment` trait (and macro) for consistency between paragraph and column.
* Clean-up: fixed all Clippy warnings.  

**Version 0.2.1**

This is a significant update, some APIs will have changed, but the plan is that these new
API forms will be stabilized toward a 0.3.0 release that can be relied on for non-breaking
changes.

* Added: additional features in the HTML writer. It should be complete pending testing.
* Added: complete writer implementation for LaTeX. It should be complete pending testing.
* Added: configuration features for each writer, `fmt_html`, `fmt_latex`, fmt_markdown`,
  etc. with the default feature including all of these.
* Added: `Writer`, and `ConfigurableWriter` to the `write` module to capture the
  instantiation of a writer struct.
* Added: more library/API documentation, but not all yet.  
* Refactor: combined all markdown-like formats into a common module.
* Refactor: moved the `Anchor` type to a new `Label`, it is no longer a stand-alone inline
  value, but has been added as a property to most block types.
* Refactor: moved `Abstract` out of the `Metadata` enum and made it a property of the document
  proper. This also affects the visitor API.
* Refactor: renamed `label`/`alt_text` on `HyperLink` to `caption`.
* Clean-up: made the document API easier, removing `Result<>` where it wasn't needed.
* Clean-up: removed the unused dependency on `log`.
* Testing: added dependency on `pretty_assertions` for better comparison of test failures.
* Testing: creating test cases across formats.
  
**Version 0.2.0**

* Added: new visitor traits and migrated the `XWiki` writer to use it.
* Added: new HTML writer using the new visitor traits.
* Added: new `Math` (inline), and `MathBlock` (block) constructs.
* Added: new `Caption` type and implementation for `Code`, `MathBlock`, and `Table`.
* Added: `Deref` for some String newtypes (`Anchor`, `Caption`, `Emoji`, and `Text`).
* Testing: using `proptest` for the String newtypes listed above.

**Version 0.1.7**

* Fixed: fixed a bug in HeadingLevel/markdown generation.

**Version 0.1.6**

* Added: started on library documentation.
* Added: `model::inline::emoji_names` module, but only enabled for `emoji_names` feature.
* Added: `readme_maker` example, this is used in the README and lib.rs documentation.
* Fixed: fixed a bug in XWiki `write_code` function.
* Refactor: document metadata, and Markdown writer accordingly.
* Refactor: renamed `TextStyle` -> `SpanStyle`, added `Sized` variant.
* Clean-up: the `read` module as it was currently empty.
* Clean-up: femoved the `model::visitor` module, too many changes to stabilize yet.
* Testing: some initial test cases.

**Version 0.1.5 (not published)**

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

* Investigate math support in some form.
* Investigate float hints.
* Investigate glossary, acronym, and bib support.