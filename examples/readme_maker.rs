use somedoc::model::block::{HasBlockContent, Heading, List, Paragraph};
use somedoc::model::document::Document;
use somedoc::model::inline::{HasInlineContent, Image, Span};
use somedoc::write::markdown::MarkdownFlavor;
use somedoc::write::write_document_to_string;

fn main() {
    let doc = readme_maker(
        "somedoc",
        "johnstonskj",
        "rust-somedoc",
        "A very simple document model and markup generator.",
    );

    let md = write_document_to_string(&doc, MarkdownFlavor::default().into()).unwrap();
    println!("{}", md);
}

fn readme_maker(crate_name: &str, repo_owner: &str, repo_name: &str, headline: &str) -> Document {
    let tbd = Paragraph::plain_str("TBD");
    let mut doc = Document::default();

    doc.add_heading(Heading::section(&format!("Crate {}", crate_name)))
        .add_paragraph(Paragraph::plain_str(headline));

    let mut para = Paragraph::default();
    para.add_image(Image::with_alt_text(
        "https://img.shields.io/badge/license-mit-118811.svg",
        "MIT License",
    ))
    .add_image(Image::with_alt_text(
        "https://img.shields.io/badge/Min%20Rust-1.40-green.svg",
        "Build",
    ))
    .add_image(Image::with_alt_text(
        &format!(
            "https://github.com/{}/{}/workflows/Rust/badge.svg",
            repo_owner, repo_name
        ),
        "Minimum Rust Version",
    ))
    .add_image(Image::with_alt_text(
        &format!(
            "https://github.com/{}/{}/workflows/Security%20audit/badge.svg",
            repo_owner, repo_name
        ),
        "Audit",
    ));

    doc.add_paragraph(para)
        .add_thematic_break()
        .add_paragraph(tbd.clone())
        .add_heading(Heading::sub_section("Example"))
        .add_paragraph(tbd.clone())
        .add_heading(Heading::sub_section("Features"))
        .add_paragraph(tbd.clone())
        .add_thematic_break()
        .add_heading(Heading::sub_section("Changes"))
        .add_paragraph(Paragraph::bold_str("Version 0.1.0"));

    let mut list = List::default();
    list.add_item_from(Span::plain_str("Initial release.").into());

    doc.add_list(list)
        .add_heading(Heading::sub_section("Issues"))
        .add_paragraph(tbd.clone());

    doc
}
